use std::sync::Arc;

use mintdb_stack::Datastore;
use std::sync::OnceLock;
use crate::cli::CF;
pub static DS: OnceLock<Arc<Datastore>> = OnceLock::new();

pub async fn init() -> anyhow::Result<()> {
    let config = CF.get().unwrap();
    let dbs = Datastore::init(&config.path).await;
    let _ = DS.set(Arc::new(dbs));
    Ok(())
}