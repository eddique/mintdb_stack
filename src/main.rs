#![allow(unused)]
mod app;
mod cli;
mod cnf;
mod db;
mod err;
mod log;
mod models;
mod net;
mod routes;
mod wal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run().await?;
    Ok(())
}
