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
use std::fs;

mod tests;

/// Test that the TOML config is properly followed
#[test]
fn test_create_structure() {
    let mut path = tests::resources();
    path.push("test_create_structure");
    let path_str = path.to_str().unwrap();

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            "create",
            "test_create_structure",
            "--path",
            path_str,
        ],
    );

    println!("alleged path: {}", path.display());

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

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_create_structure");
    fs::remove_dir_all(out_path).unwrap();
}
