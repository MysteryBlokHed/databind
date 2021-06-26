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

const DIGITS: [char; 11] = ['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

impl Compiler {
    /// Convert the provided file contents into a list of tokens
    pub fn tokenize(&mut self, get_definitions: bool) -> Vec<Token> {
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

        /// Add a token to the list
        ///
        /// # Returns
        ///
        /// `true` if a token was found and added, `false` otherwise
        macro_rules! add_token {
            ($token: expr) => {
                if self.current_char.is_whitespace() {
                    tokens.push($token);
                    current_keyword = String::new();
                    true
                } else {
                    current_keyword.push(self.current_char);
                    false
                }
            };
        }

        /// Add a token to the list and reset variables
        macro_rules! add_token_and_reset {
            ($token: expr) => {
                if add_token!($token) {
                    building_keyword = false;
                    building_token = Token::None;
                    first_whitespace = false;
                }
            };
        }

        /// Add an integer to the token list and reset variables
        macro_rules! add_int_and_reset {
            () => {
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
            };
        }

        /// Add a token to the tokens list and set the token being built
        macro_rules! set_building {
            ($token: expr) => {
                tokens.push($token);
                building_token = $token;
            };
        }

        /// Try to find an assignment operator
        ///
        /// # Returns
        ///
        /// `true` if an operator was found, `false` otherwise
        macro_rules! assignment_operator {
            () => {
                if self.current_char.is_whitespace() {
                    if assignment_operators.contains(&&current_keyword[..]) {
                        match &current_keyword[..] {
                            ".=" => tokens.push(Token::InitialSet),
                            "=" => tokens.push(Token::VarSet),
                            "+=" => tokens.push(Token::VarAdd),
                            "-=" => tokens.push(Token::VarSub),
                            _ => {
                                panic!("Someone didn't update the assignment operator match!");
                            }
                        };
                    } else {
                        println!("Invalid assignment operator provided");
                        std::process::exit(1);
                    }
                    current_keyword = String::new();
                    true
                } else {
                    current_keyword.push(self.current_char);
                    false
                }
            };
        }

        while self.current_char != '\u{0}' {
            while comment {
                self.next_char();
                if self.current_char == '\n' {
                    comment = false;
                }
            }

            if get_definitions && !tokens.is_empty() {
                match tokens.last().unwrap() {
                    Token::DefineReplace | Token::ReplaceName(_) | Token::ReplaceContents(_) => {}
                    _ => break,
                }
            }

            // When building a while loop, the contents are stored as a string for a token
            // Later, in the compile function, the while loop is converted to two databind
            // functions.
            if building_while {
                if current_keyword.trim() == ":endwhile" {
                    building_while = false;
                    building_keyword = false;
                    current_keyword = String::new();
                    tokens.push(Token::WhileContents(while_lines.join("\n")));
                    while_line = String::new();
                    while_lines = Vec::new();
                    tokens.push(Token::EndWhileLoop);
                }

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
            }

            if !building_keyword && last_char.is_whitespace() && self.current_char == ':' {
                building_keyword = true;
                if !current_non_databind.is_empty() {
                    tokens.push(Token::NonDatabind(current_non_databind));
                    current_non_databind = String::new();
                }
            } else if building_keyword && building_token == Token::None {
                current_keyword.push(self.current_char);
                let mut keyword_found = true;
                match &current_keyword[..] {
                    "var" => {
                        set_building!(Token::Var);
                        remaining_params = 3;
                    }
                    "obj" => {
                        set_building!(Token::Objective);
                        remaining_params = 2;
                    }
                    "sobj" => {
                        set_building!(Token::SetObjective);
                        remaining_params = 4;
                    }
                    "tvar" => {
                        set_building!(Token::TestVar);
                    }
                    "func" => {
                        set_building!(Token::DefineFunc);
                    }
                    "endfunc" => {
                        tokens.push(Token::EndFunc);
                        building_keyword = false;
                    }
                    "def" => {
                        set_building!(Token::DefineReplace);
                        remaining_params = 2;
                    }
                    "tag" => {
                        set_building!(Token::Tag);
                    }
                    "call" => {
                        set_building!(Token::CallFunc);
                    }
                    "while" => {
                        tokens.push(Token::WhileLoop);
                        building_while = true;
                        building_condition = true;
                    }
                    "gvar" => {
                        set_building!(Token::GetVar);
                    }
                    "sbop" => {
                        tokens.push(Token::ScoreboardOperation);
                        building_keyword = false;
                    }
                    "delvar" | "delobj" => {
                        set_building!(Token::DeleteVar);
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
                                if add_token!(Token::ModVarName(current_keyword)) {
                                    remaining_params -= 1;
                                }
                            }
                            // Assignment operator
                            2 => {
                                if assignment_operator!() {
                                    remaining_params -= 1;
                                }
                            }
                            // Value
                            _ => add_int_and_reset!(),
                        }
                    }
                    Token::TestVar => {
                        add_token_and_reset!(Token::TestVarName(current_keyword));
                    }
                    Token::DefineFunc | Token::CallFunc => {
                        add_token_and_reset!(Token::FuncName(current_keyword));
                    }
                    Token::Objective => match remaining_params {
                        2 => {
                            if add_token!(Token::ObjectiveName(current_keyword)) {
                                remaining_params -= 1;
                            }
                        }
                        _ => add_token_and_reset!(Token::ObjectiveType(current_keyword)),
                    },
                    Token::SetObjective => match remaining_params {
                        4 => {
                            if add_token!(Token::Target(current_keyword)) {
                                remaining_params -= 1;
                            }
                        }
                        3 => {
                            if add_token!(Token::ObjectiveName(current_keyword)) {
                                remaining_params -= 1;
                            }
                        }
                        2 => {
                            if assignment_operator!() {
                                remaining_params -= 1;
                            }
                        }
                        _ => add_int_and_reset!(),
                    },
                    Token::Tag => add_token_and_reset!(Token::TagName(current_keyword)),
                    Token::DefineReplace => match remaining_params {
                        2 => {
                            if add_token!(Token::ReplaceName(current_keyword)) {
                                remaining_params -= 1;
                            }
                        }
                        _ => {
                            if ['\r', '\n'].contains(&self.current_char) {
                                tokens.push(Token::ReplaceContents(current_keyword));
                                building_keyword = false;
                                building_token = Token::None;
                                current_keyword = String::new();
                                first_whitespace = false;
                            } else {
                                current_keyword.push(self.current_char);
                            }
                        }
                    },
                    Token::GetVar => add_token_and_reset!(Token::OpVarName(current_keyword)),
                    Token::DeleteVar => add_token_and_reset!(Token::DelVarName(current_keyword)),
                    _ => {}
                };
            } else if self.current_char == '#'
                && tokens.last().ok_or(Token::None).is_ok()
                && tokens.last().unwrap() == &Token::NewLine
                && current_non_databind.is_empty()
            {
                comment = true;
                continue;
            } else if !['\r', '\n'].contains(&self.current_char) {
                current_non_databind.push(self.current_char);
            } else if !current_non_databind.is_empty() {
                tokens.push(Token::NonDatabind(current_non_databind));
                current_non_databind = String::new();
            }

            if self.current_char == '\n' && !building_while {
                tokens.push(Token::NewLine);
            }

            last_char = self.current_char;
            self.next_char();
        }

        if !current_non_databind.is_empty() {
            tokens.push(Token::NonDatabind(current_non_databind));
        }

        tokens
    }
}
