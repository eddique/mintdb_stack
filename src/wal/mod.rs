use crate::cli::CF;
use mintdb_stack::{Statement, SQL};
use tokio::fs::{create_dir_all, remove_file, File, OpenOptions};
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;

pub async fn init() -> anyhow::Result<()> {
    let config = CF.get().unwrap();
    if config.wal {
        tracing::info!("Creating wal.log for master node");
        let dir = format!("{}/wal", &config.path);
        create_dir_all(dir).await?;
    }
    Ok(())
}

pub async fn write_to_wal(entry: SQL) -> anyhow::Result<()> {
    let config = CF.get().unwrap();
    let path = format!("{}/wal/wal.log", &config.path);
    tracing::info!("Writing {} {} to wal.log", &entry.stmt, &entry.tb);
    if config.wal {
        let json_str = match &entry.stmt {
            Statement::Insert => serde_json::to_string(&entry)?,
            Statement::Delete => serde_json::to_string(&entry)?,
            Statement::Drop => serde_json::to_string(&entry)?,
            _ => return Ok(()),
        };
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(path)
            .await
        {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("error {e}");
                return Ok(());
            }
        };
        file.write_all(json_str.as_bytes()).await?;
        file.write_all(b"\n").await?;
    }
    Ok(())
}

pub async fn flush_wal() -> anyhow::Result<()> {
    let config = CF.get().unwrap();
    let path = format!("{}/wal/wal.log", &config.path);
    let file = File::open(&path).await?;

    let reader = tokio::io::BufReader::new(file);

    let mut wal_logs: Vec<SQL> = Vec::new();
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        let sql: SQL = serde_json::from_str(&line)?;
        wal_logs.push(sql);
    }
    tracing::info!("flushing wal");
    remove_file(path).await?;
    for log in wal_logs.drain(..) {
        tracing::info!("batching {} stmnt", &log.stmt);
    }

    Ok(())
}
