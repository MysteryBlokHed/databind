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
//! Contains functions used by the CLI to get information from files or to
//! create files
use crate::types::TagMap;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use toml::Value;
use walkdir::WalkDir;

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

/// Creates tag files for compiled Databind code. Also checks if tags already
/// exist and merges them
///
/// # Arguments
///
/// - `src_dir` - The directory of the Databind source files
/// - `target_folder` - The output directory for compiled files
/// - `tag_map` - A map of tags to a vector of functions with that tag
///   (eg. `{"load": ["namespace:main"]}`)
pub fn create_tag_files<P: AsRef<Path>>(
    src_dir: P,
    target_folder: P,
    tag_map: &TagMap,
) -> std::io::Result<()> {
    #[derive(Deserialize, Serialize)]
    struct TagFile {
        values: Vec<String>,
    }

    let target = target_folder.as_ref().display();
    // Create tags directory
    fs::create_dir_all(format!("{}/data/minecraft/tags/functions", target))?;

    for (tag, funcs) in tag_map.iter() {
        let mut tag_file = TagFile {
            values: funcs.clone(),
        };

        {
            // Path to potential source JSON file
            let path_str = format!(
                "{}/data/minecraft/tags/functions/{}.json",
                src_dir.as_ref().display(),
                tag
            );
            let path = Path::new(&path_str);

            // Read existing tags if present
            if path.exists() && path.is_file() {
                let contents = fs::read_to_string(&path)?;
                let mut existing_tags: TagFile = serde_json::from_str(&contents)?;
                tag_file.values.append(&mut existing_tags.values);
            }
        }

        let json = serde_json::to_string(&tag_file)?;

        // Write tag file
        fs::write(
            &format!("{}/data/minecraft/tags/functions/{}.json", target, tag),
            json,
        )?;
    }

    Ok(())
}

/// Returns a vector of source files with files beginning with ! appearing first.
/// This is used to ensure that files with global macros are ordered the same
/// across platforms
///
/// # Arguments
///
/// - `src_dir` - The directory that contains the Databind source files
pub fn prioritize_macro_files<P: AsRef<Path>>(src_dir: P) -> Vec<PathBuf> {
    // Store global macro filepaths
    let mut global_macros: Vec<PathBuf> = Vec::new();
    // Store normal filepaths
    let mut normal: Vec<PathBuf> = Vec::new();
    // Sort filepaths into each vector
    let unsorted_paths = WalkDir::new(src_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.path().to_path_buf());

    for path in unsorted_paths {
        if path.file_name().unwrap().to_str().unwrap().starts_with('!') {
            global_macros.push(path);
        } else {
            normal.push(path);
        }
    }

    global_macros.append(&mut normal);
    global_macros
}

/// Read the vars.toml file into a HashMap of Strings
pub fn read_vars_toml<P: AsRef<Path>>(vars_toml: P) -> HashMap<String, String> {
    let contents = fs::read_to_string(vars_toml).unwrap();
    // Read toml file into HashMap with multiple types
    let vars_multi_type: HashMap<String, Value> = toml::from_str(&contents).unwrap();
    let mut vars: HashMap<String, String> = HashMap::new();
    for (k, v) in vars_multi_type.iter() {
        // Try to convert the value into a string
        let new_v: String = match v {
            Value::String(value) => value.clone(),
            Value::Boolean(value) => {
                if *value {
                    "1".into()
                } else {
                    "0".into()
                }
            }
            Value::Float(value) => value.to_string(),
            Value::Integer(value) => value.to_string(),
            Value::Datetime(value) => value.to_string(),
            _ => {
                println!(
                    "error: Unsupported type found in vars.toml file (key: {}, value: {})",
                    k, v
                );
                std::process::exit(1);
            }
        };
        vars.entry(format!("&{}", k)).or_insert(new_v);
    }
    vars
}
