use crate::cli;
use crate::cnf;

pub async fn run() -> anyhow::Result<()> {
    // tracing_subscriber::fmt::init();
	println!("\x1b[38;5;50m{}\x1b[0m", cnf::LOGO);
    cli::init().await?;
    Ok(())
}