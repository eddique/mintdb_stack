mod config;
mod repl;
mod start;
use crate::db;
use crate::net;
use clap::{Parser, Subcommand, Args};

use self::config::Config;
pub use config::CF;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Start(start::StartCommandArgs),
    Repl(repl::ReplCommandArgs)
}

pub async fn init() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start(args) => start::init(args).await?,
        Commands::Repl(args) => repl::init(args)?,
    }
    Ok(())
}