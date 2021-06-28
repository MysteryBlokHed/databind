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
const ASSIGNMENT_OPERATORS: [&str; 4] = [".=", "=", "+=", "-="];

impl Compiler {
    /// Convert the provided file contents into a list of tokens
    pub fn tokenize(&mut self, get_definitions: bool) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut comment = false;
        let mut building_first_token = true;
        let mut current_keyword = String::new();

        let mut building_while = false;
        let mut building_while_condition = false;

        let mut building_for = Token::None;
        let mut params_left = 0;

        macro_rules! set_building {
            ($token: expr, $params: expr) => {
                current_keyword = String::new();
                tokens.push($token);
                building_first_token = false;
                building_for = $token;
                params_left = $params;
                self.next_char();
            };
        }

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
                    params_left -= 1;
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
                    building_for = Token::None;
                    building_first_token = true;
                }
            };
        }

        macro_rules! no_args_add {
            ($token: expr) => {
                tokens.push($token);
                current_keyword = String::new();
            };
        }

        /// Add an integer to the token list and reset variables
        macro_rules! add_int_and_reset {
            () => {
                if self.current_char.is_whitespace() {
                    let var_value: i32 = current_keyword.parse().unwrap();
                    tokens.push(Token::Int(var_value));
                    building_for = Token::None;
                    building_first_token = true;
                    current_keyword = String::new();
                } else if DIGITS.contains(&self.current_char) {
                    current_keyword.push(self.current_char);
                } else {
                    println!("[ERROR] Variables can only store integers.");
                    std::process::exit(1);
                }
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
                    if ASSIGNMENT_OPERATORS.contains(&&current_keyword[..]) {
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
                    params_left -= 1;
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

            if building_first_token {
                if self.current_char.is_whitespace() {
                    if !current_keyword.is_empty() {
                        match &current_keyword[..] {
                            ":func" => {
                                set_building!(Token::DefineFunc, 1);
                            }
                            ":tag" => {
                                set_building!(Token::Tag, 1);
                            }
                            ":endfunc" => {
                                no_args_add!(Token::EndFunc);
                            }
                            ":var" => {
                                set_building!(Token::Var, 3);
                            }
                            ":obj" => {
                                set_building!(Token::Objective, 2);
                            }
                            ":sobj" => {
                                set_building!(Token::SetObjective, 4);
                            }
                            ":call" => {
                                set_building!(Token::CallFunc, 1);
                            }
                            ":tvar" => {
                                set_building!(Token::TestVar, 1);
                            }
                            ":gvar" => {
                                set_building!(Token::GetVar, 1);
                            }
                            ":def" => {
                                set_building!(Token::DefineReplace, 2);
                            }
                            ":while" => {
                                tokens.push(Token::WhileLoop);
                                building_while = true;
                                building_while_condition = true;
                            }
                            ":sbop" => {
                                no_args_add!(Token::ScoreboardOperation);
                            }
                            ":delvar" | ":delobj" => {
                                set_building!(Token::DeleteVar, 1);
                            }
                            _ => {
                                no_args_add!(Token::NonDatabind(format!("{} ", current_keyword)));
                            }
                        };
                        if self.current_char == '\n' {
                            tokens.push(Token::NewLine);
                        }
                    } else {
                        self.next_char();
                    }
                } else {
                    current_keyword.push(self.current_char);
                    self.next_char();
                }
            } else {
                match building_for {
                    Token::Var => {
                        match params_left {
                            // Variable name
                            3 => {
                                add_token!(Token::ModVarName(current_keyword));
                            }
                            // Assignment operator
                            2 => {
                                assignment_operator!();
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
                    Token::Objective => match params_left {
                        2 => {
                            add_token!(Token::ObjectiveName(current_keyword));
                        }
                        _ => add_token_and_reset!(Token::ObjectiveType(current_keyword)),
                    },
                    Token::SetObjective => match params_left {
                        4 => {
                            add_token!(Token::Target(current_keyword));
                        }
                        3 => {
                            add_token!(Token::ObjectiveName(current_keyword));
                        }
                        2 => {
                            assignment_operator!();
                        }
                        _ => add_int_and_reset!(),
                    },
                    Token::Tag => add_token_and_reset!(Token::TagName(current_keyword)),
                    Token::DefineReplace => match params_left {
                        2 => {
                            add_token!(Token::ReplaceName(current_keyword));
                        }
                        _ => {
                            if ['\r', '\n'].contains(&self.current_char) {
                                tokens.push(Token::ReplaceContents(current_keyword));
                                building_for = Token::None;
                                current_keyword = String::new();
                            } else {
                                current_keyword.push(self.current_char);
                            }
                        }
                    },
                    Token::GetVar => add_token_and_reset!(Token::OpVarName(current_keyword)),
                    Token::DeleteVar => add_token_and_reset!(Token::DelVarName(current_keyword)),
                    _ => {}
                };
                if self.current_char == '\n' {
                    tokens.push(Token::NewLine);
                }
                self.next_char();
            }
        }

        tokens
    }
}
