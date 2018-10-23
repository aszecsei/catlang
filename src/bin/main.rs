extern crate catlang;
#[macro_use]
extern crate clap;
extern crate console;
#[macro_use]
extern crate human_panic;
extern crate indicatif;
#[macro_use]
extern crate log;

use catlang::lexer;
use catlang::logger;
use clap::{App, Arg, ArgMatches, SubCommand};
use console::Emoji;
use indicatif::{HumanBytes, HumanDuration};
use std::fs;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
    setup_panic!();
    let started = Instant::now();

    const AUTHOR: &str = "Alic Szecsei <aszecsei@gmail.com>";
    let matches = App::new("catlang")
        .version(crate_version!())
        .author(AUTHOR)
        .about("Compiler for the Catlang programming language.")
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("sets the level of verbosity")
                .global(true)
                .conflicts_with("quiet"),
        ).arg(
            Arg::with_name("quiet")
                .long("quiet")
                .help("Quiet output")
                .global(true),
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
                        .default_value("2")
                        .possible_values(&["0", "1", "2", "3"])
                        .help("LLVM optimization level"),
                ),
        ).get_matches();

    if let Err(e) = run(matches) {
        panic!("Application error: {}", e);
    }

    info!(
        "{} Done in {}",
        Emoji("✨ ", ":-)"),
        HumanDuration(started.elapsed())
    );
}

fn run(matches: ArgMatches) -> std::io::Result<()> {
    let verbose_num = match matches.is_present("quiet") {
        true => 0,
        false => matches.occurrences_of("verbose") + 1,
    };
    let max_log_level = match verbose_num {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        4 | _ => log::LevelFilter::Trace,
    };

    let _res = logger::init_with_max_level(max_log_level);
    println!("Log level: {}", max_log_level);

    if let Some(m) = matches.subcommand_matches("init") {
        info!("Initializing...");
    }

    if let Some(m) = matches.subcommand_matches("build") {
        info!("Building...");
        let fname = m.value_of("INPUT").unwrap_or("main.cat");

        let file_metadata = fs::metadata(fname)?;
        info!("File size: {}", HumanBytes(file_metadata.len()));

        let mut file = fs::File::open(fname)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut scanner = lexer::Scanner::new(fname, &contents);
        loop {
            match scanner.current_lexeme {
                None => break,
                Some(lexeme) => {
                    if lexeme.token == lexer::token::Token::EOF {
                        debug!("EOF");
                        break;
                    }
                    debug!("{:?}", lexeme.token);
                    scanner.advance();
                }
            }
        }
    }
    Ok(())
}
