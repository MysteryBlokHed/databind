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

use databind::{
    compiler::{macros::Macro, Compiler},
    files, Settings,
};
use pest::error::LineColLocation;
use same_file::is_same_file;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

mod cli;
mod create_project;

/// The main function
///
/// Compiles provided files and folders to normal `.mcfunction` files
fn main() -> std::io::Result<()> {
    let matches = cli::get_matches();

    // Check if create command is used
    if let Some(subcommand) = matches.subcommand {
        return create_project::create_project(subcommand.matches);
    }

    let datapack = matches.value_of("project").unwrap();
    let datapack_is_dir = fs::metadata(datapack)?.is_dir();

    let config_path_str: String;

    if matches.is_present("config") {
        config_path_str = matches.value_of("config").unwrap().to_string();

        if fs::metadata(&config_path_str).is_err() {
            eprintln!("Non-existant config file specified.");
            std::process::exit(1);
        }
    } else {
        // Look for databind.toml in target folder
        let potential_path = format!("{}/databind.toml", datapack);
        if fs::metadata(&potential_path).is_ok() {
            config_path_str = potential_path;
        } else {
            config_path_str = String::new();
        }
    }

    let config_path = Path::new(&config_path_str);
    if config_path.is_dir() {
        eprintln!("Directory provided for config file.");
        std::process::exit(1);
    }

    let mut compiler_settings: Settings;
    if config_path.exists() && !matches.is_present("ignore-config") {
        let config_contents = fs::read_to_string(&config_path)?;
        compiler_settings = toml::from_str(&config_contents[..]).unwrap();
        compiler_settings.output = format!("{}/{}", datapack, compiler_settings.output);
        let cli_out = matches.value_of("output").unwrap();
        if cli_out != "out" {
            compiler_settings.output = cli_out.into();
        }
    } else {
        compiler_settings = Settings::default();
        compiler_settings.output = matches.value_of("output").unwrap().into();
    }

    if datapack_is_dir {
        let mut tag_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut macros: HashMap<String, Macro> = HashMap::new();
        let target_folder = &compiler_settings.output;

        if fs::metadata(target_folder).is_ok() {
            fs::remove_dir_all(&target_folder)?;
        }

        let mut inclusions = files::merge_globs(&compiler_settings.inclusions, datapack);
        let exclusions = files::merge_globs(&compiler_settings.exclusions, datapack);
        inclusions = inclusions
            .iter()
            .filter(|&x| !exclusions.contains(x))
            .cloned()
            .collect();

        let src_dir = PathBuf::from(format!("{}/src", datapack));
        let src_dir = if !src_dir.exists() || !src_dir.is_dir() {
            Path::new(&datapack)
        } else {
            src_dir.as_path()
        };

        // Read vars.toml if present
        let vars = {
            // Path to toml file
            let path = format!("{}/vars.toml", datapack);
            let vars_toml = Path::new(&path);
            // Check if file exists
            if vars_toml.exists() && vars_toml.is_file() {
                Some(files::read_vars_toml(&vars_toml))
            } else {
                None
            }
        };

        // Get filepaths with global macros appearing first
        let paths = files::prioritize_macro_files(src_dir);

        for path in paths.iter() {
            // Do not add config file to output folder
            if config_path.exists() && is_same_file(path, config_path).unwrap() {
                continue;
            }

            let new_path_str = path
                .to_str()
                .unwrap()
                .replacen(src_dir.to_str().unwrap(), "", 1);

            let relative_path = Path::new(&new_path_str);

            let mut target_path: String = target_folder.to_string();
            target_path.push('/');
            target_path.push_str(relative_path.parent().unwrap().to_str().unwrap());

            fs::create_dir_all(&target_path)?;

            let mut compile = false;
            let mut continue_loop = false;

            for file in inclusions.iter() {
                if is_same_file(file, path).expect("Failed to check file paths") {
                    compile = true;
                    break;
                }
            }

            for file in exclusions.iter() {
                if is_same_file(file, path).expect("Failed to check file paths") {
                    continue_loop = true;
                    break;
                }
            }

            if continue_loop {
                continue;
            }

            if compile {
                let subfolder = files::get_subfolder_prefix(&path);
                let file_contents = {
                    let mut file_contents = fs::read_to_string(path)
                        .expect(&format!("Failed to read file {}", path.display())[..]);
                    if let Some(vars_map) = &vars {
                        for (k, v) in vars_map.iter() {
                            file_contents = file_contents.replace(k, v);
                        }
                    }
                    file_contents
                };

                let compiled = Compiler::compile(
                    &file_contents,
                    &subfolder,
                    files::get_namespace(path).ok(),
                    &mut macros,
                );

                if let Err(compile_error) = compiled {
                    let canonical_path = path.canonicalize().unwrap();
                    let line = compile_error.line();

                    let (row, col) = match compile_error.line_col {
                        LineColLocation::Pos((row, col)) | LineColLocation::Span((row, col), _) => {
                            (row, col)
                        }
                    };

                    let line_highlighted = if !line.is_empty() {
                        let problem = &line[col..].split(' ').next().unwrap();
                        let mut highlight_text = String::new();

                        for _ in 0..(col - 1) {
                            highlight_text += " ";
                        }

                        for _ in 0..(problem.len() + 1) {
                            highlight_text += "^";
                        }

                        Some(highlight_text)
                    } else {
                        None
                    };

                    let base_error = format!(
                        "error: Unknown parsing error at {}:{}:{}",
                        canonical_path.display(),
                        row,
                        col,
                    );

                    if let Some(line_highlighted) = line_highlighted {
                        eprintln!(
                            "{}\nProblem line:\n{}\n{}",
                            base_error,
                            compile_error.line(),
                            line_highlighted,
                        );
                    } else {
                        eprintln!("{}\nMaybe there's a missing `end`?", base_error);
                    }

                    std::process::exit(1);
                };

                let mut compiled = compiled.unwrap();

                for (file, compiled_contents) in compiled.files.iter() {
                    if file.is_empty() {
                        continue;
                    }

                    let full_path = format!("{}/{}.mcfunction", target_path, file);

                    fs::write(full_path, compiled_contents)?;

                    // Add namespace prefix to function in tag map
                    for (_, funcs) in compiled.tags.iter_mut() {
                        if funcs.contains(file) {
                            let i = funcs.iter().position(|x| x == file).unwrap();
                            funcs[i] = format!(
                                "{}:{}{}",
                                files::get_namespace(&path).unwrap(),
                                &subfolder,
                                file
                            );
                        }
                    }
                }

                for (key, value) in compiled.tags {
                    tag_map
                        .entry(key)
                        .or_insert(Vec::new())
                        .append(&mut value.clone());
                }
            } else {
                let filename = relative_path.file_name().unwrap().to_str().unwrap();
                let full_path = format!("{}/{}", target_path, filename);
                // Only copy if the file doesn't exist yet
                // Intended to stop overwriting of Databind tags
                if fs::metadata(&full_path).is_err() {
                    fs::copy(&path, &full_path)?;
                }
            }
        }

        files::create_tag_files(src_dir, Path::new(&target_folder), &tag_map)?;
    } else {
        eprintln!("Databind does not support single-file compilation.");
        std::process::exit(1);
    }

    Ok(())
}
