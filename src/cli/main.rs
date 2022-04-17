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
#![warn(clippy::all)]

use databind::compiler::Compiler;

// use same_file::is_same_file;
use std::{env, fs, io};

/// Temporary main function for testing the library portion of databind
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    let contents = fs::read_to_string(&args[1])?;
    println!("Contents: {}", contents);

    let ast = Compiler::parse(&contents).expect("Failed to parse");

    println!("Parsed: {:#?}", ast);

    println!(
        "Compiled: {:#?}",
        Compiler::compile(&contents, "").expect("Compile failed")
    );

    Ok(())
}
