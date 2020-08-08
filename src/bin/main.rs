extern crate catlang;

use catlang::logger;
use clap::crate_version;
use clap::{App, Arg, ArgMatches, SubCommand};
use console::Emoji;
use human_panic::setup_panic;
use indicatif::{HumanBytes, HumanDuration};
use log::{info, warn};
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
        )
        .arg(
            Arg::with_name("quiet")
                .long("quiet")
                .help("Quiet output")
                .global(true),
        )
        .subcommand(SubCommand::with_name("init").about("initialize a new catlang project"))
        .subcommand(
            SubCommand::with_name("build")
                .about("build an executable")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .default_value("main")
                        .help("output binary name"),
                )
                .arg(
                    Arg::with_name("INPUT")
                        .required(true)
                        .index(1)
                        .help("application entry point"),
                )
                .arg(
                    Arg::with_name("optimization")
                        .short("O")
                        .default_value("2")
                        .possible_values(&["0", "1", "2", "3"])
                        .help("LLVM optimization level"),
                ),
        )
        .subcommand(SubCommand::with_name("fmt").about("format catlang code"))
        .subcommand(
            SubCommand::with_name("start-language-server")
                .about("start the catlang language server"),
        )
        .get_matches();

    if let Err(e) = run(&matches) {
        panic!("Application error: {}", e);
    }

    info!(
        "{} Done in {}",
        Emoji("✨ ", ":-)"),
        HumanDuration(started.elapsed())
    );
}

fn run(matches: &ArgMatches) -> std::io::Result<()> {
    let verbose_num = if matches.is_present("quiet") {
        0
    } else {
        matches.occurrences_of("verbose") + 1
    };
    let max_log_level = match verbose_num {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let _res = logger::init_with_max_level(max_log_level);
    println!("Log level: {}", max_log_level);

    match matches.subcommand() {
        ("init", Some(_m)) => {
            info!("Initializing...");
        }
        ("build", Some(m)) => {
            info!("Building...");
            let fname = m.value_of("INPUT").unwrap_or("main.cat");
            let file_metadata = fs::metadata(fname)?;
            info!("File size: {}", HumanBytes(file_metadata.len()));
            let mut file = fs::File::open(fname)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let parsed = catlang::syntax::parser::parse(&contents).unwrap();
            let body = parsed.body();
            info!("Body: {:?}", body);
            // let mut context = catlang::syntax::context::Context::new();
            // let mut main_block =
            // catlang::syntax::parser::Parser::parse_file(fname, &contents, &mut context);
            let _out_fname = m.value_of("output").unwrap_or("out.c");
            // catlang::syntax::codegen::llvm::codegen(main_block, out_fname);
        }
        ("fmt", Some(_m)) => {
            info!("Formatting...");
        }
        ("start-language-server", Some(_m)) => {
            // TODO: Better error handling
            catlang::language_server::LanguageServer::run().unwrap();
        }
        _ => warn!("Unrecognized subcommand"),
    }
    Ok(())
}
