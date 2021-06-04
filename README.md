<h1 align="center">Databind</h1>
<!-- Badges -->
<p align="center">
  <a href="https://crates.io/crates/databind">
    <img src="https://img.shields.io/crates/v/databind" />
  </a>
  <a href="https://databind.readthedocs.io/en/latest/">
    <img src="https://readthedocs.org/projects/databind/badge/?version=latest" />
  </a>
  <a href="#license">
    <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green"/>
  </a>
</p>
<p align="center">Expand the functionality of Minecraft Datapacks.</p>

## Building and Running

This project requires [cargo](https://www.rust-lang.org/learn/get-started).

### Building

To build the project, clone the repo and run `cargo build` in the root directory.
To build for release, run `cargo build --release`.

### Running

#### After `cargo build`

To run Databind after building it with `cargo build`, use `cargo run`.

#### From crates.io

To download Databind from crates.io, run `cargo install databind`. If Rust is
[in your PATH](https://www.rust-lang.org/tools/install#installation-notes),
then running `databind` from a command line will work.

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
