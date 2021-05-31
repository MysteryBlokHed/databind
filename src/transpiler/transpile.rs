use super::{TranspileReturn, Transpiler};

use crate::token::Token;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;

impl Transpiler<'_> {
    pub fn transpile(
        &self,
        tokens: Vec<Token>,
        namespace: Option<&str>,
        existing_var_map: Option<&HashMap<String, String>>,
        return_var_map: bool,
        return_multi_file: bool,
    ) -> TranspileReturn {
        // let mut transpiled = String::new();
        let mut var_map: HashMap<String, String>;

        // A vector of file contents
        let mut files: Vec<String> = vec![String::new()];
        // A map of filenames to indexes in the files vector
        let mut filename_to_index: HashMap<String, usize> = HashMap::new();
        filename_to_index.insert(String::new(), 0);

        if let Some(_) = existing_var_map {
            var_map = existing_var_map.unwrap().clone();
        } else {
            var_map = HashMap::new();
        }

        let mut active_token = Token::None;

        // For variable-related tokens
        let mut current_var = String::new();
        let mut assignment_operator = Token::None;
        // For functions
        let mut in_function = false;
        let mut current_function = String::new();
        let mut calling_function = false;
        // For objective-related tokens
        let mut current_objective = String::new();
        let mut objective_target = String::new();

        for token in tokens.iter() {
            match token {
                Token::Var => active_token = Token::Var,
                Token::TestVar => active_token = Token::TestVar,
                Token::Objective => active_token = Token::Objective,
                Token::SetObjective => active_token = Token::SetObjective,
                Token::Target(target) => objective_target = target.clone(),
                Token::VarName(var) => {
                    current_var = var.clone();
                    if active_token == Token::TestVar {
                        if self.settings.randomize_var_names {
                            if var_map.contains_key(var) {
                                let to_add = format!("score --databind {} ", var_map[var]);

                                if !in_function {
                                    files[0].push_str(&to_add[..]);
                                } else {
                                    files[filename_to_index[&current_function]]
                                        .push_str(&to_add[..]);
                                }
                            } else {
                                println!("[ERROR] Attempted test on non-existant variable");
                                std::process::exit(1);
                            }
                        } else {
                            let to_add = format!("score --databind {} ", var);
                            if !in_function {
                                files[0].push_str(&to_add[..]);
                            } else {
                                files[filename_to_index[&current_function]].push_str(&to_add[..]);
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
                        files.push(String::new());
                        filename_to_index.insert(name.clone(), files.len() - 1);
                        current_function = name.clone();
                    // Function call
                    } else {
                        // Function contains namespace
                        let has_namespace = name.contains(':');

                        if has_namespace {
                            let to_add = format!("function {}", name);
                            if !in_function {
                                files[0].push_str(&to_add[..]);
                            } else {
                                files[filename_to_index[&current_function]].push_str(&to_add[..]);
                            }
                        } else {
                            if let Some(ns) = namespace {
                                let to_add = format!("function {}:{}", ns, name);
                                if !in_function {
                                    files[0].push_str(&to_add[..]);
                                } else {
                                    files[filename_to_index[&current_function]]
                                        .push_str(&to_add[..]);
                                }
                            } else {
                                println!("No namespace provided for function call.");
                                std::process::exit(2);
                            }
                        }

                        calling_function = false;
                    }
                }
                Token::CallFunc => calling_function = true,
                Token::DefineFunc => in_function = true,
                Token::EndFunc => in_function = false,
                Token::ObjectiveName(name) => current_objective = name.clone(),
                // An objective type will always be the last part of a new objective
                Token::ObjectiveType(objective) => {
                    if self.settings.randomize_var_names {
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
                            if self.settings.var_display_name {
                                let to_add = format!(
                                    "scoreboard objectives add {} {} {{\"text\":\"{}\"}}\n",
                                    var_map[&current_objective], objective, current_objective
                                );

                                if !in_function {
                                    files[0].push_str(&to_add[..]);
                                } else {
                                    files[filename_to_index[&current_function]]
                                        .push_str(&to_add[..]);
                                }
                            } else {
                                let to_add = format!(
                                    "scoreboard objectives add {} {}\n",
                                    var_map[&current_objective], objective
                                );

                                if !in_function {
                                    files[0].push_str(&to_add[..]);
                                } else {
                                    files[filename_to_index[&current_function]]
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

                        if !in_function {
                            files[0].push_str(&to_add[..]);
                        } else {
                            files[filename_to_index[&current_function]].push_str(&to_add[..]);
                        }
                    }
                    active_token = Token::None;
                }
                // An int will always be the last part of a variable or objective assignment
                Token::Int(int) => {
                    if active_token == Token::Var {
                        match assignment_operator {
                            Token::InitialSet => {
                                if self.settings.randomize_var_names {
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
                                        if self.settings.var_display_name {
                                            let to_add = format!(
                                                "scoreboard objectives add {} dummy {{\"text\":\"{}\"}}\n",
                                                var_map[&current_var], current_var
                                            );

                                            if !in_function {
                                                files[0].push_str(&to_add[..]);
                                            } else {
                                                files[filename_to_index[&current_function]]
                                                    .push_str(&to_add[..]);
                                            }
                                        } else {
                                            let to_add = format!(
                                                "scoreboard objectives add {} dummy\n",
                                                var_map[&current_var]
                                            );

                                            if !in_function {
                                                files[0].push_str(&to_add[..]);
                                            } else {
                                                files[filename_to_index[&current_function]]
                                                    .push_str(&to_add[..]);
                                            }
                                        }
                                        let to_add = format!(
                                            "scoreboard players set --databind {} {}",
                                            var_map[&current_var], int
                                        );

                                        if !in_function {
                                            files[0].push_str(&to_add[..]);
                                        } else {
                                            files[filename_to_index[&current_function]]
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

                                    if !in_function {
                                        files[0].push_str(&to_add[..]);
                                    } else {
                                        files[filename_to_index[&current_function]]
                                            .push_str(&to_add[..]);
                                    }

                                    let to_add = format!(
                                        "scoreboard players set --databind {} {}",
                                        current_var, int
                                    );

                                    if !in_function {
                                        files[0].push_str(&to_add[..]);
                                    } else {
                                        files[filename_to_index[&current_function]]
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

                                if self.settings.randomize_var_names {
                                    if var_map.contains_key(&current_var) {
                                        let to_add = format!(
                                            "scoreboard players {} --databind {} {}",
                                            action, var_map[&current_var], int
                                        );

                                        if !in_function {
                                            files[0].push_str(&to_add[..]);
                                        } else {
                                            files[filename_to_index[&current_function]]
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

                                    if !in_function {
                                        files[0].push_str(&to_add[..]);
                                    } else {
                                        files[filename_to_index[&current_function]]
                                            .push_str(&to_add[..]);
                                    }
                                }
                            }
                            _ => {}
                        }
                    } else if active_token == Token::SetObjective {
                        match assignment_operator {
                            Token::VarSet => {
                                let action = match assignment_operator {
                                    Token::VarAdd => "add",
                                    Token::VarSub => "remove",
                                    _ => "set",
                                };

                                if self.settings.randomize_var_names {
                                    if var_map.contains_key(&current_objective) {
                                        let to_add = format!(
                                            "scoreboard players {} {} {} {}",
                                            action,
                                            objective_target,
                                            var_map[&current_objective],
                                            int
                                        );

                                        if !in_function {
                                            files[0].push_str(&to_add[..]);
                                        } else {
                                            files[filename_to_index[&current_function]]
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

                                    if !in_function {
                                        files[0].push_str(&to_add[..]);
                                    } else {
                                        files[filename_to_index[&current_function]]
                                            .push_str(&to_add[..]);
                                    }
                                }
                            }
                            _ => {
                                println!(".= operator was mistakenly tokenized for objective");
                                std::process::exit(2);
                            }
                        }
                    }
                    active_token = Token::None;
                    assignment_operator = Token::None;
                }
                Token::NonDatabind(string) => {
                    if !in_function {
                        files[0].push_str(string);
                    } else {
                        files[filename_to_index[&current_function]].push_str(string);
                    }
                }
                Token::NewLine => {
                    if !in_function {
                        files[0].push('\n');
                    } else {
                        files[filename_to_index[&current_function]].push('\n');
                    }
                }
                _ => {}
            }
        }

        // Remove leading/trailing whitespace from files
        for file in files.iter_mut() {
            *file = file.trim().to_string();
        }

        if return_var_map && return_multi_file {
            TranspileReturn::MultiFileAndMap(files, filename_to_index, var_map)
        } else if !return_var_map && !return_multi_file {
            TranspileReturn::SingleContents(files[0].clone())
        } else if return_var_map && !return_multi_file {
            TranspileReturn::SingleContentsAndMap(files[0].clone(), var_map)
        } else {
            TranspileReturn::MultiFile(files, filename_to_index)
        }
    }
}
