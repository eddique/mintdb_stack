mod config;
mod repl;
use crate::db;
use crate::net;

use self::config::Config;
pub async fn init() -> anyhow::Result<()> {
    let _ = config::CF.set(Config::default());
    db::init().await?;
    let srv = tokio::spawn(net::init());
    srv.await?;
    // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    // repl::start();
    Ok(())
}