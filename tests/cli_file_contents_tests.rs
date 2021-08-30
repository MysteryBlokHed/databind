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
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

mod tests;

#[derive(Debug, PartialEq, Deserialize)]
struct TagFile {
    pub values: Vec<String>,
}

/// Test multiple ways to format tag code
#[test]
fn test_tag_syntax() {
    let (out, mut path) = tests::run_in_tempdir("test_tag_syntax");

    let expected_funcs = ["func1.mcfunction", "func2.mcfunction", "func3.mcfunction"];
    let expected_tags = [
        "func1_tag.json",
        "func2_tag.json",
        "func3_tag.json",
        "all_tag.json",
    ];
    let expected_tag_contents = [
        vec!["test:func1".to_string()],
        vec!["test:func2".to_string()],
        vec!["test:func3".to_string()],
        vec![
            "test:func1".to_string(),
            "test:func2".to_string(),
            "test:func3".to_string(),
        ],
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
    tests::check_files_dont_exist(&path, &unexpected_tags, "test_tag_syntax");
}

/// Test that existing function tags defined with a JSON file aren't
/// overwritten by Databind's function tags
#[test]
fn test_existing_tags() {
    let out = tests::run_in_tempdir("test_existing_tags").0;

    let path = format!(
        "{}/data/minecraft/tags/functions/test.json",
        out.path().display()
    );

    let contents = fs::read_to_string(&path).unwrap();
    let tags: TagFile = serde_json::from_str(&contents).unwrap();

    assert!(tags.values.contains(&"test:databind_tagged".into()));
    assert!(tags.values.contains(&"test:mcfunction_tagged".into()));
}

/// Test that use of tags such as `kill @e[type=#namespace:tag]`
/// is not removed by comments
#[test]
fn test_tags_and_comments() {
    let (out, mut path) = tests::run_in_tempdir("test_tags_and_comments");

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

/// Test that escaped keywords are properly escaped
#[test]
fn test_escape() {
    let (out, mut path) = tests::run_in_tempdir("test_escape");

    let expected_funcs = [
        "main.mcfunction",
        "func.mcfunction",
        "%percent_prefix.mcfunction",
    ];

    // Check if function files are correctly placed
    path.push(format!("{}/data/test/functions", out.path().display()));
    tests::check_files_exist(&path, &expected_funcs, "test_escape");
    path.push("main.mcfunction");

    // Check contents of main.mcfunction
    let main_contents = fs::read_to_string(&path).unwrap();
    println!("{}", main_contents);
    assert!(main_contents.contains("say call"));
    assert!(main_contents.contains("function test:func"));
    assert!(main_contents.contains("function test:%percent_prefix"));
}

struct WhileFiles {
    main_files: Vec<PathBuf>,
    conditions: Vec<PathBuf>,
}

/// Returns the mcfunction files for a while loop for a given functions folder
fn get_while_files<P: AsRef<Path>>(functions_path: P) -> WhileFiles {
    let path = functions_path.as_ref();
    let main_files = glob(&format!("{}/while_*.mcfunction", path.display()))
        .unwrap()
        .filter_map(Result::ok)
        .collect();
    let conditions = glob(&format!("{}/condition_*.mcfunction", path.display()))
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    WhileFiles {
        main_files,
        conditions,
    }
}

/// Test that the contents of generated while loop functions are correct
#[test]
fn test_while_creation() {
    let out = tests::run_in_tempdir("test_while_creation").0;

    // Get the randomly-named while loop files
    let files = get_while_files(format!("{}/data/test/functions", out.path().display()));
    assert!(!files.main_files.is_empty());
    assert!(!files.conditions.is_empty());
    let while_file = &files.main_files[0];
    let condition_file = &files.conditions[0];
    // Get the function names (including the namespace) of the while functions via their paths
    let while_func = format!("test:{}", while_file.file_stem().unwrap().to_str().unwrap());
    let condition_func = format!(
        "test:{}",
        condition_file.file_stem().unwrap().to_str().unwrap()
    );

    // Test the contents of the main while function
    let while_contents = fs::read_to_string(&while_file).unwrap();
    assert!(while_contents.contains(&format!(
        "execute if CONDITION run function {}",
        condition_func
    )));
    // Test the contents of the condition function
    let condition_contents = fs::read_to_string(&condition_file).unwrap();
    assert!(condition_contents.contains("say Inside loop"));
    assert!(condition_contents.contains(&format!("function {}", while_func)));
}

/// Test that macros are properly replaced using different
/// formatting
#[test]
fn test_macro_replacement() {
    let out = tests::run_in_tempdir("test_macro_replacement").0;

    let expected_lines = [
        "say Call Test 1",
        "say Call Test 2",
        "say Call Test 3",
        "say Call Test 4",
        "say Call Test 5",
        "say Call Test 6",
        "say Call Test 7",
        "say Call Test 8",
        "say Call Test 9",
        "say Def Test 1",
        "say Def Test 2",
        "say Def Test 3",
        "say Def Test 4",
        "say Def Test 5",
    ];

    let out_path = format!(
        "{}/data/test/functions/main.mcfunction",
        out.path().display()
    );
    let contents = fs::read_to_string(&out_path).unwrap();

    for line in expected_lines.iter() {
        assert!(contents.contains(line));
        println!("Has line {}", line);
    }
}

/// Test that macros calling other macros work properly
#[test]
fn test_macro_recursion() {
    let out = tests::run_in_tempdir("test_macro_recursion").0;

    let out_path = format!(
        "{}/data/test/functions/main.mcfunction",
        out.path().display()
    );
    let contents = fs::read_to_string(&out_path).unwrap();
    assert!(contents.contains("say Macro 1 - Test"));
}

/// Test that global variables are properly read and replaced
#[test]
fn test_global_vars() {
    let out = tests::run_in_tempdir("test_global_vars").0;

    let out_path = format!(
        "{}/data/test/functions/main.mcfunction",
        out.path().display()
    );
    let contents = fs::read_to_string(&out_path).unwrap();

    assert!(contents.contains("say Variable 1"));
    assert!(contents.contains("tellraw @a \"Variable 2\""));
}

/// Test that multiple TOML types are properly supported by
/// global vars
#[test]
fn test_types_global_vars() {
    let out = tests::run_in_tempdir("test_types_global_vars").0;

    let out_path = format!(
        "{}/data/test/functions/main.mcfunction",
        out.path().display()
    );

    let expected_lines = [
        "say string=String",
        "say int=1",
        "say float=3.14",
        "say bool_true=1",
        "say bool_false=0",
    ];

    let contents = fs::read_to_string(&out_path).unwrap();

    for line in expected_lines.iter() {
        assert!(contents.contains(line));
        println!("Line {:?} found", line);
    }
}

/// Test that global macros are properly dealt with
#[test]
fn test_global_macros() {
    let out = tests::run_in_tempdir("test_global_macros").0;

    let out_path = format!(
        "{}/data/test/functions/main.mcfunction",
        out.path().display()
    );
    let contents = fs::read_to_string(&out_path).unwrap();
    assert!(contents.contains("say Global Macro Test"));
}

/// Test that macro definitions in other macro definitions
/// work properly
#[test]
fn test_macro_defs_in_defs() {
    let out = tests::run_in_tempdir("test_macro_defs_in_defs").0;
    let out_path = format!(
        "{}/data/test/functions/main.mcfunction",
        out.path().display()
    );

    let contents = fs::read_to_string(&out_path).unwrap();
    assert!(contents.contains("say Hello from new macro"));
}

/// Test that ecaped characters work in macros
#[test]
fn test_macro_escape() {
    let out = tests::run_in_tempdir("test_macro_escape").0;
    let out_path = format!(
        "{}/data/test/functions/main.mcfunction",
        out.path().display()
    );

    let contents = fs::read_to_string(&out_path).unwrap();
    assert!(contents.contains("say Quote: \""));
    assert!(contents.contains("say Backslash: \\"));
}

/// Test that the modulus operator is not treated as an escaped
/// equals sign
#[test]
fn test_modulus_operator() {
    let out = tests::run_in_tempdir("test_modulus_operator").0;
    let out_path = format!(
        "{}/data/test/functions/main.mcfunction",
        out.path().display()
    );

    let contents = fs::read_to_string(&out_path).unwrap();
    assert!(
        contents.contains("scoreboard players operation testplayer score1 %= testplayer score2")
    );
}
