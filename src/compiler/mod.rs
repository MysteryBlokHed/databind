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

pub struct Compiler {
    chars: Vec<char>,
    position: usize,
    current_char: char,
    line: usize,
    col: usize,
}

impl Compiler {
    /// Create a new Compiler.
    ///
    /// # Arguments
    ///
    /// - `text` - The contents of the file to compile
    pub fn new(text: String) -> Compiler {
        let first_char = if !text.is_empty() {
            text.chars().next().unwrap()
        } else {
            '\u{0}'
        };

        Compiler {
            chars: text.chars().filter(|x| *x != '\r').collect(),
            position: 0,
            current_char: first_char,
            line: 1,
            col: 1,
        }
    }

    /// Go to the next character in the char list
    fn next_char(&mut self) {
        self.position += 1;
        if self.position < self.chars.len() {
            self.current_char = self.chars[self.position];
            if self.current_char == '\n' {
                self.line += 1;
                self.col = 0;
            }
            self.col += 1;
        } else {
            self.current_char = '\u{0}'
        }
    }

    pub fn make_syntax_error(&self, message: &str) -> String {
        format!("error: {}:{} - {}", self.line, self.col, message)
    }
}

mod compile;
pub mod macros;
mod preprocess;
mod tokenize;
