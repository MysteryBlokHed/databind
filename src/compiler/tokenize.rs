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

        let mut building_while = false;
        let mut building_condition = false;
        let mut statement_lines: Vec<String> = Vec::new();
        let mut statement_line = String::new();
        let mut escaped = false;
        let mut building_if = false;
        // Keep track of the amount of other statements inside the
        // statement being built
        // Avoids "end" tokens prematurely closing a loop
        let mut other_statements = 0;

        // Used to tokenize macro definitions
        let mut building_macro = false;
        let mut macro_name = String::new();
        let mut macro_name_built = false;
        let mut macro_args: Vec<String> = Vec::new();
        let mut marco_args_built = false;
        let mut building_macro_arg = false;
        let mut current_macro_arg = String::new();
        let mut macro_content = String::new();

        // Used to tokenize macro calls
        let mut calling_macro = false;
        let mut in_string = false;
        let mut current_string = String::new();
        let mut escaping_string = false;

        let mut building_for = Token::None;
        let mut params_left = 0;

        macro_rules! set_building {
            ($token: expr, $params: expr) => {{
                current_token = String::new();
                tokens.push($token);
                building_first_token = false;
                building_for = $token;
                params_left = $params;
                self.next_char();
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
                    escaped = false;
                    true
                } else {
                    // A % at the beginning escapes a keyword
                    if self.current_char != '%'
                        || self.current_char == '%' && (!current_token.is_empty() || escaped)
                    {
                        current_token.push(self.current_char);
                    } else {
                        escaped = true;
                    }
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
                    println!(
                        "{}",
                        self.make_syntax_error("Variables can only store integers")
                    );
                    std::process::exit(1);
                }
            };
        }

        /// Code to start building a macro
        macro_rules! building_macro {
            () => {
                building_first_token = false;
                calling_macro = true;
                macro_name = current_token.strip_prefix('?').unwrap().into();
                no_args_add!(Token::CallMacro);
                tokens.push(Token::MacroName(macro_name.clone()));
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
                        println!(
                            "{}",
                            self.make_syntax_error("Invalid assignment operator provided")
                        );
                        std::process::exit(1);
                    }
                    params_left -= 1;
                    current_token = String::new();
                } else {
                    current_token.push(self.current_char);
                }
            };
        }

        /// Build the condition of a statement
        ///
        /// # Arguments
        ///
        /// - `condition_tk` - The token (without parenthesis) that stores the condition
        /// of the statement
        macro_rules! build_statement_condition {
            ($condition_tk: expr) => {
                if self.current_char == '\n' {
                    add_token!($condition_tk(current_token.trim().into()));
                    building_condition = false;
                }
            };
        }

        /// Add the contents of a statement to a `Vec` of lines
        macro_rules! add_statement_lines {
            () => {
                statement_line.push(self.current_char);
                if self.current_char == '\n' {
                    current_token = String::new();
                    statement_lines.push(statement_line.trim().into());
                    statement_line = String::new();
                }
            };
        }

        /// End a statement and reset the variables used by it
        ///
        /// # Arguments
        ///
        /// - `contents_tk` - The token (without parenthesis) that stores the contents
        /// of the statement
        /// - `end_tk` - The token that marks the end of the statement
        macro_rules! statement_end {
            ($contents_tk: expr, $end_tk: expr) => {
                tokens.push($contents_tk(statement_lines.join("\n")));
                tokens.push($end_tk);
                current_token = String::new();
                statement_line = String::new();
                statement_lines = Vec::new();
                self.next_char();
            };
        }

        /// End a statement and reset the variables used by it, including the
        /// "building" variable (eg. `building_if`)
        ///
        /// # Arguments
        /// - `statement_bool` - The `bool` that controls whether a statement is
        /// currently being tokenized. Gets set to `false`
        /// - `contents_tk` - The token (without parenthesis) that stores the contents
        /// of the statement
        /// - `end_tk` - The token that marks the end of the statement
        macro_rules! statement_final {
            ($statement_bool: expr, $contents_tk: expr, $end_tk: expr) => {
                $statement_bool = false;
                building_first_token = true;
                statement_end!($contents_tk, $end_tk);
            };
        }

        while self.current_char != '\u{0}' {
            while comment {
                self.next_char();
                if self.current_char == '\n' {
                    comment = false;
                }
            }

            if building_macro {
                // Find the macro's name
                if !macro_name_built {
                    if self.current_char != '(' && !self.current_char.is_whitespace() {
                        macro_name.push(self.current_char);
                    } else if self.current_char == '(' {
                        tokens.push(Token::MacroName(macro_name));
                        macro_name = String::new();
                        macro_name_built = true;
                    }
                // Get the args the macro takes
                } else if !marco_args_built {
                    // A closing ) closes the arg list
                    if self.current_char == ')' {
                        marco_args_built = true;
                        macro_args.push(current_macro_arg);
                        tokens.push(Token::DefArgList(macro_args));
                        current_macro_arg = String::new();
                        macro_args = Vec::new();
                    // Make sure that arguments start with $
                    } else if !building_macro_arg {
                        if self.current_char.is_whitespace() {
                            self.next_char();
                            continue;
                        } else if self.current_char != '$' {
                            println!(
                                "{}",
                                self.make_syntax_error("Macro arguments must be preceded by a '$', eg. !def macro($arg1, $arg2)")
                            );
                            std::process::exit(1);
                        }
                        building_macro_arg = true;
                    // Add to the comma-separated list of macro args
                    } else if building_macro_arg {
                        if !self.current_char.is_whitespace() {
                            if self.current_char != ',' {
                                current_macro_arg.push(self.current_char);
                            } else {
                                macro_args.push(current_macro_arg);
                                current_macro_arg = String::new();
                                building_macro_arg = false;
                            }
                        }
                    }
                // The contents of the macro
                } else {
                    current_token.push(self.current_char);
                    if self.current_char.is_whitespace() {
                        if current_token.trim() == "!end" {
                            if other_statements == 0 {
                                tokens.push(Token::MacroContents(macro_content));
                                tokens.push(Token::EndMacro);
                                macro_content = String::new();
                                current_token = String::new();
                                building_macro = false;
                                macro_name_built = false;
                                marco_args_built = false;
                                building_macro_arg = false;
                                building_first_token = true;
                                self.next_char();
                            } else {
                                other_statements -= 1;
                                macro_content.push_str(&current_token);
                                current_token = String::new();
                            }
                        } else {
                            if current_token.trim() == "!def" {
                                other_statements += 1;
                            }
                            macro_content.push_str(&current_token);
                            current_token = String::new();
                        }
                    }
                }
            } else if calling_macro {
                if !building_macro_arg {
                    if self.current_char == '(' {
                        building_macro_arg = true;
                    } else if !self.current_char.is_whitespace() {
                        println!(
                            "{}",
                            self.make_syntax_error(&format!(
                                "'(' was expected to start the argument list for call of macro {}",
                                macro_name
                            ))
                        );
                        std::process::exit(1);
                    }
                } else {
                    if in_string {
                        if self.current_char == '\\' {
                            if escaping_string {
                                escaping_string = false;
                                current_string.push(self.current_char);
                            } else {
                                escaping_string = true;
                            }
                        } else if self.current_char == '"' && !escaping_string {
                            in_string = false;
                            macro_args.push(current_string);
                            current_string = String::new();
                            self.next_char();
                            continue;
                        } else {
                            current_string.push(self.current_char);
                            escaping_string = false;
                        }
                    } else {
                        if self.current_char == '"' {
                            in_string = true;
                        } else if self.current_char == ')' {
                            tokens.push(Token::CallArgList(macro_args));
                            macro_args = Vec::new();
                            calling_macro = false;
                            building_macro_arg = false;
                            building_first_token = true;
                            self.next_char();
                        } else if self.current_char != ',' && !self.current_char.is_whitespace() {
                            println!(
                                "{}",
                                self.make_syntax_error(&format!(
                                    "Unexpected character {:?} found in macro call",
                                    self.current_char
                                ))
                            );
                            std::process::exit(1);
                        }
                    }
                }
            } else if building_if {
                current_token.push(self.current_char);
                if building_condition {
                    build_statement_condition!(Token::IfCondition);
                } else if current_token.trim() == "else" && other_statements == 0 {
                    statement_end!(Token::IfContents, Token::ElseStatement);
                } else if current_token.trim() == "end" {
                    if other_statements == 0 {
                        statement_final!(building_if, Token::IfContents, Token::EndIf);
                    } else {
                        other_statements -= 1
                    };
                } else {
                    add_statement_lines!();
                }
                if current_token.trim() == "runif" || current_token.trim() == "while" {
                    other_statements += 1;
                }
            } else if building_while {
                current_token.push(self.current_char);

                if building_condition {
                    build_statement_condition!(Token::WhileCondition);
                } else if current_token.trim() == "end" {
                    if other_statements == 0 {
                        statement_final!(building_while, Token::WhileContents, Token::EndWhileLoop);
                    } else {
                        other_statements -= 1;
                    }
                } else {
                    add_statement_lines!();
                }
                if current_token.trim() == "runif" || current_token.trim() == "while" {
                    other_statements += 1;
                }
            }

            if building_first_token {
                // Macro call
                // Detects macros without a space before the args
                // eg. ?macro_name("arg")
                if self.current_char == '(' && current_token.starts_with('?') {
                    building_macro!();
                    continue;
                }
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
                            "!def" => {
                                no_args_add!(Token::DefineMacro);
                                building_macro = true;
                                building_first_token = false;
                                self.next_char();
                            }
                            "runif" => {
                                no_args_add!(Token::IfStatement);
                                building_first_token = false;
                                building_if = true;
                                building_condition = true;
                                self.next_char();
                            }
                            "while" => {
                                no_args_add!(Token::WhileLoop);
                                building_first_token = false;
                                building_while = true;
                                building_condition = true;
                                self.next_char();
                            }
                            "sbop" => no_args_add!(Token::NonDatabind(
                                "scoreboard players operation ".into()
                            )),
                            "delvar" | "delobj" => set_building!(Token::DeleteVar, 1),
                            _ => {
                                if current_token.starts_with('%')
                                    && !current_token.starts_with("%=")
                                {
                                    no_args_add!(Token::NonDatabind(format!(
                                        "{} ",
                                        current_token.strip_prefix('%').unwrap()
                                    )));
                                // Macro call
                                // This will detect macro calls with a space before the args
                                // eg. ?macro_name ("arg")
                                } else if current_token.starts_with('?') {
                                    building_macro!();
                                } else {
                                    no_args_add!(Token::NonDatabind(format!("{} ", current_token)));
                                }
                            }
                        };
                        if self.current_char == '\n' {
                            tokens.push(Token::NewLine);
                        }
                    } else {
                        self.next_char();
                    }
                } else {
                    if self.current_char == '#' && current_token.is_empty() {
                        comment = true;
                        continue;
                    }
                    current_token.push(self.current_char);
                    self.next_char();
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
                    Token::DeleteVar => add_token_and_reset!(Token::DelVarName(current_token)),
                    _ => {}
                };
                if self.current_char == '\n' && !building_if && !building_macro && !building_while {
                    tokens.push(Token::NewLine);
                }
                self.next_char();
            }
        }

        tokens
    }
}
