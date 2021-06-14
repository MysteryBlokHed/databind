use serde_derive::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

fn make_pack_mcmeta(description: String) -> Result<String, serde_json::Error> {
    #[derive(Serialize)]
    struct Pack {
        pack_format: u8,
        description: String,
    }

    #[derive(Serialize)]
    struct PackMcMeta {
        pack: Pack,
    }

    let pack = PackMcMeta {
        pack: Pack {
            pack_format: 6,
            description: description,
        },
    };

    serde_json::to_string(&pack)
}

fn dir_empty(path: &dyn AsRef<Path>) -> std::io::Result<bool> {
    Ok(path.as_ref().read_dir()?.next().is_none())
}

/// Handle the create subcommand
///
/// # Arguments
///
/// - `args` - Matches from the create subcommand
pub fn create_project(args: clap::ArgMatches) -> std::io::Result<()> {
    let name = args.value_of("name").unwrap();
    let description = args.value_of("description").unwrap().to_string();
    let base_path = if args.is_present("path") {
        args.value_of("path").unwrap()
    } else {
        name
    };

    let mut path = PathBuf::new();
    path.push("./");
    path.push(base_path);

    let metadata = fs::metadata(&path);

    if metadata.is_ok() {
        let unwrapped = metadata.unwrap();
        if unwrapped.is_file() || unwrapped.is_dir() && !dir_empty(&path)? {
            println!(
                "Path {} is an already existant file or non-empty folder",
                base_path
            );
            std::process::exit(1);
        }
    }

    path.push(format!("data/{}/functions", name));
    fs::create_dir_all(&path)?;

    // Create main.databind
    path.push("main.databind");
    fs::write(
        &path,
        "\
    :func main\n\
    :tag load\n\
    tellraw @a \"Hello, World!\"\n\
    :endfunc\n",
    )?;

    path.pop();
    path.pop();
    path.pop();
    path.pop();

    // Create pack.mcmeta
    let pack_mcmeta = make_pack_mcmeta(description)?;
    path.push("pack.mcmeta");
    fs::write(&path, pack_mcmeta)?;

    println!("Created project {} in {}", name, base_path);
    Ok(())
}
