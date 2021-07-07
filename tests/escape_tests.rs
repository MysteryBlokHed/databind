use std::fs;
use std::str;
use tempdir::TempDir;

mod tests;

/// Test that escaped keywords are properly escaped
#[test]
fn test_escape() {
    let mut path = tests::resources();
    path.push("test_escape");

    let out = TempDir::new("test_escape").expect("Could not create tempdir for test");

    println!(
        "{}",
        str::from_utf8(
            &tests::run_with_args(
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
            )
            .stdout
        )
        .unwrap()
    );

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
