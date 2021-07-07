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
