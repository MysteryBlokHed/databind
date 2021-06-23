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
use std::path::PathBuf;
use tempdir::TempDir;

mod tests;

/// Test the `databind create` file structure
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
        ],
        None,
    );
    // Check that the folder was created
    assert!(path.exists() && path.is_dir());

    // Check that the config file was created
    path.push("databind.toml");
    assert!(path.exists() && path.is_file());
    path.pop();

    // Check that the pack.mcmeta file was created
    path.push("src/pack.mcmeta");
    assert!(path.exists() && path.is_file());
    path.pop();

    // Check that the main.databind file was created
    path.push("data/test_create_structure/functions/main.databind");
    assert!(path.exists() && path.is_file());
}

/// Test running `databind` alone in a created project
#[test]
fn test_databind_alone() {
    let out = TempDir::new("test_databind_alone").expect("Could not create tempdir for test");
    let mut path = PathBuf::from(out.path());

    // Create project
    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            "create",
            "test_databind_alone",
            "--path",
            out.path().to_str().unwrap(),
        ],
        None,
    );

    // Run `databind` in directory

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--manifest-path",
            &format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"))[..],
        ],
        Some(&path),
    );

    // Test that out directory exists
    path.push("out");
    println!("Checking path {}", path.display());
    assert!(path.exists() && path.is_dir());
}
