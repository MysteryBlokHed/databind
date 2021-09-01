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
//! Contains the Settings struct used to serialize/deserialize the databind.toml
//! configuration file
use serde::{Deserialize, Serialize};

/// Settings for the compiler
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// A list of globs of files to include in compilation
    pub inclusions: Vec<String>,
    /// A list of globs of files to exclude in compilation
    pub exclusions: Vec<String>,
    /// The output directory for compiled files
    pub output: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            inclusions: vec!["**/*.databind".into()],
            exclusions: Vec::new(),
            output: "out".into(),
        }
    }
}
