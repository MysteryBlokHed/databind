# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.3] - August 12, 2021

### Changed

- Tests to properly use `--release` flag
- Empty lines replace in compiler

### Fixed

- A stack overflow caused by escaping double quotes in macro calls

## [0.6.2] - August 10, 2021

### Changed

- Cargo.toml `exclude` to `include`
- Paths that trigger Build And Run workflow

### Fixed

- NewLine token not being added with CRLF line endings

## [0.6.1] - August 8, 2021

### Changed

- Docs link in Cargo.toml

### Removed

- An unneeded condition from release.yaml

### Fixed

- Docs in macros.rst

## [0.6.0] - August 3, 2021

### Added

- Macros to allow complex functions in Databind
- If/else statements
- A file to define global variables
- Functionality to merge function tags defined in JSON files with Databind function tags
- A page with Folder Structure info to docs

### Removed

- `!def` (Replaced by macros)

### Fixed

- Some lines missing newlines at the end

## [0.5.0] - July 18, 2021

### Added

- `%` character to escape keywords

### Changed

- Both keywords `endfunc` and `endwhile` to `end`
- Docs formatting to add indentation inside functions and while loops

### Fixed

- Multiple `!def`'s not working

## [0.4.0] - July 1, 2021

### Changed

- Rewrote the tokenizer

### Removed

- Preceding `:` from all keywords

## [0.3.0] - June 26, 2021

### Added

- Shorthand to delete variables/objectives (`:delvar` or `:delobj`)
- Add shorthand for scoreboard operations (`:sbop` and `:gvar`)

### Changed

- Syntax for `:sobj`

### Removed

- Unmaintained `random_var_names` and `var_display_names` settings

### Fixed

- Integers not being able to be negative
- The only allowed assignment operator for objectives being `=`

## [0.2.3] - June 25, 2021

### Added

- New integration tests

### Changed

- Some old integration tests

### Fixed

- Output folder for `databind` with no args
- Incorrect function tagging
- Comments breaking tags
- While loop problems

## [0.2.2] - June 24, 2021

### Added

- Support for datapack subfolders

### Fixed

- Incorrect version in some places
- Docs license

## [0.2.1] - June 23, 2021

### Fixed

- A bug where running `databind` with no arguments would try to unwrap a `None` value

## [0.2.0] - June 23, 2021

### Added

- `databind create` subcommand to create new projects
- Ability to run `databind` with no arguments in a Databind project to compile
- `:tag` keyword to tag functions in-code
- `:def` keyword to define text replacements
- While loops
- Option to choose output file/folder

## [0.1.0] - June 4, 2021

### Added

- Multiple function definitions in a single file
- Shorthand to call functions without namespace prefix (`func_1` instead of `namespace:func_1`)
- Transpile single files or entire folders
- Variable definitions via scoreboards
- Shorthand for objective creation
- Shorthand for testing variables in `if` commands
- Configuration options
