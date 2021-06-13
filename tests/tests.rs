use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn run_with_args(cmd: &str, args: &[&str]) -> String {
    String::from_utf8(
        Command::new(cmd)
            .args(args)
            .output()
            .expect("Failed to execute process")
            .stdout,
    )
    .unwrap()
}

pub fn resources() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("tests/resources");
    d
}

/// Check if a list of files exist
///
/// # Arguments
///
/// - `files` - A list of paths to files
/// - `print_prefix` - A prefix for the message after assertion
pub fn check_files_exist<P: AsRef<Path>>(base_path: &PathBuf, files: &[P], print_prefix: &str) {
    let mut base = base_path.clone();
    // let base_dirs = base.ancestors().collect::<Vec<&Path>>().len();

    for file in files.iter() {
        base.push(file);
        assert!(fs::metadata(&base).is_ok());
        println!(
            "{} File {} exists (and should)",
            print_prefix,
            file.as_ref().display()
        );
        base.pop();
    }
}

/// Check if a list of files does not exist
///
/// # Arguments
///
/// - `files` - A list of paths to files
/// - `print_prefix` - A prefix for the message after assertion
pub fn check_files_dont_exist<P: AsRef<Path>>(
    base_path: &PathBuf,
    files: &[P],
    print_prefix: &str,
) {
    let mut base = base_path.clone();
    // let base_dirs = base.ancestors().collect::<Vec<&Path>>().len();

    for file in files.iter() {
        base.push(file);
        assert!(fs::metadata(&file).is_err());
        println!(
            "{} File {} does not exist (and shouldn't)",
            print_prefix,
            file.as_ref().display()
        );
        base.pop();
    }
}
