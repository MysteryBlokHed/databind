use super::Transpiler;
use crate::token::Token;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

impl Transpiler<'_> {
    /// Convert the provided file contents into a list of tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let assignment_operators = [".=", "=", "+=", "-="];

        let mut last_char = ' ';
        let mut current_keyword = String::new();
        let mut building_keyword = false;
        let mut building_token = Token::None;
        let mut building_while = false;
        let mut while_line = String::new();
        let mut while_lines: Vec<String> = Vec::new();
        let mut building_condition = false;
        let mut remaining_params = 0;
        let mut first_whitespace = false;
        let mut current_non_databind = String::new();
        let mut comment = false;

        while self.current_char != '\u{0}' {
            while comment {
                self.next_char();
                if self.current_char == '\n' {
                    comment = false;
                }
            }

            // When building a while loop, the contents are stored as a string for a token
            // Later, in the transpile function, the while loop is converted to two databind
            // functions.
            if building_while {
                if building_condition {
                    if self.current_char == '\n' {
                        tokens.push(Token::WhileCondition(current_keyword.trim().to_string()));
                        current_keyword = String::new();
                        building_condition = false;
                    }
                } else {
                    while_line.push(self.current_char);
                    if self.current_char == '\n' {
                        current_keyword = String::new();
                        while_lines.push(while_line.trim().to_string());
                        while_line = String::new();
                    }
                }

                if current_keyword.trim() == ":endwhile" {
                    building_while = false;
                    tokens.push(Token::WhileContents(while_lines.join("\n")));
                    while_line = String::new();
                    while_lines = Vec::new();
                    tokens.push(Token::EndWhileLoop);
                }
            }

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
                    "obj" => {
                        tokens.push(Token::Objective);
                        building_token = Token::Objective;
                        remaining_params = 2;
                    }
                    "sobj" => {
                        tokens.push(Token::SetObjective);
                        building_token = Token::SetObjective;
                        remaining_params = 4;
                    }
                    "tvar" => {
                        tokens.push(Token::TestVar);
                        building_token = Token::TestVar;
                    }
                    "func" => {
                        tokens.push(Token::DefineFunc);
                        building_token = Token::DefineFunc;
                    }
                    "endfunc" => {
                        tokens.push(Token::EndFunc);
                        building_keyword = false;
                    }
                    "call" => {
                        tokens.push(Token::CallFunc);
                        building_token = Token::CallFunc;
                    }
                    "while" => {
                        tokens.push(Token::WhileLoop);
                        building_while = true;
                        building_condition = true;
                    }
                    "endwhile" => {
                        tokens.push(Token::EndWhileLoop);
                        building_keyword = false;
                    }
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
                            _ => {
                                if self.current_char.is_whitespace() {
                                    let var_value: i32 = current_keyword.parse().unwrap();
                                    tokens.push(Token::Int(var_value));

                                    building_keyword = false;
                                    building_token = Token::None;
                                    current_keyword = String::new();
                                    first_whitespace = false;
                                } else if DIGITS.contains(&self.current_char) {
                                    current_keyword.push(self.current_char);
                                } else {
                                    println!("[ERROR] Variables can only store integers.");
                                    std::process::exit(1);
                                }
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
                    Token::DefineFunc | Token::CallFunc => {
                        if self.current_char.is_whitespace() {
                            tokens.push(Token::FuncName(current_keyword));

                            building_keyword = false;
                            building_token = Token::None;
                            current_keyword = String::new();
                            first_whitespace = false;
                        } else {
                            current_keyword.push(self.current_char);
                        }
                    }
                    Token::Objective => match remaining_params {
                        2 => {
                            if self.current_char.is_whitespace() {
                                tokens.push(Token::ObjectiveName(current_keyword));
                                current_keyword = String::new();
                                remaining_params -= 1;
                            } else {
                                current_keyword.push(self.current_char);
                            }
                        }
                        _ => {
                            if self.current_char.is_whitespace() {
                                tokens.push(Token::ObjectiveType(current_keyword));
                                building_keyword = false;
                                building_token = Token::None;
                                current_keyword = String::new();
                                first_whitespace = false;
                            } else {
                                current_keyword.push(self.current_char);
                            }
                        }
                    },
                    Token::SetObjective => match remaining_params {
                        4 => {
                            if self.current_char.is_whitespace() {
                                tokens.push(Token::ObjectiveName(current_keyword));
                                current_keyword = String::new();
                                remaining_params -= 1;
                            } else {
                                current_keyword.push(self.current_char);
                            }
                        }
                        3 => {
                            if self.current_char.is_whitespace() {
                                tokens.push(Token::Target(current_keyword));
                                current_keyword = String::new();
                                remaining_params -= 1;
                            } else {
                                current_keyword.push(self.current_char);
                            }
                        }
                        2 => {
                            if self.current_char.is_whitespace() {
                                if assignment_operators.contains(&&current_keyword[..]) {
                                    match &current_keyword[..] {
                                        ".=" => {
                                            println!(
                                                "The .= operator is not valid for objectives."
                                            );
                                            std::process::exit(1);
                                        }
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
                        _ => {
                            if self.current_char.is_whitespace() {
                                let var_value: i32 = current_keyword.parse().unwrap();
                                tokens.push(Token::Int(var_value));

                                building_keyword = false;
                                building_token = Token::None;
                                current_keyword = String::new();
                                first_whitespace = false;
                            } else if DIGITS.contains(&self.current_char) {
                                current_keyword.push(self.current_char);
                            } else {
                                println!("[ERROR] Objectives can only store integers.");
                                std::process::exit(1);
                            }
                        }
                    },
                    _ => {}
                }
            } else if self.current_char == '#'
                && tokens.last().ok_or(Token::None).is_ok()
                && tokens.last().unwrap() == &Token::NewLine
            {
                comment = true;
                continue;
            } else if !['\r', '\n'].contains(&self.current_char) {
                current_non_databind.push(self.current_char);
            } else if current_non_databind.len() > 0 {
                tokens.push(Token::NonDatabind(current_non_databind));
                current_non_databind = String::new();
            }

            if self.current_char == '\n' && !building_while {
                tokens.push(Token::NewLine);
            }

            last_char = self.current_char;
            self.next_char();
        }

        if current_non_databind.len() > 0 {
            tokens.push(Token::NonDatabind(current_non_databind));
        }

        tokens
    }
}
