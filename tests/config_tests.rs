use std::fs;

mod tests;

/// Test that the TOML config is properly followed
#[test]
fn test_config() {
    let mut path = tests::resources();
    path.push("test_config");
    let path_str = path.to_str().unwrap();

    println!("{}", path_str);

    println!(
        "{}",
        tests::run_with_args(
            "cargo",
            &[
                "run",
                "--",
                path_str,
                "--config",
                &format!("{}/databind.toml", path_str)[..],
            ],
        )
    );

    let expected_funcs = [
        "different_extension",
        "should_be_made",
        "func1",
        "func2",
        "func3",
    ];
    let unexpected_funcs = ["should_not_be_made"];

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

    // Ensure excluded files are not generated
    for file in unexpected_funcs.iter() {
        path.push(format!("{}.mcfunction", file));
        assert!(fs::metadata(&path).is_err());
        println!(
            "test_config: Function {}.mcfunction doesn't (and shouldn't) exist",
            file
        );
        path.pop();
    }

    path.pop();
    path.pop();

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

    println!(
        "{}",
        tests::run_with_args(
            "cargo",
            &[
                "run",
                "--",
                path_str,
                "--config",
                &format!("{}/should_not_be_made.toml", path_str)[..],
            ],
        )
    );

    let expected_funcs = ["tick"];
    let expected_toml = ["should_be_made"];
    let unexpected_toml = ["should_not_be_made"];

    path.pop();
    path.push("test_no_config_out.databind/data");

    // Check if function files are correctly placed
    path.push("test/functions");
    for file in expected_funcs.iter() {
        path.push(format!("{}.mcfunction", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_no_config_out: Function {}.mcfunction exists", file);
        path.pop();
    }

    path.pop();
    path.pop();
    path.pop();

    // Check if non-config toml is created
    for file in expected_toml.iter() {
        path.push(format!("{}.toml", file));
        assert!(fs::metadata(&path).is_ok());
        println!("test_no_config_out: File {}.toml exists", file);
        path.pop();
    }

    // Ensure config toml file is not outputted
    for file in unexpected_toml.iter() {
        path.push(format!("{}.toml", file));
        assert!(fs::metadata(&path).is_err());
        println!(
            "test_no_config_out: File {}.toml doesn't (and shouldn't) exist",
            file
        );
        path.pop();
    }

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_no_config_out.databind");
    fs::remove_dir_all(out_path).unwrap();
}
