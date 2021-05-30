use super::Transpiler;

use crate::token::Token;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;

impl Transpiler<'_> {
    pub fn transpile(
        &self,
        tokens: Vec<Token>,
        existing_var_map: Option<&HashMap<String, String>>,
        return_var_map: bool,
    ) -> (String, Option<HashMap<String, String>>) {
        let mut transpiled = String::new();
        let mut var_map: HashMap<String, String>;

        if let Some(_) = existing_var_map {
            var_map = existing_var_map.unwrap().clone();
        } else {
            var_map = HashMap::new();
        }

        let mut active_token = Token::None;

        // For variable-related tokens
        let mut current_var = String::new();
        let mut assignment_operator = Token::None;

        for token in tokens.iter() {
            match token {
                Token::Var => active_token = Token::Var,
                Token::TestVar => active_token = Token::TestVar,
                Token::VarName(var) => {
                    current_var = var.clone();
                    if active_token == Token::TestVar {
                        if self.settings.randomize_var_names {
                            if var_map.contains_key(var) {
                                transpiled
                                    .push_str(&format!("score --databind {} ", var_map[var])[..]);
                            } else {
                                println!("[ERROR] Attempted test on non-existant variable");
                                std::process::exit(1);
                            }
                        } else {
                            transpiled.push_str(&format!("score --databind {} ", var)[..]);
                        }
                    }
                }
                Token::InitialSet => assignment_operator = Token::InitialSet,
                Token::VarSet => assignment_operator = Token::VarSet,
                Token::VarAdd => assignment_operator = Token::VarAdd,
                Token::VarSub => assignment_operator = Token::VarSub,
                Token::Int(int) => {
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
                                        transpiled.push_str(
                                            &format!(
                                        "scoreboard objectives add {} dummy {{\"text\":\"{}\"}}\n",
                                        var_map[&current_var], current_var
                                    )[..],
                                        );
                                    } else {
                                        transpiled.push_str(
                                            &format!(
                                                "scoreboard objectives add {} dummy\n",
                                                var_map[&current_var]
                                            )[..],
                                        );
                                    }
                                    transpiled.push_str(
                                        &format!(
                                            "scoreboard players set --databind {} {}",
                                            var_map[&current_var], int
                                        )[..],
                                    );
                                } else {
                                    println!(
                                        "[ERROR] Attempted creation of already-existing variable."
                                    );
                                    std::process::exit(1);
                                }
                            } else {
                                transpiled.push_str(
                                &format!("scoreboard objectives add {} dummy\n", current_var)[..],
                            );
                                transpiled.push_str(
                                    &format!(
                                        "scoreboard players set --databind {} {}",
                                        current_var, int
                                    )[..],
                                );
                            }
                        }
                        Token::VarSet => {
                            if self.settings.randomize_var_names {
                                if var_map.contains_key(&current_var) {
                                    transpiled.push_str(
                                        &format!(
                                            "scoreboard players set --databind {} {}",
                                            var_map[&current_var], int
                                        )[..],
                                    );
                                } else {
                                    println!("[ERROR] Attempted set of non-existant variable");
                                    std::process::exit(1);
                                }
                            } else {
                                transpiled.push_str(
                                    &format!(
                                        "scoreboard players set --databind {} {}",
                                        &current_var, int
                                    )[..],
                                );
                            }
                        }
                        Token::VarAdd => {
                            if self.settings.randomize_var_names {
                                if var_map.contains_key(&current_var) {
                                    transpiled.push_str(
                                        &format!(
                                            "scoreboard players add --databind {} {}",
                                            var_map[&current_var], int
                                        )[..],
                                    );
                                } else {
                                    println!("[ERROR] Attempted add to non-existant variable");
                                    std::process::exit(1);
                                }
                            } else {
                                transpiled.push_str(
                                    &format!(
                                        "scoreboard players add --databind {} {}",
                                        &current_var, int
                                    )[..],
                                );
                            }
                        }
                        Token::VarSub => {
                            if self.settings.randomize_var_names {
                                if var_map.contains_key(&current_var) {
                                    transpiled.push_str(
                                        &format!(
                                            "scoreboard players remove --databind {} {}",
                                            var_map[&current_var], int
                                        )[..],
                                    );
                                } else {
                                    println!(
                                        "[ERROR] Attempted subtract from non-existant variable"
                                    );
                                    std::process::exit(1);
                                }
                            } else {
                                transpiled.push_str(
                                    &format!(
                                        "scoreboard players remove --databind {} {}",
                                        &current_var, int
                                    )[..],
                                );
                            }
                        }
                        _ => {}
                    }
                    active_token = Token::None;
                    assignment_operator = Token::None;
                }
                Token::NonDatabind(string) => transpiled.push_str(string),
                Token::NewLine => transpiled.push('\n'),
                _ => {}
            }
        }

        if return_var_map {
            (transpiled, Some(var_map))
        } else {
            (transpiled, None)
        }
    }
}
