use clap::{App, Arg};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;

mod settings;
mod token;
mod transpiler;

fn main() -> std::io::Result<()> {
    let matches = App::new("Databind")
        .version("0.1.0")
        .author("Adam Thompson-Sharpe <adamthompsonsharpe@gmail.com>")
        .about("A superset of mcfunctions for Minecraft Datapacks.")
        .arg(
            Arg::with_name("DATAPACK")
                .help("The datapack (or file) to transpile")
                .required(true),
        )
        .arg(
            Arg::with_name("random-var-names")
                .long("random-var-names")
                .help(
                    "Add characters to the end of variable names. \
                    Does not work when using variables across multiple files.",
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
        .get_matches();

    let transpiler_settings = settings::Settings {
        randomize_var_names: matches.is_present("random-var-names"),
        var_display_name: matches.is_present("var-display-names"),
    };

    let datapack = matches.value_of("DATAPACK").unwrap();
    let datapack_is_dir = fs::metadata(datapack)?.is_dir();

    if datapack_is_dir {
        let mut var_map: HashMap<String, String> = HashMap::new();
        let mut target_folder = datapack.to_string();
        target_folder.push_str(".databind");

        for entry in WalkDir::new(&datapack).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                if entry.path().extension().unwrap() == "databind" {
                    let content = fs::read_to_string(entry.path())
                        .expect(&format!("Failed to read file {}", entry.path().display())[..]);
                    let mut transpile = transpiler::Transpiler::new(content, &transpiler_settings);
                    let tokens = transpile.tokenize();
                    let transpiled = transpile.transpile(tokens, Some(&var_map), true, true);

                    if let transpiler::TranspileReturn::MultiFileAndMap(
                        files,
                        filename_to_index,
                        vars,
                    ) = transpiled
                    {
                        var_map = vars;
                        let mut target_dir: String = target_folder.to_string();
                        target_dir.push('/');
                        target_dir
                            .push_str(&format!("{}", entry.path().parent().unwrap().display())[..]);

                        fs::create_dir_all(&target_dir)?;

                        for (key, value) in filename_to_index.iter() {
                            if key == "" {
                                let filename = entry.path().file_stem().unwrap().to_str().unwrap();
                                let full_path = format!("{}/{}.mcfunction", target_dir, filename);
                                println!("main full path {}", full_path);

                                fs::write(full_path, &files[0])?;
                                continue;
                            }

                            let full_path = format!("{}/{}.mcfunction", target_dir, key);
                            println!("func full path {}", full_path);

                            fs::write(full_path, &files[*value])?;
                        }
                    }
                }
            }
        }
    } else {
        let content = fs::read_to_string(datapack).unwrap();

        let mut transpile = transpiler::Transpiler::new(content, &transpiler_settings);
        let tokens = transpile.tokenize();
        if tokens.contains(&token::Token::DefineFunc) {
            println!("Cannot use functions in a lone file.");
            std::process::exit(1);
        }
        let transpiled = transpile.transpile(tokens, None, false, false);

        if let transpiler::TranspileReturn::SingleContents(contents) = transpiled {
            fs::write("databind-out.mcfunction", contents)?;
        } else {
            panic!("transpile() returned an incorrect enum");
        }
    }

    Ok(())
}
