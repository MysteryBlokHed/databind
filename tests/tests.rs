use std::path::PathBuf;
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
