# Databind [![Crates.io Badge]](https://crates.io/crates/databind) [![License Badge]](#license) [![Documentation]](https://databind.readthedocs.io/en/stable/)

Expand the functionality of Minecraft Datapacks.

## Getting Started

To get started, see the [Getting Started](https://databind.readthedocs.io/en/stable/getting_started.html)
page on the docs.

## Features

- Can be integrated with existing datapacks/mcfunctions
- Multiple mcfunction definitions in a single file
- Custom functions (macros) that can take arguments
- Tagging functions in-code
- Shorthand to call functions without namespace prefix (eg. `func_1` instead of `namespace:func_1`)
- Subcommand to create new projects easily
- If/else statements
- While loops
- A file to define variables that can be used anywhere
- Variable definitions via scoreboards
- Shorthand for objective creation
- Shorthand for testing variables in `if` commands
- Shorthand for scoreboard operations
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
Built documentation is hosted on [Read The Docs](https://databind.readthedocs.io/en/stable/).

### Building Docs

To build the documentation, go to the /docs folder and run `pip install -r requirements.txt`.
Then run `make.bat html` or `make html`, depending on platform.

### Viewing Docs

To view the documentation, open the `index.html` file generated in /docs/\_build/html.

## License

Databind is licensed under the GNU General Public License, Version 3.0
([LICENSE](LICENSE) or <https://www.gnu.org/licenses/gpl-3.0.en.html>).

[crates.io badge]: https://img.shields.io/crates/v/databind
[license badge]: https://img.shields.io/github/license/MysteryBlokHed/databind
[documentation]: https://readthedocs.org/projects/databind/badge/?version=latest
