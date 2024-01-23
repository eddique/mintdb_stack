use std::io;
use std::io::Write;

use clap::Args;

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

pub fn init(
    ReplCommandArgs { 
        path, 
        username, 
        password }: ReplCommandArgs
    ) {
    loop {
        print!("\x1b[38;5;50m$ mintdb\x1b[0m ");
        io::stdout().flush().expect("Failed to flush stdo");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        println!("{input}");
        if input == "exit" {
            println!("shutting down");
            break;
        }
    }
}