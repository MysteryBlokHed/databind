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
/// Used to define and expand Databind macros
#[allow(dead_code)]
pub struct Macro {
    name: String,
    arg_names: Vec<String>,
    content: String,
}

#[allow(dead_code)]
impl Macro {
    /// Define a new macro
    ///
    /// # Arguments
    ///
    /// - `name` - The name of the macro when being called. Similar to a function name
    /// - `arg_names` - A list of named arguments for the macro. Can be empty
    /// - `content` - The content of the macro itself
    pub fn new(name: String, arg_names: Vec<String>, content: String) -> Macro {
        Macro {
            name,
            arg_names,
            content,
        }
    }

    /// Expand a macro to regular Databind code
    ///
    /// # Arguments
    ///
    /// - `args` - The args to pass to the macro. Should be in the same order as the
    ///   `arg_names` were.
    pub fn expand(&self, args: Vec<String>) -> String {
        if args.len() != self.arg_names.len() {
            panic!("Invalid amount of args supplied for macro");
        }
        let mut expanded = self.content.clone();
        for (i, item) in args.iter().enumerate() {
            expanded = expanded.replace(&format!("${}", self.arg_names[i]), item);
        }

        expanded
    }
}
