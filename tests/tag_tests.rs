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

/// Test that tags are properly generated
#[test]
fn test_tag_generation() {
    let mut path = tests::resources();
    path.push("test_tag_generation");
    let path_str = path.to_str().unwrap();

    tests::run_with_args("cargo", &["run", "--", path_str, "--ignore-config"]);

    let expected_funcs = ["load.mcfunction", "tick.mcfunction", "func3.mcfunction"];
    let expected_tags = ["load.json", "tick.json", "second_tag.json", "func3.json"];
    let unexpected_tags = ["main.json"];

    path.pop();
    path.push("test_tag_generation.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    tests::check_files_exist(&path, &expected_funcs, "test_tag_generation");
    path.pop();
    path.pop();

    // Check if tag files are correctly placed
    path.push("minecraft/tags/functions");
    tests::check_files_exist(&path, &expected_tags, "test_tag_generation");

    // Ensure unexpected tag files do not exist
    tests::check_files_dont_exist(&path, &unexpected_tags, "test_tag_generation");

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_tag_generation.databind");
    fs::remove_dir_all(out_path).unwrap();
}

/// Test multiple ways to format :tag code
#[test]
fn test_tag_syntax() {
    let mut path = tests::resources();
    path.push("test_tag_syntax");
    let path_str = path.to_str().unwrap();

    tests::run_with_args("cargo", &["run", "--", path_str, "--ignore-config"]);

    let expected_funcs = ["func1.mcfunction", "func2.mcfunction", "func3.mcfunction"];
    let expected_tags = [
        "func1_tag.json",
        "func2_tag.json",
        "func3_tag.json",
        "all_tag.json",
    ];
    let unexpected_tags = ["main.json"];

    path.pop();
    path.push("test_tag_syntax.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    tests::check_files_exist(&path, &expected_funcs, "test_tag_syntax");
    path.pop();
    path.pop();

    // Check if tag files are correctly placed
    path.push("minecraft/tags/functions");
    tests::check_files_exist(&path, &expected_tags, "test_tag_syntax");

    // Ensure unexpected tag files do not exist
    tests::check_files_dont_exist(&path, &unexpected_tags, "test_tag_syntax");

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_tag_syntax.databind");
    fs::remove_dir_all(out_path).unwrap();
}
