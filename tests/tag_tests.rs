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
use tempdir::TempDir;

mod tests;

/// Test that tags are properly generated
#[test]
fn test_tag_generation() {
    let mut path = tests::resources();
    path.push("test_tag_generation");

    let out = TempDir::new("test_tag_generation").expect("Could not create tempdir for test");

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
        None,
    );

    let expected_funcs = ["load.mcfunction", "tick.mcfunction", "func3.mcfunction"];
    let expected_tags = ["load.json", "tick.json", "second_tag.json", "func3.json"];
    let unexpected_tags = ["main.json"];

    // Check if function files are correctly placed
    path.push(format!("{}/data/test/functions", out.path().display()));
    tests::check_files_exist(&path, &expected_funcs, "test_tag_generation");
    path.pop();
    path.pop();

    // Check if tag files are correctly placed
    path.push("minecraft/tags/functions");
    tests::check_files_exist(&path, &expected_tags, "test_tag_generation");

    // Ensure unexpected tag files do not exist
    tests::check_files_dont_exist(&path, &unexpected_tags, "test_tag_generation");
}

/// Test multiple ways to format :tag code
#[test]
fn test_tag_syntax() {
    let mut path = tests::resources();
    path.push("test_tag_syntax");

    let out = TempDir::new("test_tag_syntax").expect("Could not create tempdir for test");

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
        None,
    );

    let expected_funcs = ["func1.mcfunction", "func2.mcfunction", "func3.mcfunction"];
    let expected_tags = [
        "func1_tag.json",
        "func2_tag.json",
        "func3_tag.json",
        "all_tag.json",
    ];
    let unexpected_tags = ["main.json"];

    // Check if function files are correctly placed
    path.push(format!("{}/data/test/functions", out.path().display()));
    tests::check_files_exist(&path, &expected_funcs, "test_tag_syntax");
    path.pop();
    path.pop();

    // Check if tag files are correctly placed
    path.push("minecraft/tags/functions");
    tests::check_files_exist(&path, &expected_tags, "test_tag_syntax");

    // Ensure unexpected tag files do not exist
    tests::check_files_dont_exist(&path, &unexpected_tags, "test_tag_syntax");
}

/// Test that use of tags such as `kill @e[type=#namespace:tag]`
/// is not removed by comments
#[test]
fn test_tags_and_comments() {
    let mut path = tests::resources();
    path.push("test_tags_and_comments");

    let out = TempDir::new("test_tags_and_comments").expect("Could not create tempdir for test");

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
        None,
    );

    let expected_funcs = ["func1.mcfunction"];
    let expected_include = "kill @e[type=#test:tag_should_be_included]";
    let expected_exclude = "# should not be included";

    path.push(format!("{}/data/test/functions", out.path().display()));
    tests::check_files_exist(&path, &expected_funcs, "test_tags_and_comments");

    // Check contents of func1.mcfunction
    path.push("func1.mcfunction");
    let contents = fs::read_to_string(&path).unwrap();
    assert!(contents.contains(expected_include));
    assert!(!contents.contains(expected_exclude));
}
