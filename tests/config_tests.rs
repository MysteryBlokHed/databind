use std::fs;

mod tests;

/// Test that the TOML config is properly followed
#[test]
fn test_config() {
    let mut path = tests::resources();
    path.push("test_config");
    let path_str = path.to_str().unwrap();

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            path_str,
            "--config",
            &format!("{}/databind.toml", path_str)[..],
        ],
    );

    let expected_funcs = ["main", "should_be_made", "should_not_be_made"];
    let expected_json = ["main", "should_be_made"];
    let unexpected_json = ["should_not_be_made"];

    path.pop();
    path.push("test_config.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    for file in expected_funcs.iter() {
        path.push(format!("{}.mcfunction", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_config: Function {}.mcfunction exists", file);
        path.pop();
    }

    path.pop();
    path.pop();

    // Check if JSON files are correctly placed
    path.push("minecraft/tags/functions");
    for file in expected_json.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_config: JSON {}.json exists", file);
        path.pop();
    }

    // Ensure unexpected JSON files do not exist
    for file in unexpected_json.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_err());
        println!(
            "test_config: JSON {}.json doesn't (and shouldn't) exist",
            file
        );
        path.pop();
    }

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_config.databind");
    fs::remove_dir_all(out_path).unwrap();
}

/// Test that CLI options are properly followed
#[test]
fn test_cli_args_no_json() {
    let mut path = tests::resources();
    path.push("test_cli_args");
    let path_str = path.to_str().unwrap();

    tests::run_with_args("cargo", &["run", "--", path_str, "--ignore-config"]);

    let expected_funcs = ["main", "a_function"];
    let unexpected_json = ["main", "a_function"];

    path.pop();
    path.push("test_cli_args.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    for file in expected_funcs.iter() {
        path.push(format!("{}.mcfunction", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_cli_args_no_json: Function {}.mcfunction exists", file);
        path.pop();
    }

    path.pop();
    path.pop();

    // Ensure unexpected JSON files do not exist
    path.push("minecraft/tags/functions");
    for file in unexpected_json.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_err());
        println!(
            "test_cli_args_no_json: JSON {}.json doesn't (and shouldn't) exist",
            file
        );
        path.pop();
    }

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_cli_args.databind");
    fs::remove_dir_all(out_path).unwrap();
}

#[test]
fn test_cli_args_generate_json() {
    let mut path = tests::resources();
    path.push("test_cli_args");
    let path_str = path.to_str().unwrap();

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            path_str,
            "--ignore-config",
            "--generate-func-json",
        ],
    );

    let expected_funcs = ["main", "a_function"];
    let expected_json = ["main", "a_function"];

    path.pop();
    path.push("test_cli_args.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    for file in expected_funcs.iter() {
        path.push(format!("{}.mcfunction", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_cli_args_no_json: Function {}.mcfunction exists", file);
        path.pop();
    }

    path.pop();
    path.pop();

    // Check if JSON files are correctly placed
    path.push("minecraft/tags/functions");
    for file in expected_json.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_config: JSON {}.json exists", file);
        path.pop();
    }

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_cli_args.databind");
    fs::remove_dir_all(out_path).unwrap();
}
