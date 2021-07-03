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
const ASSIGNMENT_OPERATORS: [&str; 4] = [":=", "=", "+=", "-="];

impl Compiler {
    /// Convert the provided file contents into a list of tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut comment = false;
        let mut building_first_token = true;
        let mut current_token = String::new();
        let mut last_char = '\n';

        let mut building_while = false;
        let mut building_while_condition = false;
        let mut while_lines: Vec<String> = Vec::new();
        let mut while_line = String::new();

        let mut building_macro = false;
        let mut macro_lines: Vec<String> = Vec::new();
        let mut macro_line = String::new();

        let mut building_for = Token::None;
        let mut params_left = 0;

        macro_rules! next_char {
            () => {
                last_char = self.current_char;
                self.next_char();
            };
        }

        macro_rules! set_building {
            ($token: expr, $params: expr) => {{
                current_token = String::new();
                tokens.push($token);
                building_first_token = false;
                building_for = $token;
                params_left = $params;
                next_char!();
            }};
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
                    current_token = String::new();
                    params_left -= 1;
                    true
                } else {
                    current_token.push(self.current_char);
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
            ($token: expr) => {{
                tokens.push($token);
                current_token = String::new();
            }};
        }

        /// Add an integer to the token list and reset variables
        macro_rules! add_int_and_reset {
            () => {
                if self.current_char.is_whitespace() {
                    let var_value: i32 = current_token.parse().unwrap();
                    add_token_and_reset!(Token::Int(var_value));
                } else if DIGITS.contains(&self.current_char) {
                    current_token.push(self.current_char);
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
                    if ASSIGNMENT_OPERATORS.contains(&&current_token[..]) {
                        match &current_token[..] {
                            ":=" => tokens.push(Token::InitialSet),
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
                    current_token = String::new();
                } else {
                    current_token.push(self.current_char);
                }
            };
        }

        while self.current_char != '\u{0}' {
            while comment {
                next_char!();
                if self.current_char == '\n' {
                    comment = false;
                }
            }

            #[allow(unused_assignments)]
            if building_macro {
                current_token.push(self.current_char);
                match params_left {
                    // Macro name
                    1 => {
                        if self.current_char.is_whitespace() {
                            add_token!(Token::MacroName(current_token));
                            params_left = 0;
                        }
                    }
                    // Macro arg name
                    2 => {
                        if self.current_char.is_whitespace() {
                            add_token!(Token::MacroArgName(current_token));
                            params_left = 0;
                        }
                    }
                    // Macro contents
                    _ => {
                        if self.current_char.is_whitespace() {
                            if self.current_char == '\n' {
                                macro_lines.push(macro_line);
                                macro_line = String::new();
                            } else if current_token == "!arg" {
                                params_left = 2;
                            } else if current_token == "!end" {
                                tokens.push(Token::MacroContents(macro_lines.join("\n")));
                                tokens.push(Token::EndMacro);
                                building_first_token = true;
                                building_macro = false;
                                macro_line = String::new();
                                macro_lines = Vec::new();
                            } else if !current_token.is_empty() {
                                macro_line.push_str(&format!("{} ", current_token));
                            }
                            current_token = String::new();
                        }
                    }
                }
            } else if building_while {
                current_token.push(self.current_char);
                if building_while_condition {
                    if self.current_char == '\n' {
                        add_token!(Token::WhileCondition(current_token.trim().into()));
                        building_while_condition = false;
                    }
                } else if current_token.trim() != "end" {
                    while_line.push(self.current_char);
                    if self.current_char == '\n' {
                        current_token = String::new();
                        while_lines.push(while_line.trim().into());
                        while_line = String::new();
                    }
                } else {
                    building_while = false;
                    building_first_token = true;
                    tokens.push(Token::WhileContents(while_lines.join("\n")));
                    tokens.push(Token::EndWhileLoop);
                    current_token = String::new();
                    while_line = String::new();
                    while_lines = Vec::new();
                    self.next_char();
                }
            }

            if building_first_token {
                if self.current_char.is_whitespace() {
                    if !current_token.is_empty() {
                        match &current_token[..] {
                            "func" => set_building!(Token::DefineFunc, 1),
                            "tag" => set_building!(Token::Tag, 1),
                            "end" => no_args_add!(Token::EndFunc),
                            "var" => set_building!(Token::Var, 3),
                            "obj" => set_building!(Token::Objective, 2),
                            "sobj" => set_building!(Token::SetObjective, 4),
                            "call" => set_building!(Token::CallFunc, 1),
                            "tvar" => set_building!(Token::TestVar, 1),
                            "gvar" => set_building!(Token::GetVar, 1),
                            "!macro" => {
                                // Uses the `params_left` variable to track what's being built
                                // (eg. name of macro or an argument's name)
                                set_building!(Token::DefineMacro, 1);
                                building_macro = true;
                            }
                            "while" => {
                                no_args_add!(Token::WhileLoop);
                                building_first_token = false;
                                building_while = true;
                                building_while_condition = true;
                                self.next_char();
                            }
                            "sbop" => no_args_add!(Token::ScoreboardOperation),
                            "delvar" | "delobj" => set_building!(Token::DeleteVar, 1),
                            _ => {
                                if current_token.starts_with('?') {
                                    set_building!(Token::CallMacro, -1);
                                } else {
                                    no_args_add!(Token::NonDatabind(format!("{} ", current_token)));
                                }
                            }
                        };
                        if self.current_char == '\n' {
                            tokens.push(Token::NewLine);
                        }
                    } else {
                        next_char!();
                    }
                } else {
                    if last_char == '\n' && self.current_char == '#' {
                        comment = true;
                        continue;
                    }
                    current_token.push(self.current_char);
                    next_char!();
                }
            } else {
                match building_for {
                    Token::Var => {
                        match params_left {
                            // Variable name
                            3 => {
                                add_token!(Token::ModVarName(current_token));
                            }
                            // Assignment operator
                            2 => assignment_operator!(),
                            // Value
                            _ => add_int_and_reset!(),
                        }
                    }
                    Token::TestVar => {
                        add_token_and_reset!(Token::TestVarName(current_token));
                    }
                    Token::DefineFunc | Token::CallFunc => {
                        add_token_and_reset!(Token::FuncName(current_token));
                    }
                    Token::Objective => match params_left {
                        2 => {
                            add_token!(Token::ObjectiveName(current_token));
                        }
                        _ => add_token_and_reset!(Token::ObjectiveType(current_token)),
                    },
                    Token::SetObjective => match params_left {
                        4 => {
                            add_token!(Token::Target(current_token));
                        }
                        3 => {
                            add_token!(Token::ObjectiveName(current_token));
                        }
                        2 => assignment_operator!(),
                        _ => add_int_and_reset!(),
                    },
                    Token::Tag => add_token_and_reset!(Token::TagName(current_token)),
                    Token::GetVar => add_token_and_reset!(Token::OpVarName(current_token)),
                    Token::CallMacro => match params_left {
                        -1 => {
                            add_token!(Token::MacroName(current_token));
                        }
                        0 => {
                            building_for = Token::None;
                            building_first_token = true;
                        }
                        _ => {
                            add_token!(Token::MacroArgName(current_token));
                        }
                    },
                    Token::DeleteVar => add_token_and_reset!(Token::DelVarName(current_token)),
                    _ => {}
                };
                if self.current_char == '\n' {
                    tokens.push(Token::NewLine);
                }
                next_char!();
            }
        }

        tokens
    }
}
