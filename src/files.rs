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
//! Contains functions used to get information from filepaths for the Databind CLI.
use glob::glob;
use std::path::{Path, PathBuf};

/// Get the prefix of a subfolder before a function call (eg. `"cmd/"` for
/// a subfolder called `cmd`)
/// Used for Databind calls, so a line like `call func` in a subfolder `cmd`
/// will become `function namespace:cmd/func`
pub fn get_subfolder_prefix<P: AsRef<Path>>(functions_path: &P) -> String {
    let mut after_functions = PathBuf::from(
        functions_path
            .as_ref()
            .to_str()
            .unwrap()
            .split("functions")
            .last()
            .unwrap(),
    );

    after_functions.pop();

    // Ensure no backslashes and remove leading slash, if present
    let prefix = after_functions.to_str().unwrap().replace('\\', "/");

    if prefix == "/" {
        String::new()
    } else if let Some(new) = prefix.strip_prefix('/') {
        format!("{}/", new)
    } else {
        format!("{}/", prefix)
    }
}

/// Get namespace (name of folder containing the main /functions)
pub fn get_namespace<P: AsRef<Path>>(functions_path: &P) -> Result<&str, &str> {
    let namespace_folder = functions_path
        .as_ref()
        .to_str()
        .unwrap()
        .split("functions")
        .next()
        .unwrap();

    let namespace_folder =
        if let Some(new) = namespace_folder.strip_suffix(|x: char| ['\\', '/'].contains(&x)) {
            new
        } else {
            namespace_folder
        };

    let folders = namespace_folder.split(|x: char| ['\\', '/'].contains(&x));
    Ok(folders.last().unwrap())
}

/// Convert multiple globs into a `Vec<PathBuf>`
pub fn merge_globs(globs: &[String], prefix: &str) -> Vec<PathBuf> {
    let mut merged_globs: Vec<PathBuf> = Vec::new();

    for files_glob in globs.iter() {
        let relative_files_glob = format!("{}/{}", prefix, files_glob);

        let mut files: Vec<PathBuf> = glob(&relative_files_glob)
            .expect(&format!("Failed to parse glob {}", files_glob)[..])
            .filter_map(Result::ok)
            .collect();
        merged_globs.append(&mut files);
    }

    merged_globs
}

/// Try to find a config file
///
/// # Returns
///
/// Either the path to the config file or an error.
pub fn find_config_in_parents(
    start: &dyn AsRef<Path>,
    config_file: String,
) -> Result<PathBuf, &str> {
    let mut start = PathBuf::from(start.as_ref());
    let mut last = PathBuf::new();

    while start != last {
        start.push(&config_file);
        if start.exists() && start.is_file() {
            return Ok(start);
        }
        start.pop();
        last = start.clone();
        start.pop();
    }

    Err("Did not find databind.toml in parents")
}
