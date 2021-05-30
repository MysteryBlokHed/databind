use crate::settings::Settings;
use crate::token::Token;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub struct Lexer {
    chars: Vec<char>,
    position: usize,
    current_char: char,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        let first_char = text.chars().nth(0).unwrap();

        Lexer {
            chars: text.chars().collect(),
            position: 0,
            current_char: first_char,
        }
    }

    fn next_char(&mut self) {
        self.position += 1;
        if self.position < self.chars.len() {
            self.current_char = self.chars[self.position];
        } else {
            self.current_char = '\u{0}'
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let assignment_operators = [".=", "=", "+=", "-="];

        let mut last_char = ' ';
        let mut current_keyword = String::new();
        let mut building_keyword = false;
        let mut building_token = Token::None;
        let mut remaining_params = 0;
        let mut first_whitespace = false;
        let mut current_non_databind = String::new();

        while self.current_char != '\u{0}' {
            println!("CURRENT NON DATABIND: |{}|", current_non_databind);
            if !building_keyword && last_char.is_whitespace() && self.current_char == ':' {
                building_keyword = true;
                if current_non_databind.len() > 0 {
                    tokens.push(Token::NonDatabind(current_non_databind));
                    current_non_databind = String::new();
                }
            } else if building_keyword && building_token == Token::None {
                current_keyword.push(self.current_char);
                let mut keyword_found = true;
                match &current_keyword[..] {
                    "var" => {
                        tokens.push(Token::Var);
                        building_token = Token::Var;
                        remaining_params = 3;
                    }
                    "def" => {
                        tokens.push(Token::CreateDef);
                        building_token = Token::CreateDef;
                        remaining_params = 3;
                    }
                    "gdef" => {
                        tokens.push(Token::GetDef);
                        building_token = Token::CreateDef;
                        remaining_params = 1;
                    }
                    "obj" => {
                        tokens.push(Token::Objective);
                        building_token = Token::Objective;
                        remaining_params = 2;
                    }
                    "tvar" => {
                        tokens.push(Token::TestVar);
                        building_token = Token::TestVar;
                        // remaining_params = 1;
                    }
                    "func" => tokens.push(Token::DefineFunc),
                    "endfunc" => tokens.push(Token::EndFunc),
                    "call" => tokens.push(Token::CallFunc),
                    _ => keyword_found = false,
                }

                if keyword_found {
                    current_keyword = String::new();
                }
            } else if building_keyword {
                if self.current_char.is_whitespace() && !first_whitespace {
                    first_whitespace = true;
                    last_char = self.current_char;
                    self.next_char();
                    continue;
                }

                match building_token {
                    Token::Var => {
                        match remaining_params {
                            // Variable name
                            3 => {
                                if self.current_char.is_whitespace() {
                                    tokens.push(Token::VarName(current_keyword));
                                    current_keyword = String::new();
                                    remaining_params -= 1;
                                } else {
                                    current_keyword.push(self.current_char);
                                }
                            }
                            // Assignment operator
                            2 => {
                                if self.current_char.is_whitespace() {
                                    if assignment_operators.contains(&&current_keyword[..]) {
                                        match &current_keyword[..] {
                                            ".=" => tokens.push(Token::InitialSet),
                                            "=" => tokens.push(Token::VarSet),
                                            "+=" => tokens.push(Token::VarAdd),
                                            "-=" => tokens.push(Token::VarSub),
                                            _ => {
                                                println!("[ERROR] Someone didn't update the assignment operator match!");
                                                std::process::exit(2);
                                            }
                                        }
                                        current_keyword = String::new();
                                        remaining_params -= 1;
                                    }
                                } else {
                                    current_keyword.push(self.current_char);
                                }
                            }
                            // Value
                            1 => {
                                if self.current_char.is_whitespace() {
                                    let var_value: i32 = current_keyword.parse().unwrap();
                                    tokens.push(Token::Int(var_value));
                                    remaining_params -= 1;
                                } else if DIGITS.contains(&self.current_char) {
                                    current_keyword.push(self.current_char);
                                } else {
                                    println!("[ERROR] Variables can only contain integers.");
                                    std::process::exit(1);
                                }
                            }
                            _ => {
                                building_keyword = false;
                                building_token = Token::None;
                                current_keyword = String::new();
                                first_whitespace = false;
                            }
                        }
                    }
                    Token::TestVar => {
                        if self.current_char.is_whitespace() {
                            tokens.push(Token::VarName(current_keyword));
                            building_keyword = false;
                            building_token = Token::None;
                            current_keyword = String::new();
                            first_whitespace = false;
                        } else {
                            current_keyword.push(self.current_char);
                        }
                    }
                    _ => {}
                }
            } else if !['\r', '\n'].contains(&self.current_char) {
                current_non_databind.push(self.current_char);
            } else if current_non_databind.len() > 0 {
                tokens.push(Token::NonDatabind(current_non_databind));
                current_non_databind = String::new();
            }

            last_char = self.current_char;
            self.next_char();
        }

        if current_non_databind.len() > 0 {
            tokens.push(Token::NonDatabind(current_non_databind));
        }

        tokens
    }

    pub fn transpile(&self, tokens: Vec<Token>, settings: Settings) -> String {
        let mut transpiled = String::new();

        let mut var_map: HashMap<String, String> = HashMap::new();

        let mut active_token = Token::None;

        // For variable-related tokens
        let mut current_var = String::new();
        let mut assignment_operator = Token::None;

        for token in tokens.iter() {
            match token {
                Token::Var => active_token = Token::Var,
                Token::VarName(var) => {
                    current_var = var.clone();
                }
                Token::InitialSet => assignment_operator = Token::InitialSet,
                Token::Int(int) => {
                    match assignment_operator {
                        Token::InitialSet => {
                            if settings.randomize_var_names {
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
                                    if settings.var_display_name {
                                        transpiled.push_str(
                                        &format!(
                                            "scoreboard objectives add {} dummy {{\"text\":\"{}\"}}",
                                            var_map[&current_var], current_var
                                        )[..],
                                    );
                                    } else {
                                        transpiled.push_str(
                                            &format!(
                                                "scoreboard objectives add {} dummy",
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
                                    &format!("scoreboard objectives add {} dummy", current_var)[..],
                                );
                                transpiled.push_str(&format!(
                                    "scoreboard players set --databind {} {}",
                                    current_var, int
                                ));
                            }
                        }
                        _ => {}
                    }
                    active_token = Token::None;
                    assignment_operator = Token::None;
                }
                Token::NonDatabind(string) => {
                    transpiled.push_str(string);
                }
                _ => {}
            }
        }

        transpiled
    }
}
