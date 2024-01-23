use std::io;
use std::io::Write;
use serde_json::Value;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct ReplCli {
    #[command(subcommand)]
    command: SQLCommands,
}

pub fn init(
    ReplCommandArgs { 
        path, 
        username, 
        password }: ReplCommandArgs
    ) -> anyhow::Result<()> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stdout(), "{err}")?;
                std::io::stdout().flush()?;
            }
        }
    }
    Ok(())
}

fn readline() -> anyhow::Result<String> {
    write!(std::io::stdout(), "\x1b[38;5;50m$ \x1b[0m")?;
    std::io::stdout().flush()?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)?;
    Ok(buffer)
}

fn respond(line: &str) -> anyhow::Result<bool>{
    let mut args = shlex::split(line).unwrap_or_default();
    args.insert(0, "repl".to_string());
    let app = ReplCli::try_parse_from(args)?;
    match app.command {
        SQLCommands::Insert(args) => {
            println!("args: {args:?}");
        }
        SQLCommands::Select(args) => {
            println!("args: {args:?}");
        }
        SQLCommands::Delete(args) => {
            println!("args: {args:?}");
        }
        SQLCommands::Quit => {
            println!("Shutting down.\x1b[38;5;50m Have a great day!\x1b[0m 😎");
            return Ok(true);
        }
        SQLCommands::Ping => {
            println!("  \x1b[38;5;50mpong\x1b[0m");
        }
    };

    Ok(false)
}

#[derive(Debug, Subcommand)]
pub enum SQLCommands {
    #[command()]
    Select(SelectArgs),
    #[command()]
    Insert(InsertArgs),
    #[command()]
    Delete(DeleteArgs),
    Quit,
    Ping,
}


#[derive(Args, Debug)]
pub struct ReplCommandArgs {
    #[arg(help = "Database path used for storing data")]
	#[arg(default_value = "mint.db")]
	path: String,
    #[arg(
		help = "The username for the initial database root user. Only if no other root user exists",
		help_heading = "Authentication"
	)]
	#[arg(
		short = 'u',
		long = "username",
		visible_alias = "user",
		requires = "password"
	)]
	username: Option<String>,
    #[arg(
		short = 'p',
		long = "password",
		visible_alias = "pass",
		requires = "username"
	)]
	password: Option<String>,
}

#[derive(Args, Debug)]
pub struct SelectArgs {
    #[arg(help = "Columns to return from the query")]
	columns: Vec<String>,
    #[arg(
		help = "Table to query",
		help_heading = "Table"
	)]
	#[arg(
		short = 't',
		long = "table",
		visible_alias = "tbl",
	)]
	table: Option<String>,
    #[arg(
		short = 'd',
		long = "document",
		visible_alias = "doc",
		requires = "table"
	)]
	document: Option<String>,
    #[arg(
		short = 'q',
		long = "query",
		visible_alias = "query",
	)]
	query: Vec<String>,
}
#[derive(Args, Debug)]
pub struct InsertArgs {
	#[arg(
		short = 't',
		long = "table",
		visible_alias = "table",
	)]
	table: String,
    #[arg(
		short = 'd',
		long = "data",
		visible_alias = "data",
        value_parser = parse_json
	)]
	data: Value,
    #[arg(
		short = 'q',
		long = "query",
		visible_alias = "query",
		requires = "query"
	)]
	query: Option<Vec<String>>,
}
#[derive(Args, Debug)]
pub struct DeleteArgs {
	#[arg(
		short = 't',
		long = "table",
		visible_alias = "table",
	)]
	table: String,
    #[arg(
		short = 'd',
		long = "document",
		visible_alias = "doc",
        requires = "table"
	)]
	document: String,
    #[arg(
		short = 'q',
		long = "query",
		visible_alias = "query",
		requires = "query"
	)]
	query: Option<Vec<String>>,
}
fn parse_json(s: &str) -> Result<Value, &'static str> {
    serde_json::from_str(s).map_err(|_| "Failed to parse JSON")
}