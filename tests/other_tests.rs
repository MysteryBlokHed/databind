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
use std::fs;
use std::path::PathBuf;
use tempdir::TempDir;

mod tests;

/// Test the `databind create` file structure.
/// Also tests the contents of the config file and
/// the `pack.mcmeta` file
#[test]
fn test_create_structure() {
    let out = TempDir::new("test_create_structure").expect("Could not create tempdir for test");
    let mut path = PathBuf::from(out.path());

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            "create",
            "test_create_structure",
            "--path",
            out.path().to_str().unwrap(),
            "--desc",
            "test_create_structure description",
        ],
        None,
    );
    // Check that the folder was created
    assert!(path.exists() && path.is_dir());

    {
        // Check that the config file was created
        path.push("databind.toml");
        assert!(path.exists() && path.is_file());

        /// Settings for the compiler.
        /// Taken from `settings.rs`
        #[derive(Debug, PartialEq, Deserialize)]
        pub struct Settings {
            pub inclusions: Vec<String>,
            pub exclusions: Vec<String>,
            pub output: String,
        }

        // Check config file contents
        let contents = fs::read_to_string(&path).unwrap();
        let contents_config: Settings = toml::from_str(&contents).unwrap();
        // Same as Settings::default()
        let expected_config = Settings {
            inclusions: vec!["**/*.databind".into()],
            exclusions: Vec::new(),
            output: "out".into(),
        };
        assert_eq!(contents_config, expected_config);
        path.pop();
    }

    {
        // Check that the pack.mcmeta file was created
        path.push("src/pack.mcmeta");
        assert!(path.exists() && path.is_file());

        /// Inside pack.mcmeta.
        /// Taken from `create_project.rs`
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Pack {
            pack_format: u8,
            description: String,
        }

        /// pack.mcmeta.
        /// Taken from `create_project.rs`
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct PackMcMeta {
            pack: Pack,
        }

        let contents = fs::read_to_string(&path).unwrap();
        let contents_pack: PackMcMeta = serde_json::from_str(&contents).unwrap();
        let expected_pack = PackMcMeta {
            pack: Pack {
                pack_format: 7,
                description: "test_create_structure description".into(),
            },
        };
        assert_eq!(contents_pack, expected_pack);
        path.pop();
    }

    // Check that the main.databind file was created
    path.push("data/test_create_structure/functions/main.databind");
    assert!(path.exists() && path.is_file());
}
