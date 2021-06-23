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
#![warn(clippy::all)]

use glob::glob;
use same_file::is_same_file;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod cli;
mod create_project;
mod settings;
mod token;
mod transpiler;

#[derive(Serialize)]
struct TagFile<'a> {
    values: &'a Vec<String>,
}

/// Get namespace (name of folder containing `/functions`)
fn get_namespace(functions_path: &Path) -> &str {
    let namespace_folder = functions_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap();

    let folders = namespace_folder.split(&['/', '\\'][..]);

    folders.last().unwrap()
}

/// Convert multiple globs into a `Vec<PathBuf>`
fn merge_globs(globs: &Vec<String>, prefix: &str) -> Vec<PathBuf> {
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

/// The main function
///
/// Transpiles provided files and folders to normal `.mcfunction` files
fn main() -> std::io::Result<()> {
    let matches = cli::get_cli_matches();

    // Check if create command is used
    if let Some(subcommand) = matches.subcommand {
        return create_project::create_project(subcommand.matches);
    }

    let config_path_str: String;

    if matches.is_present("config") {
        config_path_str = matches.value_of("config").unwrap().to_string();

        if fs::metadata(&config_path_str).is_err() {
            println!("Non-existant config file specified.");
            std::process::exit(1);
        }
    } else {
        config_path_str = String::from("databind.toml");
    }

    let config_path = Path::new(&config_path_str);
    if config_path.is_dir() {
        println!("Directory provided for config file.");
        std::process::exit(1);
    }

    let mut transpiler_settings: settings::Settings;

    if config_path.exists() && !matches.is_present("ignore-config") {
        let config_contents = fs::read_to_string(&config_path)?;
        transpiler_settings = toml::from_str(&config_contents[..]).unwrap();
    } else {
        transpiler_settings = settings::Settings::default();
    }

    // Override config settings with CLI arguments if passed
    if matches.is_present("random-var-names") {
        transpiler_settings.random_var_names = true;
    }
    if matches.is_present("var-display-names") {
        transpiler_settings.var_display_names = true;
    }
    if matches.is_present("output") {
        transpiler_settings.output = Some(matches.value_of("output").unwrap().to_string());
    }

    let datapack = matches.value_of("DATAPACK").unwrap();
    let datapack_is_dir = fs::metadata(datapack)?.is_dir();

    if datapack_is_dir {
        let mut var_map: HashMap<String, String> = HashMap::new();
        let mut tag_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut target_folder: String;

        if let Some(output) = &transpiler_settings.output {
            target_folder = output.clone();
        } else {
            target_folder = datapack.to_string();
            target_folder = target_folder
                .trim_end_matches('/')
                .trim_end_matches('\\')
                .to_string();
            target_folder.push_str(".databind");
        }

        if fs::metadata(&target_folder).is_ok() {
            println!("Deleting old databind folder...");
            fs::remove_dir_all(&target_folder)?;
            println!("Done.");
        }

        let mut inclusions = merge_globs(&transpiler_settings.inclusions, datapack);
        let exclusions = merge_globs(&transpiler_settings.exclusions, datapack);
        inclusions = inclusions
            .iter()
            .filter(|&x| !exclusions.contains(x))
            .cloned()
            .collect();

        for entry in WalkDir::new(&datapack).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                // Do not add config file to output folder
                if config_path.exists() && is_same_file(entry.path(), config_path).unwrap() {
                    continue;
                }

                let new_path_str = entry.path().to_str().unwrap().replacen(datapack, "", 1);
                let path = Path::new(&new_path_str);

                let mut target_path: String = target_folder.to_string();
                target_path.push('/');
                target_path.push_str(&format!("{}", path.parent().unwrap().to_str().unwrap())[..]);

                fs::create_dir_all(&target_path)?;

                let mut transpile = false;
                let mut continue_loop = false;

                for file in inclusions.iter() {
                    if is_same_file(file, entry.path()).expect("Failed to check file paths") {
                        transpile = true;
                        break;
                    }
                }

                for file in exclusions.iter() {
                    if is_same_file(file, entry.path()).expect("Failed to check file paths") {
                        continue_loop = true;
                        break;
                    }
                }

                if continue_loop {
                    continue;
                }

                if transpile {
                    let content = fs::read_to_string(entry.path())
                        .expect(&format!("Failed to read file {}", entry.path().display())[..]);
                    let mut transpile =
                        transpiler::Transpiler::new(content, &transpiler_settings, true);
                    let tokens = transpile.tokenize(false);
                    let mut transpiled = transpile.transpile(
                        tokens,
                        Some(get_namespace(entry.path())),
                        Some(&var_map),
                    );

                    var_map = transpiled.var_map;

                    for (key, value) in transpiled.filename_map.iter() {
                        let full_path = format!("{}/{}.mcfunction", target_path, key);

                        fs::write(full_path, &transpiled.file_contents[*value])?;

                        // Add namespace prefix to function in tag map
                        for (_, funcs) in transpiled.tag_map.iter_mut() {
                            if funcs.contains(key) {
                                let i = funcs.iter().position(|x| x == key).unwrap();
                                funcs[i] = format!("{}:{}", get_namespace(entry.path()), key);
                            }
                        }
                    }

                    tag_map.extend(transpiled.tag_map);
                } else {
                    let filename = path.file_name().unwrap().to_str().unwrap();
                    let full_path = format!("{}/{}", target_path, filename);
                    fs::copy(entry.path(), full_path)?;
                }
            }
        }

        // Write tag files
        fs::create_dir_all(format!("{}/data/minecraft/tags/functions", target_folder))?;

        for (tag, funcs) in tag_map.iter() {
            let tag_file = TagFile { values: funcs };
            let json = serde_json::to_string(&tag_file)?;

            // Write tag file
            fs::write(
                format!(
                    "{}/data/minecraft/tags/functions/{}.json",
                    target_folder, tag
                ),
                json,
            )?;
        }
    } else {
        println!("Databind does not support single-file compilation.");
        std::process::exit(1);
    }

    Ok(())
}
