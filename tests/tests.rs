/*
 * Databind - Expand the functionality of Minecraft Datapacks.
 * Copyright (C) 2021  Adam Thompson-Sharpe
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempdir::TempDir;

pub fn run_with_args(cmd: &str, args: &[&str], path: Option<&dyn AsRef<Path>>) -> Output {
    if let Some(p) = path {
        Command::new(cmd)
            .args(args)
            .current_dir(p.as_ref())
            .output()
            .expect("Failed to execute process")
    } else {
        Command::new(cmd)
            .args(args)
            .output()
            .expect("Failed to execute process")
    }
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

/// Run Databind on a path and send output to a path
pub fn run<P: AsRef<Path>>(out: P, path: P) {
    let args = if cfg!(debug_assertions) {
        vec![
            "run",
            "--",
            path.as_ref().to_str().unwrap(),
            "--ignore-config",
            "--out",
            out.as_ref().to_str().unwrap(),
        ]
    } else {
        vec![
            "run",
            "--release",
            "--",
            path.as_ref().to_str().unwrap(),
            "--ignore-config",
            "--out",
            out.as_ref().to_str().unwrap(),
        ]
    };

    run_with_args("cargo", &args, None);
}

/// Create a temporary output directory for a test and run
/// Databind there
pub fn run_in_tempdir(directory: &str) -> (TempDir, PathBuf) {
    let mut path = resources();
    path.push(directory);

    let out = TempDir::new(directory).expect("Could not create tempdir for test");
    run(out.path(), &path);
    (out, path)
}
