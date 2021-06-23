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
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

fn make_pack_mcmeta(description: String) -> Result<String, serde_json::Error> {
    #[derive(Serialize)]
    struct Pack {
        pack_format: u8,
        description: String,
    }

    #[derive(Serialize)]
    struct PackMcMeta {
        pack: Pack,
    }

    let pack = PackMcMeta {
        pack: Pack {
            pack_format: 6,
            description,
        },
    };

    serde_json::to_string(&pack)
}

fn dir_empty(path: &dyn AsRef<Path>) -> std::io::Result<bool> {
    Ok(path.as_ref().read_dir()?.next().is_none())
}

/// Handle the create subcommand
///
/// # Arguments
///
/// - `args` - Matches from the create subcommand
pub fn create_project(args: clap::ArgMatches) -> std::io::Result<()> {
    let allowed_chars = "abcdefghijklmnopqrstuvwxyz0123456789_-.";

    let name = args.value_of("name").unwrap();

    if name.chars().any(|x| !allowed_chars.contains(x)) {
        println!("Project name contains disallowed characters");
        std::process::exit(1);
    }

    let description = args.value_of("description").unwrap().to_string();
    let base_path = if args.is_present("path") {
        args.value_of("path").unwrap()
    } else {
        name
    };

    let mut path = PathBuf::from(base_path);

    let metadata = fs::metadata(&path);

    if let Ok(meta) = metadata {
        if meta.is_file() || meta.is_dir() && !dir_empty(&path)? {
            println!(
                "Path {} is an already existant file or non-empty folder",
                base_path
            );
            std::process::exit(1);
        }
    }

    path.push(format!("src/data/{}/functions", name));
    fs::create_dir_all(&path)?;

    // Create main.databind
    path.push("main.databind");
    fs::write(
        &path,
        "\
    :func main\n\
    :tag load\n\
    tellraw @a \"Hello, World!\"\n\
    :endfunc\n",
    )?;

    path.pop();
    path.pop();
    path.pop();
    path.pop();

    // Create pack.mcmeta
    let pack_mcmeta = make_pack_mcmeta(description)?;
    path.push("pack.mcmeta");
    fs::write(&path, pack_mcmeta)?;

    path.pop();
    path.pop();

    // Create databind.toml
    let databind_toml = toml::to_string(&Settings::default()).unwrap();
    path.push("databind.toml");
    fs::write(&path, databind_toml)?;

    println!("Created project {} in {}", name, base_path);
    Ok(())
}
