mod config;
mod repl;
mod start;
use crate::db;
use crate::net;

use self::config::Config;
pub use config::CF;

pub async fn init() -> anyhow::Result<()> {

    start::init().await?;
    // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    // repl::start();
    Ok(())
}