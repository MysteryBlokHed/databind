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
use tempdir::TempDir;

mod tests;

/// Test that databind creates files in the right places
///
/// Uses `tests/resources/test_file_structure`
#[test]
fn test_file_structure() {
    let mut path = tests::resources();
    path.push("test_file_structure");

    let out = TempDir::new("test_file_structure").expect("Could not create tempdir for test");

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            path.to_str().unwrap(),
            "--ignore-config",
            "--out",
            out.path().to_str().unwrap(),
        ],
    );

    let expected_funcs = [
        "load.mcfunction",
        "tick.mcfunction",
        "first_func.mcfunction",
        "second_func.mcfunction",
    ];
    let expected_tags = ["load.json", "tick.json"];
    let unexpected_tags = ["main.json", "first_func.json", "second_func.json"];

    // Check if function files are correctly placed
    path.push(format!("{}/data/test/functions", out.path().display()));
    tests::check_files_exist(&path, &expected_funcs, "test_file_structure");
    path.pop();
    path.pop();

    // Check if tag files are correctly placed
    path.push("minecraft/tags/functions");
    tests::check_files_exist(&path, &expected_tags, "test_file_structure");

    // Ensure unexpected tag files do not exist
    tests::check_files_dont_exist(&path, &unexpected_tags, "test_file_structure");
}

/// Test that nested functions are properly generated
#[test]
fn test_nested_funcs() {
    let mut path = tests::resources();
    path.push("test_nested_funcs");

    let out = TempDir::new("test_nested_funcs").expect("Could not create tempdir for test");

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            path.to_str().unwrap(),
            "--ignore-config",
            "--out",
            out.path().to_str().unwrap(),
        ],
    );

    let expected_funcs = ["func1.mcfunction", "func2.mcfunction", "func3.mcfunction"];

    // Check if function files are correctly placed
    path.push(format!("{}/data/test/functions", out.path().display()));
    tests::check_files_exist(&path, &expected_funcs, "test_nested_funcs");
    path.pop();
    path.pop();
}
