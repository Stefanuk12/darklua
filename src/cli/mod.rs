pub mod error;
pub mod minify;
pub mod process;
pub mod utils;

use clap::{Args, Parser, Subcommand};
use log::LevelFilter;

use self::error::CliError;

type CommandResult = Result<(), CliError>;

#[derive(Debug, Args, Clone)]
pub struct GlobalOptions {
    /// Sets verbosity level (can be specified multiple times)
    #[arg(long, short, global(true), action = clap::ArgAction::Count)]
    verbose: u8,
}

impl GlobalOptions {
    pub fn get_log_level_filter(&self) -> LevelFilter {
        match self.verbose {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Minify lua files without applying any transformation
    Minify(minify::Options),
    /// Process lua files with rules
    ///
    /// Configure the code transformation using a configuration file.
    /// If no configuration is passed, darklua will attempt to read
    /// `.darklua.json` or `darklua.json5` from the working directory.
    Process(process::Options),
}

impl Command {
    pub async fn run(&self, global_options: &GlobalOptions) -> CommandResult {
        match self {
            Command::Minify(options) => minify::run(options, global_options).await,
            Command::Process(options) => process::run(options, global_options).await,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "darklua", about, version, propagate_version = true)]
/// Transform Lua scripts
///
/// For specific help about each command, run `darklua <command> --help`
///
/// Site: https://darklua.com
pub struct Darklua {
    #[command(flatten)]
    global_options: GlobalOptions,
    #[command(subcommand)]
    command: Command,
}

impl Darklua {
    pub async fn run(&self) -> CommandResult {
        self.command.run(&self.global_options).await
    }

    pub fn get_log_level_filter(&self) -> LevelFilter {
        self.global_options.get_log_level_filter()
    }
}
