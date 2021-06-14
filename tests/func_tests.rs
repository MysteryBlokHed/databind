use std::fs;

mod tests;

/// Test that databind creates files in the right places
///
/// Uses `tests/resources/test_file_structure`
#[test]
fn test_file_structure() {
    let mut path = tests::resources();
    path.push("test_file_structure");
    let path_str = path.to_str().unwrap();

    tests::run_with_args("cargo", &["run", "--", path_str, "--ignore-config"]);

    let expected_funcs = [
        "load.mcfunction",
        "tick.mcfunction",
        "first_func.mcfunction",
        "second_func.mcfunction",
    ];
    let expected_tags = ["load.json", "tick.json"];
    let unexpected_tags = ["main.json", "first_func.json", "second_func.json"];

    path.pop();
    path.push("test_file_structure.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    tests::check_files_exist(&path, &expected_funcs, "test_file_structure");
    path.pop();
    path.pop();

    // Check if tag files are correctly placed
    path.push("minecraft/tags/functions");
    tests::check_files_exist(&path, &expected_tags, "test_file_structure");

    // Ensure unexpected tag files do not exist
    tests::check_files_dont_exist(&path, &unexpected_tags, "test_file_structure");

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_file_structure.databind");
    fs::remove_dir_all(out_path).unwrap();
}

/// Test that nested functions are properly generated
#[test]
fn test_nested_funcs() {
    let mut path = tests::resources();
    path.push("test_nested_funcs");
    let path_str = path.to_str().unwrap();

    tests::run_with_args("cargo", &["run", "--", path_str, "--ignore-config"]);

    let expected_funcs = ["func1.mcfunction", "func2.mcfunction", "func3.mcfunction"];

    path.pop();
    path.push("test_nested_funcs.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    tests::check_files_exist(&path, &expected_funcs, "test_nested_funcs");
    path.pop();
    path.pop();

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_nested_funcs.databind");
    fs::remove_dir_all(out_path).unwrap();
}
