use clap::{App, Arg};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

mod settings;
mod token;
mod transpiler;

/// Get namespace (name of folder containing `/functions`)
fn get_namespace(functions_path: &Path) -> &str {
    let namespace_folder = functions_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap();

    let namespace: &str;

    if namespace_folder.contains("/") {
        namespace = namespace_folder.split('/').last().unwrap();
    } else {
        namespace = namespace_folder.split('\\').last().unwrap();
    }

    namespace
}

fn create_func_json(functions_path: &Path, func_name: &str) -> String {
    format!(
        "{{\"values\": [\"{}:{}\"]}}",
        get_namespace(functions_path),
        func_name
    )
}

fn main() -> std::io::Result<()> {
    let matches = App::new("Databind")
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
            Arg::with_name("generate-func-json")
                .long("generate-func-json")
                .help("Generate JSON files for functions in minecraft/tags/functions"),
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
        .get_matches();

    let config_path_str: String;

    if matches.is_present("config") {
        config_path_str = matches.value_of("config").unwrap().to_string();

        if fs::metadata(&config_path_str).is_err() {
            println!("Non-existant config file specified.");
            std::process::exit(1);
        }
    } else {
        config_path_str = String::from("databind.toml");
    }

    let config_path = Path::new(&config_path_str);
    if config_path.is_dir() {
        println!("Directory provided for config file.");
        std::process::exit(1);
    }

    let mut transpiler_settings: settings::Settings;

    if config_path.exists() && !matches.is_present("ignore-config") {
        let config_contents = fs::read_to_string(&config_path)?;
        transpiler_settings = toml::from_str(&config_contents[..]).unwrap();

        // Override config settings with CLI arguments if passed
        if matches.is_present("random-var-names") {
            transpiler_settings.random_var_names = true;
        }
        if matches.is_present("var-display-names") {
            transpiler_settings.var_display_names = true;
        }
        if matches.is_present("generate-func-json") {
            transpiler_settings.generate_func_json = true;
        }
    } else {
        transpiler_settings = settings::Settings {
            random_var_names: matches.is_present("random-var-names"),
            var_display_names: matches.is_present("var-display-names"),
            generate_func_json: matches.is_present("generate-func-json"),
            func_json_exclusions: Vec::new(),
        }
    }

    let datapack = matches.value_of("DATAPACK").unwrap();
    let datapack_is_dir = fs::metadata(datapack)?.is_dir();

    if datapack_is_dir {
        let mut var_map: HashMap<String, String> = HashMap::new();
        let mut target_folder = datapack.to_string();
        target_folder.push_str(".databind");

        if fs::metadata(&target_folder).is_ok() {
            println!("Deleting old databind folder...");
            fs::remove_dir_all(&target_folder)?;
            println!("Done.");
        }

        for entry in WalkDir::new(&datapack).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                let new_path_str = entry.path().to_str().unwrap().replace(datapack, "");
                let path = Path::new(&new_path_str);

                let mut target_path: String = target_folder.to_string();
                target_path.push('/');
                target_path.push_str(&format!("{}", path.parent().unwrap().to_str().unwrap())[..]);

                fs::create_dir_all(&target_path)?;

                if entry.path().extension().unwrap() == "databind" {
                    let content = fs::read_to_string(entry.path())
                        .expect(&format!("Failed to read file {}", entry.path().display())[..]);
                    let mut transpile = transpiler::Transpiler::new(content, &transpiler_settings);
                    let tokens = transpile.tokenize();
                    let transpiled = transpile.transpile(
                        tokens,
                        Some(get_namespace(entry.path())),
                        Some(&var_map),
                        true,
                        true,
                    );

                    if let transpiler::TranspileReturn::MultiFileAndMap(
                        files,
                        filename_to_index,
                        vars,
                    ) = transpiled
                    {
                        var_map = vars;

                        for (key, value) in filename_to_index.iter() {
                            if key == "" {
                                let filename_no_ext = path.file_stem().unwrap().to_str().unwrap();
                                let full_path =
                                    format!("{}/{}.mcfunction", target_path, filename_no_ext);

                                // Create <function>.json if it does not exist and if it is not in exclusions
                                if transpiler_settings.generate_func_json
                                    && !transpiler_settings
                                        .func_json_exclusions
                                        .contains(&filename_no_ext.to_string())
                                {
                                    let json_path_str = format!(
                                        "{}/data/minecraft/tags/functions/{}.json",
                                        datapack, filename_no_ext
                                    );
                                    let json_path = Path::new(&json_path_str);

                                    if !json_path.exists() {
                                        let new_json_path_str = format!(
                                            "{}/data/minecraft/tags/functions/{}.json",
                                            target_folder, filename_no_ext
                                        );
                                        let new_json_path = Path::new(&new_json_path_str);

                                        fs::create_dir_all(&new_json_path.parent().unwrap())?;
                                        fs::write(
                                            new_json_path,
                                            create_func_json(path, filename_no_ext),
                                        )?;
                                    }
                                }

                                fs::write(full_path, &files[0])?;
                                continue;
                            }

                            let full_path = format!("{}/{}.mcfunction", target_path, key);

                            // Create <function>.json if it does not exist and if it is not in exclusions
                            if transpiler_settings.generate_func_json
                                && !transpiler_settings.func_json_exclusions.contains(key)
                            {
                                let json_path_str = format!(
                                    "{}/data/minecraft/tags/functions/{}.json",
                                    datapack, key
                                );
                                let json_path = Path::new(&json_path_str);

                                // Create <function>.json if it does not exist
                                if !json_path.exists() {
                                    let new_json_path_str = format!(
                                        "{}/data/minecraft/tags/functions/{}.json",
                                        target_folder, key,
                                    );
                                    let new_json_path = Path::new(&new_json_path_str);

                                    fs::create_dir_all(&new_json_path.parent().unwrap())?;
                                    fs::write(new_json_path, create_func_json(path, key))?;
                                }
                            }

                            fs::write(full_path, &files[*value])?;
                        }
                    }
                } else {
                    let filename = path.file_name().unwrap().to_str().unwrap();
                    let full_path = format!("{}/{}", target_path, filename);
                    fs::copy(entry.path(), full_path)?;
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
        let transpiled = transpile.transpile(tokens, None, None, false, false);

        if let transpiler::TranspileReturn::SingleContents(contents) = transpiled {
            fs::write("databind-out.mcfunction", contents)?;
        } else {
            panic!("transpile() returned an incorrect enum");
        }
    }

    Ok(())
}
