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

mod tests;

/// Test that the TOML config is properly followed
#[test]
fn test_config() {
    let mut path = tests::resources();
    path.push("test_config");
    let path_str = path.to_str().unwrap();

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            path_str,
            "--config",
            &format!("{}/databind.toml", path_str)[..],
        ],
    );

    let expected_funcs = ["func1.mcfunction", "func2.mcfunction", "func3.mcfunction"];
    path.pop();

    path.push("test_config.databind/data/test/functions");
    tests::check_files_exist(&path, &expected_funcs, "test_config:");
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

    tests::run_with_args(
        "cargo",
        &[
            "run",
            "--",
            path_str,
            "--config",
            &format!("{}/should_not_be_made.toml", path_str)[..],
        ],
    );

    let expected_funcs = ["tick.mcfunction"];
    let expected_toml = ["should_be_made.toml"];
    let unexpected_toml = ["should_not_be_made.toml"];

    path.pop();
    path.push("test_no_config_out.databind/data/test/functions");
    tests::check_files_exist(&path, &expected_funcs, "test_no_config_out:");
    path.pop();
    path.pop();
    path.pop();

    // Check if non-config toml is created
    tests::check_files_exist(&path, &expected_toml, "test_no_config_out");

    // Ensure config toml file is not outputted
    tests::check_files_dont_exist(&path, &unexpected_toml, "test_no_config_out");

    // Delete generated folder
    let mut out_path = tests::resources();
    out_path.push("test_no_config_out.databind");
    fs::remove_dir_all(out_path).unwrap();
}
