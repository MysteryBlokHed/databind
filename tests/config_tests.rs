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

    let expected_funcs = [
        "different_extension.mcfunction",
        "should_be_made.mcfunction",
        "func1.mcfunction",
        "func2.mcfunction",
        "func3.mcfunction",
    ];
    let unexpected_funcs = ["should_not_be_made.mcfunction"];
    path.pop();

    path.push("test_config.databind/data/test/functions");
    tests::check_files_exist(&path, &expected_funcs, "test_config:");
    path.pop();
    path.pop();

    path.push("minecraft/tags/functions");
    tests::check_files_dont_exist(&path, &unexpected_funcs, "test_config:");

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_config.databind");
    fs::remove_dir_all(out_path).unwrap();
}

#[test]
fn test_no_config_out() {
    let mut path = tests::resources();
    path.push("test_no_config_out");
    let path_str = path.to_str().unwrap();

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            path_str,
            "--config",
            &format!("{}/should_not_be_made.toml", path_str)[..],
        ],
    );

    let expected_funcs = ["tick.mcfunction"];
    let expected_toml = ["should_be_made.toml"];
    let unexpected_toml = ["should_not_be_made.toml"];

    path.pop();
    path.push("test_no_config_out.databind/data/test/functions");
    tests::check_files_exist(&path, &expected_funcs, "test_no_config_out:");
    path.pop();
    path.pop();
    path.pop();

    // Check if non-config toml is created
    tests::check_files_exist(&path, &expected_toml, "test_no_config_out");

    // Ensure config toml file is not outputted
    tests::check_files_dont_exist(&path, &unexpected_toml, "test_no_config_out");

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_no_config_out.databind");
    fs::remove_dir_all(out_path).unwrap();
}
