use std::fs;

mod tests;

/// Test that tags are properly generated
#[test]
fn test_tag_generation() {
    let mut path = tests::resources();
    path.push("test_tag_generation");
    let path_str = path.to_str().unwrap();

    println!("{}", path_str);

    tests::run_with_args("cargo", &["run", "--", path_str, "--ignore-config"]);

    let expected_funcs = ["main", "load", "tick", "func3"];
    let expected_tags = ["load", "tick", "second_tag", "func3"];
    let unexpected_tags = ["main"];

    path.pop();
    path.push("test_tag_generation.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    for file in expected_funcs.iter() {
        path.push(format!("{}.mcfunction", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_tag_generation: Function {}.mcfunction exists", file);
        path.pop();
    }

    path.pop();
    path.pop();

    // Check if tag files are correctly placed
    path.push("minecraft/tags/functions");
    for file in expected_tags.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_tag_generation: Tag {}.json exists", file);
        path.pop();
    }

    // Ensure unexpected tag files do not exist
    for file in unexpected_tags.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_err());
        println!(
            "test_tag_generation: Tag {}.json doesn't (and shouldn't) exist",
            file
        );
        path.pop();
    }

    path.pop();
    path.pop();

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

    println!("{}", path_str);

    tests::run_with_args("cargo", &["run", "--", path_str, "--ignore-config"]);

    let expected_funcs = ["main", "func1", "func2", "func3"];
    let expected_tags = ["func1_tag", "func2_tag", "func3_tag", "all_tag"];
    let unexpected_tags = ["main"];

    path.pop();
    path.push("test_tag_syntax.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    for file in expected_funcs.iter() {
        path.push(format!("{}.mcfunction", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_tag_syntax: Function {}.mcfunction exists", file);
        path.pop();
    }

    path.pop();
    path.pop();

    // Check if tag files are correctly placed
    path.push("minecraft/tags/functions");
    for file in expected_tags.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_tag_syntax: Tag {}.json exists", file);
        path.pop();
    }

    // Ensure unexpected tag files do not exist
    for file in unexpected_tags.iter() {
        path.push(format!("{}.json", file));
        assert!(fs::metadata(&path).is_err());
        println!(
            "test_tag_syntax: Tag {}.json doesn't (and shouldn't) exist",
            file
        );
        path.pop();
    }

    path.pop();
    path.pop();

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_tag_syntax.databind");
    fs::remove_dir_all(out_path).unwrap();
}
