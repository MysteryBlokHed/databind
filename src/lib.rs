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
//! use databind::compiler::Compiler;
//! use std::collections::HashMap;
//!
//! #[test]
//! fn joe() {
//!     // Source file
//!     let source_file = "
//!     func main\n\
//!         say Hello, World!\n\
//!     end\n\
//!     func second_func\n\
//!         say Second function\n\
//!     end"
//!     .to_string();
//!
//!     // Create compiler and tokenize source file
//!     let mut compiler = Compiler::new(source_file, None);
//!     let tokens = compiler.tokenize();
//!     // Compile tokens to files
//!     let compiled = compiler.compile(tokens, None, "", &HashMap::new(), false);
//!     // Print the contents of each file
//!     for (k, v) in compiled.filename_map.iter() {
//!         println!("Output File: {}.mcfunction", k);
//!         println!("Contents:\n{}", compiled.file_contents[*v]);
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

pub mod compiler;
pub mod settings;
pub mod token;
