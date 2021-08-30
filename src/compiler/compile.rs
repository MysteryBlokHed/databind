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

use crate::compiler::macros::Macro;
use crate::token::Token;
use regex::Regex;
use std::collections::HashMap;

/// Return from the compiler
///
/// # Arguments
///
/// - `file_contents` - A list of file contents
/// - `filename_map` - A map of filenames to indexes in the file_contents Vec
/// - `tag_map` - A map of tags to functions
#[derive(Debug, Clone)]
pub struct CompileReturn {
    pub file_contents: Vec<String>,
    pub filename_map: HashMap<String, usize>,
    pub tag_map: HashMap<String, Vec<String>>,
    pub global_macros: Option<HashMap<String, Macro>>,
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
        global_macros: &HashMap<String, Macro>,
        return_macros: bool,
    ) -> CompileReturn {
        let returned_macros;

        // Parse macros if there are any calls
        let tokens = if tokens.contains(&Token::CallMacro) || return_macros {
            let ret = self.parse_macros(tokens, return_macros, global_macros);
            returned_macros = if return_macros { ret.1 } else { None };
            ret.0
        } else {
            returned_macros = None;
            tokens
        };
        // Parse while loops if there are any
        let tokens = if tokens.contains(&Token::IfStatement) || tokens.contains(&Token::WhileLoop) {
            self.parse_shorthand(tokens, subfolder)
        } else {
            tokens
        };

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
                        files.push("# Compiled with MysteryBlokHed/databind\n".into());
                        filename_to_index.insert(name.clone(), files.len() - 1);
                        current_functions.push(name.clone());
                    // Function call
                    } else {
                        // Function contains namespace
                        let has_namespace = name.contains(':');

                        if has_namespace {
                            let to_add = format!("function {}\n", name);

                            current_file!().push_str(&to_add[..]);
                        } else if let Some(ns) = namespace {
                            let to_add = format!("function {}:{}\n", ns, name);

                            current_file!().push_str(&to_add[..]);
                        } else {
                            panic!("internal: No namespace provided for function call");
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
                        panic!("internal: Tag found outside function. This token should have been ignored");
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
                                    "scoreboard players set --databind {} {}\n",
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
                                    "scoreboard players {} --databind {} {}\n",
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
                                    "scoreboard players {} {} {} {}\n",
                                    action, objective_target, &current_objective, int
                                );

                                current_file!().push_str(&to_add[..]);
                            }
                            _ => {
                                panic!("internal: := operator was tokenized for objective");
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
        let re = Regex::new(r"\n{2,}").unwrap();
        for file in files.iter_mut() {
            *file = file.trim().to_string();
            if file.contains("\n\n") {
                *file = re.replace_all(file, "\n").into();
            }
        }

        CompileReturn {
            file_contents: files,
            filename_map: filename_to_index,
            tag_map,
            global_macros: returned_macros,
        }
    }
}
