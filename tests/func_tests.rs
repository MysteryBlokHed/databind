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
    tests::run_with_args("cargo", &["run", "--", path_str]);

    let expected_files = ["load", "tick", "first_func", "second_func"];

    path.pop();
    path.push("test_file_structure.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    for file in expected_files.iter() {
        path.push(format!("{}.mcfunction", file));
        println!("testing exists @: {}", path.display());
        assert!(fs::metadata(&path).is_ok());
        path.pop();
    }

    path.pop();
    path.pop();

    // Check if json files are correctly placed
    path.push("minecraft/tags/functions");
    for file in expected_files.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_ok());
        path.pop();
    }

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_file_structure.databind");
    fs::remove_dir_all(out_path).unwrap();
}
