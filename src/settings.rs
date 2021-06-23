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
use serde::{Deserialize, Serialize};

/// Settings for the transpiler
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub random_var_names: bool,
    pub var_display_names: bool,
    pub inclusions: Vec<String>,
    pub exclusions: Vec<String>,
    pub output: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            random_var_names: false,
            var_display_names: false,
            inclusions: vec!["**/*.databind".into()],
            exclusions: Vec::new(),
            output: "out".into(),
        }
    }
}
