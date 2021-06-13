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

    let expected_funcs = ["main", "load", "tick", "first_func", "second_func"];
    let expected_tags = ["load", "tick"];
    let unexpected_tags = ["main", "first_func", "second_func"];

    path.pop();
    path.push("test_file_structure.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    for file in expected_funcs.iter() {
        path.push(format!("{}.mcfunction", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_file_structure: Function {}.mcfunction exists", file);
        path.pop();
    }

    path.pop();
    path.pop();

    // Check if tag files are correctly placed
    path.push("minecraft/tags/functions");
    for file in expected_tags.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_file_structure: Tag {}.json exists", file);
        path.pop();
    }

    // Ensure unexpected tag files do not exist
    for file in unexpected_tags.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_err());
        println!(
            "test_file_structure: Tag {}.json doesn't (and shouldn't) exist",
            file
        );
        path.pop();
    }

    path.pop();
    path.push("test_file_structure.databind/data");

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_file_structure.databind");
    fs::remove_dir_all(out_path).unwrap();
}
