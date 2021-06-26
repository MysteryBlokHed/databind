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
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;

/// Return from the compiler
///
/// # Arguments
///
/// - `file_contents` - A list of file contents
/// - `filename_map` - A map of filenames to indexes in the file_contents Vec
/// - `var_map` - A map of variable names used in files to randomized names
/// - `tag_map` - A map of tags to functions
pub struct CompileReturn {
    pub file_contents: Vec<String>,
    pub filename_map: HashMap<String, usize>,
    pub var_map: HashMap<String, String>,
    pub tag_map: HashMap<String, Vec<String>>,
}

impl Compiler<'_> {
    /// Convert tokens to a compiled file or files
    ///
    /// # Arguments
    ///
    /// - `tokens` - A list of tokens
    /// - `namespace` - The namespace to use for functions, if relevant
    /// - `existing_var_map` - An existing map of variables to randomized names
    pub fn compile(
        &self,
        tokens: Vec<Token>,
        namespace: Option<&str>,
        existing_var_map: Option<&HashMap<String, String>>,
        subfolder: &str,
    ) -> CompileReturn {
        let tokens = self.parse_shorthand(tokens, subfolder);

        let mut var_map: HashMap<String, String>;
        let mut tag_map: HashMap<String, Vec<String>> = HashMap::new();

        // A vector of file contents
        let mut files: Vec<String> = vec![String::new()];
        // A map of filenames to indexes in the files vector
        let mut filename_to_index: HashMap<String, usize> = HashMap::new();

        if let Some(map) = existing_var_map {
            var_map = map.clone();
        } else {
            var_map = HashMap::new();
        }

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
                Token::VarName(var) => {
                    println!("varname found, active token: {:?}", active_token);
                    if active_token == Token::DeleteVar {
                        if self.settings.random_var_names {
                            if var_map.contains_key(var) {
                                let to_add =
                                    format!("scoreboard objectives remove {}", var_map[var]);

                                if func_depth == 0 {
                                    files[0].push_str(&to_add[..]);
                                } else {
                                    files[filename_to_index[&current_functions[func_depth - 1]]]
                                        .push_str(&to_add[..]);
                                }
                            } else {
                                println!("[ERROR] Attempted test on non-existant variable");
                                std::process::exit(1);
                            }
                        } else {
                            let to_add = format!("scoreboard objectives remove {}", var);
                            if func_depth == 0 {
                                files[0].push_str(&to_add[..]);
                            } else {
                                files[filename_to_index[&current_functions[func_depth - 1]]]
                                    .push_str(&to_add[..]);
                            }
                        }
                    } else {
                        current_var = var.clone();
                        if active_token == Token::TestVar {
                            if self.settings.random_var_names {
                                if var_map.contains_key(var) {
                                    let to_add = format!("score --databind {} ", var_map[var]);

                                    if func_depth == 0 {
                                        files[0].push_str(&to_add[..]);
                                    } else {
                                        files
                                            [filename_to_index[&current_functions[func_depth - 1]]]
                                            .push_str(&to_add[..]);
                                    }
                                } else {
                                    println!("[ERROR] Attempted test on non-existant variable");
                                    std::process::exit(1);
                                }
                            } else {
                                let to_add = format!("score --databind {} ", var);
                                if func_depth == 0 {
                                    files[0].push_str(&to_add[..]);
                                } else {
                                    files[filename_to_index[&current_functions[func_depth - 1]]]
                                        .push_str(&to_add[..]);
                                }
                            }
                        }
                    }
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
                            if func_depth == 0 {
                                files[0].push_str(&to_add[..]);
                            } else {
                                files[filename_to_index[&current_functions[func_depth - 1]]]
                                    .push_str(&to_add[..]);
                            }
                        } else if let Some(ns) = namespace {
                            let to_add = format!("function {}:{}", ns, name);
                            if func_depth == 0 {
                                files[0].push_str(&to_add[..]);
                            } else {
                                files[filename_to_index[&current_functions[func_depth - 1]]]
                                    .push_str(&to_add[..]);
                            }
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
                    if self.settings.random_var_names {
                        if !var_map.contains_key(&current_objective) {
                            let mut random_name = current_objective.clone();
                            let extension: String = rand::thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(4)
                                .map(char::from)
                                .collect();
                            random_name.push('-');
                            random_name.push_str(&extension[..]);

                            var_map.insert(current_objective.clone(), random_name);
                            if self.settings.var_display_names {
                                let to_add = format!(
                                    "scoreboard objectives add {} {} {{\"text\":\"{}\"}}\n",
                                    var_map[&current_objective], objective, current_objective
                                );

                                if func_depth == 0 {
                                    files[0].push_str(&to_add[..]);
                                } else {
                                    files[filename_to_index[&current_functions[func_depth - 1]]]
                                        .push_str(&to_add[..]);
                                }
                            } else {
                                let to_add = format!(
                                    "scoreboard objectives add {} {}\n",
                                    var_map[&current_objective], objective
                                );

                                if func_depth == 0 {
                                    files[0].push_str(&to_add[..]);
                                } else {
                                    files[filename_to_index[&current_functions[func_depth - 1]]]
                                        .push_str(&to_add[..]);
                                }
                            }
                        } else {
                            println!("[ERROR] Attempted creation of already-existing objective.");
                            std::process::exit(1);
                        }
                    } else {
                        let to_add = format!(
                            "scoreboard objectives add {} {}\n",
                            current_objective, objective
                        );

                        if func_depth == 0 {
                            files[0].push_str(&to_add[..]);
                        } else {
                            files[filename_to_index[&current_functions[func_depth - 1]]]
                                .push_str(&to_add[..]);
                        }
                    }
                    active_token = Token::None;
                }
                // An int will always be the last part of a variable or objective assignment
                Token::Int(int) => {
                    if active_token == Token::Var {
                        match assignment_operator {
                            Token::InitialSet => {
                                if self.settings.random_var_names {
                                    if !var_map.contains_key(&current_var) {
                                        let mut random_name = current_var.clone();
                                        let extension: String = rand::thread_rng()
                                            .sample_iter(&Alphanumeric)
                                            .take(4)
                                            .map(char::from)
                                            .collect();
                                        random_name.push('-');
                                        random_name.push_str(&extension[..]);

                                        var_map.insert(current_var.clone(), random_name);
                                        if self.settings.var_display_names {
                                            let to_add = format!(
                                                "scoreboard objectives add {} dummy {{\"text\":\"{}\"}}\n",
                                                var_map[&current_var], current_var
                                            );

                                            if func_depth == 0 {
                                                files[0].push_str(&to_add[..]);
                                            } else {
                                                files[filename_to_index
                                                    [&current_functions[func_depth - 1]]]
                                                    .push_str(&to_add[..]);
                                            }
                                        } else {
                                            let to_add = format!(
                                                "scoreboard objectives add {} dummy\n",
                                                var_map[&current_var]
                                            );

                                            if func_depth == 0 {
                                                files[0].push_str(&to_add[..]);
                                            } else {
                                                files[filename_to_index
                                                    [&current_functions[func_depth - 1]]]
                                                    .push_str(&to_add[..]);
                                            }
                                        }
                                        let to_add = format!(
                                            "scoreboard players set --databind {} {}",
                                            var_map[&current_var], int
                                        );

                                        if func_depth == 0 {
                                            files[0].push_str(&to_add[..]);
                                        } else {
                                            files[filename_to_index
                                                [&current_functions[func_depth - 1]]]
                                                .push_str(&to_add[..]);
                                        }
                                    } else {
                                        println!(
                                            "[ERROR] Attempted creation of already-existing variable."
                                        );
                                        std::process::exit(1);
                                    }
                                } else {
                                    let to_add = format!(
                                        "scoreboard objectives add {} dummy\n",
                                        current_var
                                    );

                                    if func_depth == 0 {
                                        files[0].push_str(&to_add[..]);
                                    } else {
                                        files
                                            [filename_to_index[&current_functions[func_depth - 1]]]
                                            .push_str(&to_add[..]);
                                    }

                                    let to_add = format!(
                                        "scoreboard players set --databind {} {}",
                                        current_var, int
                                    );

                                    if func_depth == 0 {
                                        files[0].push_str(&to_add[..]);
                                    } else {
                                        files
                                            [filename_to_index[&current_functions[func_depth - 1]]]
                                            .push_str(&to_add[..]);
                                    }
                                }
                            }
                            Token::VarAdd | Token::VarSub | Token::VarSet => {
                                let action = match assignment_operator {
                                    Token::VarAdd => "add",
                                    Token::VarSub => "remove",
                                    _ => "set",
                                };

                                if self.settings.random_var_names {
                                    if var_map.contains_key(&current_var) {
                                        let to_add = format!(
                                            "scoreboard players {} --databind {} {}",
                                            action, var_map[&current_var], int
                                        );

                                        if func_depth == 0 {
                                            files[0].push_str(&to_add[..]);
                                        } else {
                                            files[filename_to_index
                                                [&current_functions[func_depth - 1]]]
                                                .push_str(&to_add[..]);
                                        }
                                    } else {
                                        println!(
                                            "[ERROR] Attempted {} of non-existant variable",
                                            action
                                        );
                                        std::process::exit(1);
                                    }
                                } else {
                                    let to_add = format!(
                                        "scoreboard players {} --databind {} {}",
                                        action, &current_var, int
                                    );

                                    if func_depth == 0 {
                                        files[0].push_str(&to_add[..]);
                                    } else {
                                        files
                                            [filename_to_index[&current_functions[func_depth - 1]]]
                                            .push_str(&to_add[..]);
                                    }
                                }
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

                                if self.settings.random_var_names {
                                    if var_map.contains_key(&current_objective) {
                                        let to_add = format!(
                                            "scoreboard players {} {} {} {}",
                                            action,
                                            objective_target,
                                            var_map[&current_objective],
                                            int
                                        );

                                        if func_depth == 0 {
                                            files[0].push_str(&to_add[..]);
                                        } else {
                                            files[filename_to_index
                                                [&current_functions[func_depth - 1]]]
                                                .push_str(&to_add[..]);
                                        }
                                    } else {
                                        println!(
                                            "[ERROR] Attempted {} of non-existant variable",
                                            action
                                        );
                                        std::process::exit(1);
                                    }
                                } else {
                                    let to_add = format!(
                                        "scoreboard players {} {} {} {}",
                                        action, objective_target, &current_objective, int
                                    );

                                    if func_depth == 0 {
                                        files[0].push_str(&to_add[..]);
                                    } else {
                                        files
                                            [filename_to_index[&current_functions[func_depth - 1]]]
                                            .push_str(&to_add[..]);
                                    }
                                }
                            }
                            _ => {
                                panic!(".= operator was tokenized for objective");
                            }
                        }
                    }
                    active_token = Token::None;
                    assignment_operator = Token::None;
                }
                Token::NonDatabind(string) => {
                    if func_depth == 0 {
                        files[0].push_str(string);
                    } else {
                        files[filename_to_index[&current_functions[func_depth - 1]]]
                            .push_str(string);
                    }
                }
                Token::DeleteVar => active_token = Token::DeleteVar,
                Token::NewLine => {
                    if func_depth == 0 {
                        files[0].push('\n');
                    } else {
                        files[filename_to_index[&current_functions[func_depth - 1]]].push('\n');
                    }
                }
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
            var_map,
            tag_map,
        }
    }
}
