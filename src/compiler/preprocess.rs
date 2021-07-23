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
use super::macros::Macro;
use super::Compiler;
use crate::token::Token;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;

/// Whether a function to set up an if statement objective has been created
static mut IF_INIT_CREATED: bool = false;
/// The objective used to store the results of if condition tests
static mut IF_INIT_OBJ: String = String::new();

impl Compiler {
    /// Replace macro calls with the contents of a macro.
    /// Recursively calls itself until no macro calls are left
    /// in case a macro definition contains a macro call
    ///
    /// This function leaves macro definition tokens in the token list
    /// as they are ignored by the compiler
    ///
    /// # Arguments
    ///
    /// - `tokens` - A list of tokens to look for macro calls in
    /// - `return_macros` - Whether to return the HashMap of macros.
    ///   Used for global macros (files beginning with `@`)
    /// - `existing_macros` - Global macros to use
    pub fn parse_macros(
        &self,
        tokens: Vec<Token>,
        return_macros: bool,
        existing_macros: &HashMap<String, Macro>,
    ) -> (Vec<Token>, Option<HashMap<String, Macro>>) {
        let mut new_tokens = tokens.clone();

        let mut macros: HashMap<String, Macro> = existing_macros.clone();
        let mut call_index: usize = 0;
        let mut index_offset: usize = 0;

        let mut active_macro_name = String::new();
        let mut macro_def_args: Vec<String> = Vec::new();

        for i in 0..tokens.len() {
            let token = tokens.get(i).unwrap();
            match token {
                Token::CallMacro => call_index = i,
                Token::MacroName(name) => active_macro_name = name.clone(),
                Token::DefArgList(args) => {
                    macro_def_args = args.clone();
                }
                Token::CallArgList(args) => {
                    if !macros.contains_key(&active_macro_name) {
                        println!("A non-existant macro {} was called", active_macro_name);
                        std::process::exit(1);
                    }

                    let tks = {
                        let new_contents = macros[&active_macro_name].replace(args);
                        Compiler::new(new_contents.clone()).tokenize()
                    };

                    // Remove the tokens CallMacro, MacroName, and CallArgList
                    let tks_len = tks.len();
                    new_tokens.splice(
                        call_index + index_offset..call_index + index_offset + 3,
                        tks,
                    );
                    index_offset += tks_len - 3;
                }
                Token::MacroContents(contents) => {
                    macros.insert(
                        active_macro_name,
                        Macro::new(macro_def_args, contents.clone()),
                    );
                    active_macro_name = String::new();
                    macro_def_args = Vec::new();
                }
                _ => {}
            }
        }

        if new_tokens.contains(&Token::CallMacro) {
            self.parse_macros(new_tokens, return_macros, existing_macros)
        } else if return_macros {
            (new_tokens, Some(macros))
        } else {
            (new_tokens, None)
        }
    }

    /// Replace while loops, if statements, and scoreboard operations.
    /// Called recursively until none are left
    ///
    /// # Arguments
    ///
    /// - `tokens` - A list of tokens to look for while loops or `sbop`'s in
    /// - `subfolder` - If the while loop is in a subfolder, the prefix
    ///   to put before the function name (eg. `"cmd/"` for a subfolder named `cmd`)
    pub fn parse_shorthand(&self, tokens: Vec<Token>, subfolder: &str) -> Vec<Token> {
        let mut new_tokens = tokens.clone();

        let mut active_index: usize = 0;
        let mut index_offset: usize = 0;
        let mut new_contents = String::new();
        let mut chars = Compiler::random_chars();
        // Whether the IfContents token is referring to an else
        let mut on_else = false;

        /// Replace tokens such as while loops or if statements with their
        /// compiled versions
        macro_rules! replace_tokens {
            ($tks_length: expr) => {
                chars = Compiler::random_chars();

                // Tokenize new contents
                let tks = Compiler::new(new_contents.clone()).tokenize();
                new_contents = String::new();

                // When gettings indexes in the new tokens vector,
                // the length and position of elements will have changed
                // due to new things being added
                new_tokens.splice(
                    active_index + index_offset..active_index + index_offset + $tks_length,
                    tks.iter().cloned(),
                );
                index_offset += tks.len() - $tks_length;
            };
        }

        for i in 0..tokens.len() {
            let token = tokens.get(i).unwrap();
            match token {
                Token::WhileLoop | Token::IfStatement => active_index = i,
                Token::WhileCondition(condition) => new_contents.push_str(
                    &format!(
                        "func while_{chars}\n\
                             execute if {condition} run call {subfolder}condition_{chars}\n\
                         end\n",
                        chars = chars,
                        condition = condition,
                        subfolder = subfolder
                    )[..],
                ),
                Token::ElseStatement => on_else = true,
                Token::WhileContents(contents) => new_contents.push_str(&format!(
                    "func condition_{chars}\n\
                         {contents}\n\
                         call {subfolder}while_{chars}\n\
                     end\n",
                    chars = chars,
                    contents = contents,
                    subfolder = subfolder
                )),
                Token::EndWhileLoop | Token::EndIf => {
                    if token == &Token::EndWhileLoop {
                        new_contents.push_str(&format!("call {}while_{}\n", subfolder, chars)[..]);
                    }
                    replace_tokens!(4);
                }
                Token::IfCondition(condition) => {
                    // Unsafe due to check of IF_INIT_CREATED
                    unsafe {
                        if !IF_INIT_CREATED {
                            let chars = Compiler::random_chars();
                            IF_INIT_OBJ = format!("if_init_{}", chars);
                            new_contents.push_str(&format!(
                                "func {obj}\n\
                                     obj {obj} dummy\n\
                                 end\n",
                                obj = IF_INIT_OBJ,
                            ));
                            IF_INIT_CREATED = true
                        }
                    }

                    unsafe {
                        new_contents.push_str(&format!(
                            "%# If statement\n\
                             execute if {condition} run sobj {obj} --databind-{chars} = 1\n\
                             execute unless {condition} run sobj {obj} --databind-{chars} = 0\n\
                             execute if score --databind-{chars} {obj} matches 1 run call {subfolder}if_true_{chars}\n",
                            condition = condition,
                            obj = IF_INIT_OBJ,
                            chars = chars,
                            subfolder = subfolder,
                        ));
                    }
                }
                Token::IfContents(contents) => {
                    if on_else {
                        unsafe {
                            new_contents.push_str(&format!(
                                "func if_false_{chars}\n\
                                     {}\n\
                                 end\n\
                                 execute if score --databind-{chars} {} matches 0 run call {}if_false_{chars}\n",
                                contents,
                                IF_INIT_OBJ,
                                subfolder,
                                chars = chars,
                            ));
                        }
                    } else {
                        new_contents.push_str(&format!(
                            "func if_true_{}\n\
                                {}\n\
                            end\n",
                            chars, contents
                        ))
                    }
                }
                Token::ScoreboardOperation => {
                    new_tokens[i + index_offset] =
                        Token::NonDatabind("scoreboard players operation ".into());
                }
                _ => {}
            }
        }

        // Run recursively until no if statements, while loops,
        // or scoreboard operations are left
        if new_tokens.iter().any(|x| {
            [
                Token::IfStatement,
                Token::WhileLoop,
                Token::ScoreboardOperation,
            ]
            .contains(&x)
        }) {
            self.parse_shorthand(new_tokens, subfolder)
        } else {
            new_tokens
        }
    }

    /// Return a random string of 4 lowercase alphanumeric characters
    fn random_chars() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect::<String>()
            .to_lowercase()
    }
}
