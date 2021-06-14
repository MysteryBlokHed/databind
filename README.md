# Databind [![Crates.io Version](https://img.shields.io/crates/v/databind)](https://crates.io/crates/databind) [![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green)](#license) [![Documentation](https://readthedocs.org/projects/databind/badge/?version=latest)](https://databind.readthedocs.io/en/latest/)

Expand the functionality of Minecraft Datapacks.

## Features

- Multiple function definitions in a single file
- Tagging functions in-code
- Shorthand to call functions without namespace prefix (`func_1` instead of `namespace:func_1`)
- Transpile single files or entire folders
- Define text replacements for long and repeated lines of code
- While loops
- Variable definitions via scoreboards
- Shorthand for objective creation
- Shorthand for testing variables in `if` commands
- Configuration options

## Building and Running

This project requires [cargo](https://www.rust-lang.org/learn/get-started).

To build the project, clone the repo and run `cargo build` in the root directory.
To build for release, run `cargo build --release`.

To run Databind after building it with `cargo build`, use `cargo run`.

## Installation

### From crates.io

To download Databind from crates.io, run `cargo install databind`. If Rust is
[in your PATH](https://www.rust-lang.org/tools/install#installation-notes),
then running `databind` from a command line will work.

### Locally

To install Databind from a cloned repository, run `cargo install --path .` in the root directory.

## Documentation

Documentation is build using reStructuredText and Sphinx. Requires Python.
Built documentation is hosted on [Read The Docs](https://databind.readthedocs.io/en/latest/).

### Building Docs

To build the documentation, go to the /docs folder and run `pip install -r requirements.txt`.
Then run `make.bat html` or `make html`, depending on platform.

### Viewing Docs

To view the documentation, open the `index.html` file generated in /docs/\_build/html.

## License

Databind is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.
