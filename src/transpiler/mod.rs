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
use crate::settings::Settings;

pub struct Transpiler<'a> {
    chars: Vec<char>,
    position: usize,
    current_char: char,
    settings: &'a Settings,
}

impl Transpiler<'_> {
    /// Create a new Transpiler.
    ///
    /// # Arguments
    ///
    /// - `text` - The contents of the file to transpile
    /// - `settings` - The settings for the transpiler
    /// - `replacement` - Whether to replace :def's. Required to avoid stack overflow
    pub fn new(text: String, settings: &Settings, replacement: bool) -> Transpiler<'_> {
        let text = if replacement {
            Transpiler::replace_definitions(&text)
        } else {
            text
        };

        let first_char = if !text.is_empty() {
            text.chars().next().unwrap()
        } else {
            '\u{0}'
        };

        Transpiler {
            chars: text.chars().collect(),
            position: 0,
            current_char: first_char,
            settings,
        }
    }

    /// Go to the next character in the char list
    fn next_char(&mut self) {
        self.position += 1;
        if self.position < self.chars.len() {
            self.current_char = self.chars[self.position];
        } else {
            self.current_char = '\u{0}'
        }
    }
}

mod preprocess;
mod tokenize;
mod transpile;
