use serde_json::Value;
use crate::Datastore;
use crate::err::{Error, Result};
use tokio::io::AsyncWriteExt;
impl Datastore {
    pub(crate) async fn write_document(&self, path: &str, idx: &str, id: &str, data: Value) -> Result<()> {
        if id.contains('/') {
            return Err(Error::BadRequest("document id contains invalid character '/'".to_string()));
        }
        let idx_path = format!("{path}/ds/{idx}");
        let file_path = format!("{idx_path}/{id}.bin");
        self.mk_dir(&idx_path).await;
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .await?;
        let data = serde_cbor::to_vec(&data)?;
        match file.write_all(&data).await {
            Ok(v) => Ok(()),
            Err(e) => Err(Error::BadRequest(e.to_string()))
        }
    }
    pub(crate) async fn delete_document(&self, path: &str, idx: &str, id: &str) {
        let path = format!("{path}/ds/{idx}/{id}");
        _ = tokio::fs::remove_file(&path).await.ok();
    }
}