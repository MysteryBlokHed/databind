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
use glob::glob;
use regex::Regex;
use std::path::PathBuf;
use tempdir::TempDir;
mod tests;

#[test]
fn test_while_structure() {
    let mut path = tests::resources();
    path.push("test_while_creation");

    let out = TempDir::new("test_while_structure").expect("Could not create tempdir for test");

    tests::run_with_args(
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
    );

    let files: Vec<PathBuf> = glob(&format!(
        "{}/data/test/functions/*.mcfunction",
        out.path().display()
    ))
    .unwrap()
    .filter_map(Result::ok)
    .collect();

    for file in files.iter() {
        let file_str = file
            .to_str()
            .unwrap()
            .split(|x: char| ['\\', '/'].contains(&x))
            .last()
            .unwrap();

        if file_str.starts_with("while") {
            let re = Regex::new("while_[0-9a-z]{4}.mcfunction").unwrap();
            assert!(re.is_match(file_str));
        } else if file_str.starts_with("condition") {
            let re = Regex::new("condition_[0-9a-z]{4}.mcfunction").unwrap();
            assert!(re.is_match(file_str));
        } else {
            assert!(file_str == "main.mcfunction");
        }
    }
}
