mod repl;
use crate::db;
use crate::net;
pub async fn init() -> anyhow::Result<()> {
    db::init().await?;
    let srv = tokio::spawn(net::init());
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    repl::start();
    Ok(())
}