extern crate catlang;

use console::Emoji;
use human_panic::setup_panic;
use indicatif::{HumanBytes, HumanDuration};
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Instant;
use structopt::StructOpt;
use tracing::info;

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
        /// LLVM target triple
        #[structopt(long, default_value = &catlang::codegen::DEFAULT_TARGET_TRIPLE)]
        target: String,
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
    #[structopt(
        short,
        long,
        parse(from_occurrences),
        conflicts_with = "quiet",
        global = true
    )]
    pub verbose: u8,
    /// quiet output
    #[structopt(long, global = true)]
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
    let verbose_num = opt.verbose + 1;
    let max_log_level = match verbose_num {
        1 => tracing::Level::WARN,
        2 => tracing::Level::INFO,
        3 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    if !opt.quiet {
        tracing_subscriber::fmt()
            .with_max_level(max_log_level)
            .init();
    }

    match &opt.command {
        Command::Init {} => {
            info!("Initializing...");
        }
        Command::Build {
            output: _output,
            optimization,
            input,
            target,
        } => {
            info!("Building...");
            let file_metadata = fs::metadata(input.clone())?;
            info!("File size: {}", HumanBytes(file_metadata.len()));
            let mut file = fs::File::open(input)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            info!("Read file contents");
            let parsed = catlang::syntax::parser::parse(&contents).unwrap();
            info!("Parsed file");
            let body = parsed.body();
            info!("Body: {:?}", body);
            // let _out_fname = m.value_of("_output").unwrap_or("out.c");
            // catlang::syntax::codegen::llvm::codegen(main_block, out_fname);

            // codegen
            println!("{}", catlang::codegen::run(*optimization, target)?);
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
