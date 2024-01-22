use std::sync::Arc;

use mintdb_stack::Datastore;
use once_cell::sync::OnceCell;
use crate::cli::CF;
pub static DS: OnceCell<Arc<Datastore>> = OnceCell::new();

pub async fn init() -> anyhow::Result<()> {
    // let dbs = Datastore::new().await;
    let config = CF.get().unwrap();
    let dbs = Datastore::init(&config.path).await;
    let _ = DS.set(Arc::new(dbs));
    Ok(())
}