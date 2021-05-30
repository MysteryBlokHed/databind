use crate::token::Token;

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
        let mut comment = false;

        while self.current_char != '\u{0}' {
            while comment {
                self.next_char();
                if self.current_char == '\n' {
                    comment = false;
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
            } else if self.current_char == '#' && tokens.last().unwrap() == &Token::NewLine {
                comment = true;
                continue;
            } else if !['\r', '\n'].contains(&self.current_char) {
                current_non_databind.push(self.current_char);
            } else if current_non_databind.len() > 0 {
                tokens.push(Token::NonDatabind(current_non_databind));
                current_non_databind = String::new();
            }

            if self.current_char == '\n' {
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
