/*
 * Databind - Expand the functionality of Minecraft Datapacks.
 * Copyright (C) 2021  Adam Thompson-Sharpe
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use super::Compiler;

use crate::token::Token;
use std::collections::HashMap;

/// Return from the compiler
///
/// # Arguments
///
/// - `file_contents` - A list of file contents
/// - `filename_map` - A map of filenames to indexes in the file_contents Vec
/// - `tag_map` - A map of tags to functions
pub struct CompileReturn {
    pub file_contents: Vec<String>,
    pub filename_map: HashMap<String, usize>,
    pub tag_map: HashMap<String, Vec<String>>,
}

impl Compiler {
    /// Convert tokens to a compiled file or files
    ///
    /// # Arguments
    ///
    /// - `tokens` - A list of tokens
    /// - `namespace` - The namespace to use for functions, if relevant
    pub fn compile(
        &self,
        tokens: Vec<Token>,
        namespace: Option<&str>,
        subfolder: &str,
    ) -> CompileReturn {
        let tokens = self.parse_shorthand(tokens, subfolder);

        let mut tag_map: HashMap<String, Vec<String>> = HashMap::new();

        // A vector of file contents
        let mut files: Vec<String> = vec![String::new()];
        // A map of filenames to indexes in the files vector
        let mut filename_to_index: HashMap<String, usize> = HashMap::new();

        let mut active_token = Token::None;

        // For variable-related tokens
        let mut current_var = String::new();
        let mut assignment_operator = Token::None;
        // For functions
        let mut func_depth = 0;
        let mut current_functions: Vec<String> = Vec::new();
        let mut calling_function = false;
        // For objective-related tokens
        let mut current_objective = String::new();
        let mut objective_target = String::new();

        macro_rules! current_file {
            () => {
                files[filename_to_index[&current_functions[func_depth - 1]]];
            };
        }

        for token in tokens.iter() {
            // Don't compile contents outside of a function
            match token {
                Token::DefineFunc => {}
                _ => {
                    if func_depth == 0 {
                        continue;
                    }
                }
            }

            match token {
                Token::Var => active_token = Token::Var,
                Token::TestVar => active_token = Token::TestVar,
                Token::Objective => active_token = Token::Objective,
                Token::SetObjective => active_token = Token::SetObjective,
                Token::Target(target) => objective_target = target.clone(),
                Token::ModVarName(var) => current_var = var.clone(),
                Token::DelVarName(var) => {
                    let to_add = format!("scoreboard objectives remove {}", var);
                    current_file!().push_str(&to_add[..]);
                }
                Token::TestVarName(var) | Token::OpVarName(var) => {
                    current_var = var.clone();
                    let to_front = if let Token::TestVarName(_) = token {
                        "score "
                    } else {
                        ""
                    };

                    let to_add = format!("{}--databind {} ", to_front, var);
                    current_file!().push_str(&to_add[..]);
                }
                Token::InitialSet => assignment_operator = Token::InitialSet,
                Token::VarSet => assignment_operator = Token::VarSet,
                Token::VarAdd => assignment_operator = Token::VarAdd,
                Token::VarSub => assignment_operator = Token::VarSub,
                Token::FuncName(name) => {
                    // Function definition
                    if !calling_function {
                        files.push("# Compiled with MysteryBlokHed/databind".into());
                        filename_to_index.insert(name.clone(), files.len() - 1);
                        current_functions.push(name.clone());
                    // Function call
                    } else {
                        // Function contains namespace
                        let has_namespace = name.contains(':');

                        if has_namespace {
                            let to_add = format!("function {}", name);

                            current_file!().push_str(&to_add[..]);
                        } else if let Some(ns) = namespace {
                            let to_add = format!("function {}:{}", ns, name);

                            current_file!().push_str(&to_add[..]);
                        } else {
                            panic!("No namespace provided for function call.");
                        }

                        calling_function = false;
                    }
                }
                Token::CallFunc => calling_function = true,
                Token::DefineFunc => func_depth += 1,
                Token::EndFunc => {
                    func_depth -= 1;
                    current_functions.pop();
                }
                Token::TagName(tag) => {
                    if func_depth == 0 {
                        println!("Tag found outside of function.");
                        std::process::exit(1);
                    }

                    tag_map
                        .entry(tag.clone())
                        .or_insert_with(Vec::new)
                        .push(current_functions[func_depth - 1].clone());
                }
                Token::ObjectiveName(name) => current_objective = name.clone(),
                // An objective type will always be the last part of a new objective
                Token::ObjectiveType(objective) => {
                    let to_add = format!(
                        "scoreboard objectives add {} {}\n",
                        current_objective, objective
                    );

                    current_file!().push_str(&to_add[..]);
                    active_token = Token::None;
                }
                // An int will always be the last part of a variable or objective assignment
                Token::Int(int) => {
                    if active_token == Token::Var {
                        match assignment_operator {
                            Token::InitialSet => {
                                let to_add =
                                    format!("scoreboard objectives add {} dummy\n", current_var);

                                current_file!().push_str(&to_add[..]);

                                let to_add = format!(
                                    "scoreboard players set --databind {} {}",
                                    current_var, int
                                );

                                current_file!().push_str(&to_add[..]);
                            }
                            Token::VarAdd | Token::VarSub | Token::VarSet => {
                                let action = match assignment_operator {
                                    Token::VarAdd => "add",
                                    Token::VarSub => "remove",
                                    _ => "set",
                                };

                                let to_add = format!(
                                    "scoreboard players {} --databind {} {}",
                                    action, &current_var, int
                                );

                                current_file!().push_str(&to_add[..]);
                            }
                            _ => {}
                        }
                    } else if active_token == Token::SetObjective {
                        match assignment_operator {
                            Token::VarSet | Token::VarAdd | Token::VarSub => {
                                let action = match assignment_operator {
                                    Token::VarAdd => "add",
                                    Token::VarSub => "remove",
                                    _ => "set",
                                };

                                let to_add = format!(
                                    "scoreboard players {} {} {} {}",
                                    action, objective_target, &current_objective, int
                                );

                                current_file!().push_str(&to_add[..]);
                            }
                            _ => {
                                panic!(":= operator was tokenized for objective");
                            }
                        }
                    }
                    active_token = Token::None;
                    assignment_operator = Token::None;
                }
                Token::NonDatabind(string) => current_file!().push_str(string),
                Token::NewLine => current_file!().push('\n'),
                _ => {}
            }
        }

        // Remove leading/trailing whitespace from files as well as empty lines
        for file in files.iter_mut() {
            *file = file.trim().to_string();
            *file = file.replace("\n\n", "\n");
        }

        CompileReturn {
            file_contents: files,
            filename_map: filename_to_index,
            tag_map,
        }
    }
}
