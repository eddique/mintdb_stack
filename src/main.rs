#![allow(unused)]
mod app;
mod cli;
mod cnf;
mod db;
mod err;
mod net;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run().await?;
    Ok(())
}
