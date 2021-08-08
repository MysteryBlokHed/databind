/*
 * Databind - Expand the functionality of Minecraft Datapacks.
 * Copyright (C) 2021  Adam Thompson-Sharpe
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
use clap::{App, Arg, SubCommand};

/// Set up Clap CLI and get arguments
pub fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Databind")
        .setting(clap::AppSettings::SubcommandsNegateReqs)
        .version("0.6.1")
        .author("Adam Thompson-Sharpe <adamthompsonsharpe@gmail.com>")
        .about("Expand the functionality of Minecraft Datapacks.")
        .arg(
            Arg::with_name("project")
                .help("The Databind project to compile")
                .required(true)
                .value_name("PROJECT"),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Configuration for the compiler")
                .takes_value(true)
                .value_name("FILE"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("out")
                .help("The output file or directory")
                .takes_value(true)
                .default_value("out")
                .value_name("DIRECTORY"),
        )
        .arg(
            Arg::with_name("ignore-config")
                .long("ignore-config")
                .help("Ignore the config file. Used for testing"),
        )
        .subcommand(
            SubCommand::with_name("create")
                .about("Create a new project")
                .arg(
                    Arg::with_name("name")
                        .help("The name of the project")
                        .required(true)
                        .value_name("NAME"),
                )
                .arg(
                    Arg::with_name("description")
                        .help("The pack description")
                        .default_value("A databind pack")
                        .long("description")
                        .alias("desc")
                        .takes_value(true)
                        .value_name("DESCRIPTION"),
                )
                .arg(
                    Arg::with_name("path")
                        .help("The path to create the pack in")
                        .long("path")
                        .takes_value(true)
                        .value_name("PATH"),
                )
                .arg(
                    Arg::with_name("version")
                        .help("The pack_format for the pack.mcmeta file")
                        .long("version")
                        .takes_value(true)
                        .default_value("7")
                        .value_name("VERSION"),
                ),
        )
}
