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
//! Contains structs and functions used to define and replace macros in
//! Databind source code
use std::collections::HashMap;

/// A definition of a Databind macrco
#[derive(Debug, Clone)]
pub struct Macro {
    /// The names of the macro arguments
    arg_names: Vec<String>,
    /// The contents of the macro
    content: String,
}

impl Macro {
    /// Create a new Databind macro
    ///
    /// # Arguments
    ///
    /// - `arg_names` - The names of the arguments in the macro
    /// definition. Should be prefixed by `$` in the definition,
    /// but not in this argument (eg. `$arg1` should be just `arg1`)
    /// - `content` - The content of the macro definition
    pub fn new(arg_names: Vec<String>, content: String) -> Macro {
        Macro { arg_names, content }
    }

    /// Returns the text to replace a macro call with
    pub fn replace(&self, args: &Vec<String>) -> String {
        let mut replacements: HashMap<String, String> = HashMap::new();
        for (i, value) in args.iter().enumerate() {
            replacements.insert(format!("${}", self.arg_names[i].clone()), value.clone());
        }

        let mut replaced_content = self.content.clone();

        for (arg, value) in replacements.iter() {
            replaced_content = replaced_content.replace(arg, value);
        }

        replaced_content
    }
}
