extern crate ansi_term;
extern crate clap;
#[macro_use]
extern crate log;
extern crate loggerv;

pub mod token;
pub mod lexer;

use clap::{App, Arg, SubCommand};

fn main() {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    const VERSION: &str = "0.0.1";
    const AUTHOR: &str = "Alic Szecsei <aszecsei@gmail.com>";
    let matches = App::new("catlang")
        .version(VERSION)
        .author(AUTHOR)
        .about("Compiler for the Catlang programming language.")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("sets the level of verbosity"),
        ).subcommand(SubCommand::with_name("init").about("initialize a new catlang project"))
        .subcommand(
            SubCommand::with_name("build")
                .about("build an executable")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .default_value("main")
                        .help("output binary name"),
                ).arg(
                    Arg::with_name("INPUT")
                        .required(true)
                        .index(1)
                        .help("application entry point"),
                ).arg(
                    Arg::with_name("optimization")
                        .short("O")
                        .default_value("0")
                        .possible_values(&["0", "1", "2", "3"])
                        .help("LLVM optimization level"),
                ),
        ).get_matches();

    loggerv::Logger::new()
        .verbosity(matches.occurrences_of("v"))
        .level(true)
        .line_numbers(false)
        .separator(" | ")
        .module_path(true)
        .colors(true)
        .init()
        .unwrap();

    if let Some(_matches) = matches.subcommand_matches("init") {
        info!("Initializing catlang project...");
    }

    if let Some(matches) = matches.subcommand_matches("build") {
        info!("Using input file: {}", matches.value_of("INPUT").unwrap());
        info!("Using output file: {}", matches.value_of("output").unwrap());
        info!(
            "Optimization level: {}",
            matches.value_of("optimization").unwrap()
        );
    }
}
