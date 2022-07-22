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
//! Expand the functionality of Minecraft Datapacks.
//!
//! ## CLI
//!
//! If you're just looking to use Databind as a tool and not to make your own
//! Rust project with it, then you can use the CLI. Documentation for it is
//! available [here](https://databind.rtfd.io/en/stable).
//!
//! ## Quick Start
//!
//! Here's some code to compile a given string and print the output
//! for each file:
//!
//! ```rust
//! use std::collections::HashMap;
//!
//! fn main() {
//!     // Databind ource file
//!     let source_file = "
//!     func main\n\
//!         say Hello, World!\n\
//!     end\n\
//!     func second_func\n\
//!         say Second function\n\
//!     end"
//!     .to_string();
//!
//!     // Compiled
//!     let compiled = Compiler::compile(&source_file, "");
//!     // Print the contents of each file
//!     for (file, contents) in compiled.files.iter() {
//!         println!("Output File: {}.mcfunction", file);
//!         println!("Contents:\n{}", contents);
//!         println!("----------");
//!     }
//! }
//! ```
//!
//! The code above results in the following output:
//!
//! ```text
//! Output File: main.mcfunction
//! Contents:
//! # Compiled with MysteryBlokHed/databind
//! say Hello, World!
//! ----------
//! Output File: second_func.mcfunction
//! Contents:
//! # Compiled with MysteryBlokHed/databind
//! say Second function
//! ----------
//! ```
#![warn(clippy::all)]
#[macro_use]
extern crate pest_derive;

pub mod ast;
pub mod compiler;
pub mod files;
mod settings;
pub use settings::Settings;
