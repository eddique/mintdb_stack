use super::config::{Config, CF};
use crate::db;
use crate::log;
use crate::net;
use clap::Args;

pub async fn init(
    StartCommandArgs {
        path,
        username,
        password,
        crt,
        key,
        log,
        log_level,
    }: StartCommandArgs,
) -> anyhow::Result<()> {
    let _ = CF.set(Config {
        path,
        username,
        password,
        crt,
        key,
		log,
		log_level,
    });
	let _guard = log::init().await?;
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
    #[arg(short = 'c', long = "cert", visible_alias = "cert", requires = "key")]
    crt: Option<String>,
    #[arg(short = 'k', long = "key", visible_alias = "key")]
    key: Option<String>,

    #[arg(help = "Path used for storing log data")]
	#[arg(
        short = 'l',
        long = "log",
        visible_alias = "log",
    )]
    log: bool,

	#[arg(help = "Path used for storing log data")]
	#[arg(
        short = 'v',
        long = "log-level",
        visible_alias = "log-level",
    )]
    log_level: Option<String>,
}