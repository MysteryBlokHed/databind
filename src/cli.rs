use clap::{App, Arg};

/// Set up Clap CLI and get arguments
pub fn get_cli_matches<'a>() -> clap::ArgMatches<'a> {
    App::new("Databind")
        .version("0.1.0")
        .author("Adam Thompson-Sharpe <adamthompsonsharpe@gmail.com>")
        .about("Expand the functionality of Minecraft Datapacks.")
        .arg(
            Arg::with_name("DATAPACK")
                .help("The datapack (or file) to transpile")
                .required(true),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Configuration for the transpiler")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ignore-config")
                .long("ignore-config")
                .help("Ignore the config file. Used for testing"),
        )
        .arg(
            Arg::with_name("generate-func-tags")
                .long("generate-func-tags")
                .help("Generate tags for functions in minecraft/tags/functions"),
        )
        .arg(
            Arg::with_name("random-var-names")
                .long("random-var-names")
                .help(
                    "Add characters to the end of variable names. \
                Does not work when using variables across multiple files",
                ),
        )
        .arg(
            Arg::with_name("var-display-names")
                .long("var-display-names")
                .help(
                    "Change the display name of variables in-game to hide extra characters. \
                Only relevant with --random-var-names",
                ),
        )
        .get_matches()
}
