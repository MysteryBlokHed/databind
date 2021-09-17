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
//! Contains functions and structs used to tokenize and compile Databind
//! source code
use crate::{files, token::Token, types::GlobalMacros};
use compile::CompileReturn;
use std::path::Path;

/// Used to tokenize and compile Databind source code
pub struct Compiler {
    /// The characters in a source file
    chars: Vec<char>,
    /// The current index in the characters vector
    position: usize,
    /// The current character
    current_char: char,
    /// The current line number (Used for errors)
    line: usize,
    /// The current column number (Used for errors)
    col: usize,
    /// The path of the file being compiled (Used for errors)
    path: String,
}

impl Compiler {
    /// Create a new Compiler.
    ///
    /// # Arguments
    ///
    /// - `text` - The contents of the file to compile
    /// - `path` - The path of the file being compiled. Used for error messages
    pub fn new(text: String, path: Option<String>) -> Compiler {
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
            path: if let Some(p) = path {
                p
            } else {
                "INTERNAL".into()
            },
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
            } else {
                self.col += 1;
            }
        } else {
            self.current_char = '\u{0}'
        }
    }

    pub fn make_syntax_error(&self, message: &str) -> String {
        format!(
            "error: {}:{}:{} - {}",
            self.path, self.line, self.col, message
        )
    }

    /// Pass different arguments to `Compiler::compile` depending on whether the source file
    /// contains global macros or not and return the result
    ///
    /// # Arguments
    ///
    /// - `tokens` - The vector of tokens to pass
    /// - `target_filename` - The filename of the tokenized file (used to check for `!` at beginning)
    /// - `functions_dir` - The functions/ directory containing the tokenized file
    ///   (used to get namespace & subfolder)
    /// - `global_macros` - The global macros map to pass and to update if it needs to be
    pub fn compile_check_macro<P: AsRef<Path>>(
        &self,
        tokens: Vec<Token>,
        target_filename: &str,
        functions_dir: P,
        global_macros: &mut GlobalMacros,
    ) -> CompileReturn {
        if target_filename.starts_with('!') {
            let ret = self.compile(
                tokens,
                Some(&files::get_namespace(&functions_dir).unwrap()),
                &files::get_subfolder_prefix(&functions_dir),
                &global_macros,
                true,
            );
            global_macros.extend(ret.global_macros.clone().unwrap());
            ret.clone()
        } else {
            self.compile(
                tokens,
                Some(&files::get_namespace(&functions_dir).unwrap()),
                &files::get_subfolder_prefix(&functions_dir),
                &global_macros,
                false,
            )
        }
    }
}

mod compile;
mod macros;
mod preprocess;
mod tokenize;
pub use macros::Macro;
