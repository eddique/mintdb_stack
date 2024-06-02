use super::config::{Config, CF};
use crate::db;
use crate::log;
use crate::net;
use crate::wal;
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
        pod,
    }: StartCommandArgs,
) -> anyhow::Result<()> {
    let wal = if let Some(pod_name) = pod {
        pod_name.contains("-0")
    } else { false };
    let _ = CF.set(Config {
        path,
        username,
        password,
        crt,
        key,
		log,
		log_level,
        wal,
    });
	let _guard = log::init().await?;
    wal::init().await?;
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

    #[arg(help = "Boolean used for storing log data to file")]
	#[arg(
        short = 'l',
        long = "log",
        visible_alias = "log",
    )]
    log: bool,

	#[arg(help = "Log level for default")]
	#[arg(
        short = 'L',
        long = "log-level",
        visible_alias = "log-level",
    )]
    log_level: Option<String>,
    #[arg(help = "Pod name for WAL")]
	#[arg(
        short = 'N',
        long = "pod",
        visible_alias = "pod-name",
    )]
    pod: Option<String>,
}