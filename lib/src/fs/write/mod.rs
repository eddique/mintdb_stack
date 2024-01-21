use serde_json::Value;
use crate::Datastore;
use tokio::io::AsyncWriteExt;
impl Datastore {
    pub(crate) async fn write_document(&self, path: &str, idx: &str, id: &str, data: Value) {
        let idx_path = format!("{path}/ds/{idx}");
        let file_path = format!("{idx_path}/{id}.bin");
        self.mk_dir(&idx_path).await;
        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .await
            .unwrap();
        let data = serde_cbor::to_vec(&data).unwrap();
        match file.write_all(&data).await {
            Ok(_) => {}
            Err(_) => {}
        }
    }
    pub(crate) async fn delete_document(&self, path: &str, idx: &str, id: &str) {
        let path = format!("{path}/ds/{idx}/{id}");
        _ = tokio::fs::remove_file(&path).await.ok();
    }
}