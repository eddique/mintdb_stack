use super::config::{CF, Config};
use crate::db;
use crate::net;

pub async fn init() -> anyhow::Result<()> {
    let _ = CF.set(Config::default());
    db::init().await?;
    net::init().await.unwrap();
    Ok(())
}