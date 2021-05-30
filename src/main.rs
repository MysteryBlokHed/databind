use clap::{App, Arg};
use std::fs;

mod settings;
mod token;
mod transpiler;

fn main() {
    let matches = App::new("Databind")
        .version("0.1.0")
        .author("Adam Thompson-Sharpe <adamthompsonsharpe@gmail.com>")
        .about("A superset of mcfunctions for Minecraft Datapacks.")
        .arg(
            Arg::with_name("FILE")
                .help("The databind file to transpile")
                .required(true),
        )
        .arg(
            Arg::with_name("random-var-names")
                .long("random-var-names")
                .help("Add characters to the end of variable names"),
        )
        .arg(
            Arg::with_name("var-display-names")
                .long("var-display-names")
                .help(
                    "Change the display name of variables in-game to hide extra characters. \
                    Only relevant with --random-var-names",
                ),
        )
        .get_matches();

    let transpiler_settings = settings::Settings {
        randomize_var_names: matches.is_present("random-var-names"),
        var_display_name: matches.is_present("var-display-names"),
    };

    let file = matches.value_of("FILE").unwrap();
    let content = fs::read_to_string(file).unwrap();

    let mut transpile = transpiler::Transpiler::new(content, transpiler_settings);
    let tokens = transpile.tokenize();

    let transpiled = transpile.transpile(tokens);
    println!("{}", transpiled);
}
