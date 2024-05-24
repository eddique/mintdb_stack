use crate::Datastore;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use tokio::io::AsyncReadExt;

impl Datastore {
    pub(crate) async fn read_dir(&self, path: &str) {
        let path = format!("{path}/ds");
        self.mk_dir(&path).await;
        let mut entries = tokio::fs::read_dir(&path).await.unwrap();
        let mut collections = vec![];
        while let Some(entry) = entries.next_entry().await.unwrap() {
            match entry.file_name().into_string() {
                Ok(name) => collections.push(name),
                Err(_) => continue,
            }
        }
        for idx in collections {
            self.load_table(&path, &idx).await;
        }
    }
    pub(crate) async fn load_table(&self, path: &str, idx: &str) {
        let path = format!("{path}/{idx}");
        self.mk_dir(&path).await;
        if let Ok(mut entries) = tokio::fs::read_dir(path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path_buf = entry.path();
                let mut file = match tokio::fs::File::open(&path_buf).await {
                    Ok(f) => f,
                    Err(_) => tokio::fs::File::create(&path_buf).await.unwrap(),
                };
                if let Some(id) = path_buf.file_stem() {
                    let id = id.to_str().unwrap_or("default");
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer).await.unwrap();
                    let doc: BTreeMap<String, Value> =
                        serde_cbor::from_slice(&buffer).unwrap_or_default();
                    self.insert(idx, &json!(doc)).await;
                }
            }
        }
    }
    pub(crate) async fn mk_dir(&self, path: &str) {
        _ = tokio::fs::create_dir_all(path).await;
    }
}
