extern crate catlang;

use catlang::logger;
use console::Emoji;
use human_panic::setup_panic;
use indicatif::{HumanBytes, HumanDuration};
use log::info;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Command {
    /// initialize a catlang project
    Init {},
    /// build a catlang project
    Build {
        /// output binary name
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
        /// optimization level
        #[structopt(short = "O", default_value = "2", possible_values = &["0", "1", "2", "3"])]
        optimization: u8,
        /// application entry point
        #[structopt(name = "INPUT", parse(from_os_str))]
        input: PathBuf,
    },
    /// auto-format a catlang project
    Fmt {},
    /// start the catlang language server
    #[structopt(name = "start-language-server")]
    LanguageServer {},
}

#[derive(StructOpt)]
#[structopt(name = "catlang")]
/// compiler for the catlang programming language
struct Opt {
    /// verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences), conflicts_with = "quiet")]
    pub verbose: u8,
    /// quiet output
    #[structopt(long)]
    pub quiet: bool,
    #[structopt(subcommand)]
    pub command: Command,
}

fn main() {
    setup_panic!();
    let started = Instant::now();

    let opt = Opt::from_args();

    if let Err(e) = run(&opt) {
        panic!("Application error: {}", e);
    }

    info!(
        "{} Done in {}",
        Emoji("✨ ", ":-)"),
        HumanDuration(started.elapsed())
    );
}

fn run(opt: &Opt) -> anyhow::Result<()> {
    let verbose_num = if opt.quiet { 0 } else { opt.verbose + 1 };
    let max_log_level = match verbose_num {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let _res = logger::init_with_max_level(max_log_level);

    match &opt.command {
        Command::Init {} => {
            info!("Initializing...");
        }
        Command::Build {
            output: _output,
            optimization: _optimization,
            input,
        } => {
            info!("Building...");
            let file_metadata = fs::metadata(input.clone())?;
            info!("File size: {}", HumanBytes(file_metadata.len()));
            let mut file = fs::File::open(input)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let parsed = catlang::syntax::parser::parse(&contents).unwrap();
            let body = parsed.body();
            info!("Body: {:?}", body);
            // let _out_fname = m.value_of("_output").unwrap_or("out.c");
            // catlang::syntax::codegen::llvm::codegen(main_block, out_fname);
        }
        Command::Fmt {} => {
            info!("Formatting...");
        }
        Command::LanguageServer {} => {
            catlang::language_server::LanguageServer::run()?;
        }
    }
    Ok(())
}
