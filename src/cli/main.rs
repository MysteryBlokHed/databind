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
    compiler::Compiler,
    files,
    types::{GlobalMacros, TagMap},
    Settings,
};
use same_file::is_same_file;
use std::{
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
            println!("Non-existant config file specified.");
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
        println!("Directory provided for config file.");
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
        let mut tag_map = TagMap::new();
        let mut global_macros = GlobalMacros::new();
        let target_folder = &compiler_settings.output;

        if fs::metadata(target_folder).is_ok() {
            println!("Deleting old databind folder...");
            fs::remove_dir_all(&target_folder)?;
            println!("Deleted.");
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
                let contents = {
                    let mut file_contents = fs::read_to_string(path)
                        .expect(&format!("Failed to read file {}", path.display())[..]);
                    if let Some(vars_map) = &vars {
                        for (k, v) in vars_map.iter() {
                            file_contents = file_contents.replace(k, v);
                        }
                    }
                    file_contents
                };
                let mut compile = Compiler::new(
                    contents,
                    Some(path.canonicalize().unwrap().to_str().unwrap().into()),
                );
                let tokens = compile.tokenize();

                let mut compiled = compile.compile_check_macro(
                    tokens,
                    path.file_name().unwrap().to_str().unwrap(),
                    path,
                    &mut global_macros,
                );

                for (key, value) in compiled.filename_map.iter() {
                    let full_path = format!("{}/{}.mcfunction", target_path, key);

                    fs::write(full_path, &compiled.file_contents[*value])?;

                    // Add namespace prefix to function in tag map
                    for (_, funcs) in compiled.tag_map.iter_mut() {
                        if funcs.contains(key) {
                            let i = funcs.iter().position(|x| x == key).unwrap();
                            funcs[i] = format!(
                                "{}:{}{}",
                                files::get_namespace(&path).unwrap(),
                                files::get_subfolder_prefix(&path),
                                key
                            );
                        }
                    }
                }

                for (key, value) in compiled.tag_map {
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
        println!("Databind does not support single-file compilation.");
        std::process::exit(1);
    }

    Ok(())
}
