# Databind [![Crates.io Badge]](https://crates.io/crates/databind) [![License Badge]](#license) [![Docs Badge]](https://databind.readthedocs.io/en/stable/) [![Build & Test Badge]](https://github.com/MysteryBlokHed/databind/actions/workflows/build_and_test.yaml)

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

The installation instructions below are to build and install Databind from source.
If you'd like to download a built binary instead, go to the
[releases page](https://github.com/MysteryBlokHed/databind/releases).

### From crates.io

To download Databind from crates.io, run `cargo install databind --locked`. If Rust is
[in your PATH](https://www.rust-lang.org/tools/install#installation-notes),
then running `databind` from a command line will work.

### Locally

To install Databind from a cloned repository, run `cargo install --path . --locked` in the root directory.

## Documentation

### CLI/Language Docs

Documentation is build using reStructuredText and Sphinx. Requires Python.
Built documentation is hosted on [Read The Docs](https://databind.readthedocs.io/en/stable/).

#### Building Docs

To build the documentation, go to the /docs folder and run `pip install -r requirements.txt`.
Then run `make.bat html` or `make html`, depending on platform.

#### Viewing Docs

To view the documentation, open the `index.html` file generated in /docs/\_build/html.

### Library Docs

#### Building Docs

To build the library documentation, run `cargo doc` or `cargo doc --release`.

#### Viewing Docs

To view the docs, open the generated `index.html` file at `target/doc/databind/index.html`.
Built documentation is available at [docs.rs](docs.rs/databind/).

## License

Databind is licensed under the GNU General Public License, Version 3.0
([LICENSE](LICENSE) or <https://www.gnu.org/licenses/gpl-3.0.en.html>).

[crates.io badge]: https://img.shields.io/crates/v/databind
[license badge]: https://img.shields.io/github/license/MysteryBlokHed/databind
[docs badge]: https://readthedocs.org/projects/databind/badge/?version=latest
[build & test badge]: https://github.com/MysteryBlokHed/databind/actions/workflows/build_and_test.yaml/badge.svg
