use super::config::{CF, Config};
use crate::db;
use crate::net;
use clap::Args;

pub async fn init(
    StartCommandArgs { 
        path, 
        username, 
        password, 
        crt, 
        key }: StartCommandArgs
    ) -> anyhow::Result<()> {
		println!("path: {path}");
		panic!("");
    let _ = CF.set(Config {
        path,
        username,
        password,
        crt,
        key
    });
    db::init().await?;
    net::init().await.unwrap();
    Ok(())
}

#[derive(Args, Debug)]
pub struct StartCommandArgs {
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
    #[arg(
		short = 'c',
		long = "cert",
		visible_alias = "cert",
		requires = "key"
	)]
	crt: Option<String>,
    #[arg(
		short = 'k',
		long = "key",
		visible_alias = "key"
	)]
	key: Option<String>,
}