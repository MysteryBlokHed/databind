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
use glob::glob;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use tempdir::TempDir;

mod tests;

/// Test that databind creates files in the right places
///
/// Uses `tests/resources/test_file_structure`
#[test]
fn test_file_structure() {
    let (out, mut path) = tests::run_in_tempdir("test_file_structure");

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
    let (out, mut path) = tests::run_in_tempdir("test_nested_funcs");

    let expected_funcs = ["func1.mcfunction", "func2.mcfunction", "func3.mcfunction"];

    // Check if function files are correctly placed
    path.push(format!("{}/data/test/functions", out.path().display()));
    tests::check_files_exist(&path, &expected_funcs, "test_nested_funcs");
    path.pop();
    path.pop();
}

/// Test that the TOML config is properly followed
#[test]
fn test_config() {
    let mut path = tests::resources();
    path.push("test_config");
    let path_str = path.to_str().unwrap();

    let out = TempDir::new("test_config").expect("Could not create tempdir for test");

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            path_str,
            "--config",
            &format!("{}/databind.toml", path_str)[..],
            "--out",
            out.path().to_str().unwrap(),
        ],
        None,
    );

    let expected_funcs = ["func1.mcfunction", "func2.mcfunction", "func3.mcfunction"];

    path.push(format!("{}/data/test/functions", out.path().display()));
    tests::check_files_exist(&path, &expected_funcs, "test_config:");
    path.pop();
    path.pop();
}

#[test]
fn test_no_config_out() {
    let (out, mut path) = tests::run_in_tempdir("test_no_config_out");

    let expected_funcs = ["tick.mcfunction"];
    let expected_toml = ["should_be_made.toml"];
    let unexpected_toml = ["should_not_be_made.toml"];

    path.push(format!("{}/data/test/functions", out.path().display()));
    tests::check_files_exist(&path, &expected_funcs, "test_no_config_out:");
    path.pop();
    path.pop();
    path.pop();

    // Check if non-config toml is created
    tests::check_files_exist(&path, &expected_toml, "test_no_config_out");

    // Ensure config toml file is not outputted
    tests::check_files_dont_exist(&path, &unexpected_toml, "test_no_config_out");
}

/// Test that tags are properly generated
#[test]
fn test_tag_generation() {
    #[derive(Debug, PartialEq, Deserialize)]
    struct TagFile {
        pub values: Vec<String>,
    }

    let (out, mut path) = tests::run_in_tempdir("test_tag_generation");

    let expected_funcs = ["load.mcfunction", "tick.mcfunction", "func3.mcfunction"];
    let expected_tags = ["load.json", "tick.json", "second_tag.json", "func3.json"];
    let expected_tag_contents = [
        vec!["test:load".to_string()],
        vec!["test:tick".to_string()],
        vec!["test:load".to_string(), "test:tick".to_string()],
        vec!["test:func3".to_string()],
    ];
    let unexpected_tags = ["main.json"];

    // Check if function files are correctly placed
    path.push(format!("{}/data/test/functions", out.path().display()));
    tests::check_files_exist(&path, &expected_funcs, "test_tag_generation");
    path.pop();
    path.pop();

    // Check if tag files are correctly placed
    path.push("minecraft/tags/functions");
    tests::check_files_exist(&path, &expected_tags, "test_tag_generation");
    // Check tag file contents
    for i in 0..expected_tags.len() {
        path.push(&expected_tags[i]);
        let contents = fs::read_to_string(&path).unwrap();
        let contents_tag: TagFile = serde_json::from_str(&contents).unwrap();
        let expected_tag = TagFile {
            values: expected_tag_contents[i].clone(),
        };
        assert_eq!(contents_tag, expected_tag);
        path.pop();
    }

    // Ensure unexpected tag files do not exist
    tests::check_files_dont_exist(&path, &unexpected_tags, "test_tag_generation");
}

/// Test running `databind` alone in a created project.
/// Runs deeper than project root to ensure that it can both
/// find the `databind.toml` file and that the output folder
/// is in the correct directory
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

    path.push("src");
    println!("running in path: {}", path.display());

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

    // Pop /src from the path to check for /out in the project root
    path.pop();

    // Test that out directory exists
    path.push("out");
    println!("Checking path {}", path.display());
    assert!(path.exists() && path.is_dir());
}

/// Test that while loops create files in the right place
#[test]
fn test_while_structure() {
    let mut path = tests::resources();
    path.push("test_while_creation");

    let out = TempDir::new("test_while_structure").expect("Could not create tempdir for test");
    tests::run(out.path(), &path);

    let files: Vec<PathBuf> = glob(&format!(
        "{}/data/test/functions/*.mcfunction",
        out.path().display()
    ))
    .unwrap()
    .filter_map(Result::ok)
    .collect();

    for file in files.iter() {
        let file_str = file
            .to_str()
            .unwrap()
            .split(|x: char| ['\\', '/'].contains(&x))
            .last()
            .unwrap();

        if file_str.starts_with("while") {
            let re = Regex::new("while_[0-9a-z]{4}.mcfunction").unwrap();
            assert!(re.is_match(file_str));
        } else if file_str.starts_with("condition") {
            let re = Regex::new("condition_[0-9a-z]{4}.mcfunction").unwrap();
            assert!(re.is_match(file_str));
        } else {
            assert!(file_str == "main.mcfunction");
        }
    }
}

/// Test that if statements create files in the right place
#[test]
fn test_if_structure() {
    let mut path = tests::resources();
    path.push("test_if_creation");

    let out = TempDir::new("test_if_structure").expect("Could not create tempdir for test");
    tests::run(out.path(), &path);

    let files: Vec<PathBuf> = glob(&format!(
        "{}/data/test/functions/*.mcfunction",
        out.path().display()
    ))
    .unwrap()
    .filter_map(Result::ok)
    .collect();

    for file in files.iter() {
        let file_str = file
            .to_str()
            .unwrap()
            .split(|x: char| ['\\', '/'].contains(&x))
            .last()
            .unwrap();

        if file_str.starts_with("if_init") {
            let re = Regex::new("if_init_[0-9a-z]{4}.mcfunction").unwrap();
            assert!(re.is_match(file_str));
        } else if file_str.starts_with("if_true_") {
            let re = Regex::new("if_true_[0-9a-z]{4}.mcfunction").unwrap();
            assert!(re.is_match(file_str));
        } else if file_str.starts_with("if_false_") {
            let re = Regex::new("if_false_[0-9a-z]{4}.mcfunction").unwrap();
            assert!(re.is_match(file_str));
        } else {
            assert!(file_str == "main.mcfunction");
        }
    }
}
