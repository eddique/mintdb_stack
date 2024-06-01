use crate::cli::CF;
use std::env;
use std::path::Path;
use tokio::fs::create_dir_all;
use tracing_subscriber::layer::SubscriberExt;

pub async fn init() -> anyhow::Result<Option<tracing_appender::non_blocking::WorkerGuard>> {
    let config = CF.get().unwrap();
    if let Some(log_level) = &config.log_level {
        env::set_var("RUST_LOG", log_level);
    }
    let filter = tracing_subscriber::EnvFilter::from_default_env();
    if config.log {
        let dir = format!("{}/log", &config.path);
        let path = Path::new(&dir);
        create_dir_all(path).await?;
        let file_appender = tracing_appender::rolling::hourly(&dir, "app.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        let file_subscriber = tracing_subscriber::fmt::Layer::new()
            .with_ansi(false)
            .with_writer(non_blocking);

        let std_subscriber = tracing_subscriber::fmt::Layer::new();

        let registry = tracing_subscriber::Registry::default()
            .with(file_subscriber)
            .with(std_subscriber)
            .with(filter);

        tracing::subscriber::set_global_default(registry)
            .expect("setting default subscriber failed");

        Ok(Some(guard))
    } else {
        tracing_subscriber::fmt().with_env_filter(filter).init();
        Ok(None)
    }
}
