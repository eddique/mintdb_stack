use std::sync::Arc;

use mintdb_stack::Datastore;
use once_cell::sync::OnceCell;

pub static DS: OnceCell<Arc<Datastore>> = OnceCell::new();

pub async fn init() -> anyhow::Result<()> {
    let dbs = Datastore::new().await;
    let _ = DS.set(Arc::new(dbs));
    Ok(())
}