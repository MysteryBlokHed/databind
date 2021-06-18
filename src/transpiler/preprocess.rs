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
use super::Transpiler;
use crate::settings::Settings;
use crate::token::Token;
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use std::collections::HashMap;

impl Transpiler<'_> {
    /// Get text replacement definitions and replace matches.
    /// Definitions that are not at the top of the file will be ignored
    /// and cause errors
    ///
    /// # Arguments
    ///
    /// - `content` - The contents of a file
    pub fn replace_definitions(contents: &String) -> String {
        let settings = &Settings::default();
        let mut transpiler = Transpiler::new(contents.clone(), settings, false);
        let mut new_contents = contents.clone();

        let replacement_tokens = transpiler.tokenize(true);

        let mut replacement_map: HashMap<String, String> = HashMap::new();
        let mut current_name = &String::new();

        for token in replacement_tokens.iter() {
            match token {
                Token::ReplaceName(name) => current_name = name,
                Token::ReplaceContents(contents) => {
                    replacement_map
                        .entry(current_name.to_owned())
                        .or_insert(contents.clone());
                }
                _ => {}
            }
        }

        // Replace text
        for (name, replacement) in replacement_map.iter() {
            new_contents = new_contents.replace(name, replacement);
        }

        // Remove :def lines
        let re = Regex::new(":def.*\n").unwrap();
        new_contents = re.replace(&new_contents[..], "").to_string();

        new_contents
    }

    /// Replace while loops with databind function definitions
    ///
    /// # Arguments
    ///
    /// - `tokens` - A list of tokens that may include while loops
    pub fn while_convert(&self, tokens: Vec<Token>) -> Vec<Token> {
        let mut new_tokens = tokens.clone();

        let mut while_index: usize = 0;
        let mut index_offset: usize = 0;
        let mut looping = true;
        let mut new_contents = String::new();
        let mut chars = Transpiler::get_chars();

        for i in 0..tokens.len() {
            let token = tokens.get(i).unwrap();
            if looping {
                match token {
                    Token::WhileCondition(condition) => new_contents.push_str(
                        &format!(
                            ":func while_{chars}\n\
                             execute if {condition} run :call condition_{chars}\n\
                             :endfunc\n",
                            chars = chars,
                            condition = condition
                        )[..],
                    ),
                    Token::WhileContents(contents) => new_contents.push_str(
                        &format!(
                            ":func condition_{chars}\n\
                             {contents}\n\
                             :call while_{chars}\n\
                             :endfunc\n",
                            chars = chars,
                            contents = contents
                        )[..],
                    ),
                    Token::EndWhileLoop => {
                        new_contents.push_str(&format!(":call while_{}\n", chars)[..]);
                        chars = Transpiler::get_chars();

                        // Tokenize new contents
                        let tks =
                            Transpiler::new(new_contents.clone(), &Settings::default(), false)
                                .tokenize(false);

                        // When gettings indexes in the new tokens vector,
                        // the length and position of elements will have changed
                        // due to new things being added
                        new_tokens.splice(
                            while_index + index_offset..while_index + index_offset + 4,
                            tks.iter().cloned(),
                        );
                        index_offset += tks.len() - 4;
                    }
                    _ => {}
                }
            }

            if token == &Token::WhileLoop {
                looping = true;
                while_index = i;
            }
        }

        new_tokens
    }

    /// Randomly generate characters
    fn get_chars() -> String {
        let chars = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect::<String>()
            .to_lowercase();
        chars
    }
}
