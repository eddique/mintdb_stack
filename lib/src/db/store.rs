use serde_json::{Value, json};
use std::{collections::BTreeMap, ops::Index, sync::Arc};
use tokio::sync::RwLock;

use crate::Result;

pub type Collection = BTreeMap<String, Value>;
pub type Collections = BTreeMap<String, Collection>;

#[derive(Clone, Debug, Default)]
pub struct Datastore {
    pub collections: Arc<RwLock<Collections>>,
}

impl Datastore {
    pub async fn new() -> Self {
        Self::default()
    }
    pub async fn init(path: &str) -> Self {
        let db = Self::default();
        db.read_dir(path).await;
        db
    }
    pub async fn create_collections(&self, collections: &Vec<String>) {
        let mut lk = self.collections.write().await;
        let mut idxs = Collections::new();
        for idx in collections {
            if !lk.contains_key(idx) {
                idxs.insert(idx.to_string(), Collection::new());
            }
        }
        lk.extend(idxs);
    }
    pub async fn create_collection(&self, name: &str) {
        let collection = Collection::new();
        let mut lk = self.collections.write().await;
        lk.insert(name.to_string(), collection);
    }
    pub async fn insert(&self, idx: &str, data: &Value) -> Value {
        let mut id = String::new();
        if let Some(Value::String(_id)) = data.get("id") {
            id = _id.into();
        } else {
            id = crate::uuid_v4!()
        }
        let mut lk = self.collections.write().await;
        if let Some(collection) = lk.get_mut(idx) {
            collection.insert(format!("{idx}:{id}"), data.clone());
            drop(lk);

            // TODO: if self.opt.path != "memory"
            self.write_document("mint.db", idx, &id, data.clone()).await;
            data.clone()
        } else {
            let collection = Collection::from([(format!("{idx}:{id}"), data.clone())]);
            lk.insert(idx.to_string(), collection);
            data.clone()
        }
    }
    pub async fn get(&self, idx: &str, id: &str) -> Value {
        let lk = self.collections.read().await;
        if let Some(collection) = lk.get(idx) {
            if let Some(doc) = collection.get(&format!("{idx}:{id}")) {
                if let Some(mut doc) = doc.clone().as_object_mut() {
                    let _ = doc.remove("embedding");
                    return json!(doc);
                }
                return doc.clone();
            }
        }
        Value::Null
    }
    pub async fn get_many(&self, idx: &str) -> Result<Vec<Value>> {
        let lk = self.collections.read().await;
        let mut results = vec![];
        if let Some(tb) = lk.get(idx) {
            for (key, value) in tb {
                if let Some(mut doc) = value.clone().as_object_mut() {
                    let _ = doc.remove("embedding");
                    results.push(json!(doc));
                    continue;
                }
                results.push(value.clone());
            }
        }
        Ok(results)
    }
    pub async fn drop(&self, idx: &str) {
        let mut lk = self.collections.write().await;
        lk.remove(idx);
    }
    pub async fn delete(&self, idx: &str, id: &str) -> Result<Value> {
        let mut lk = self.collections.write().await;
        if let Some(collection) = lk.get_mut(idx) {
            if let Some((key, entry)) = collection.remove_entry(id) {
                return Ok(entry);
                // TODO: if self.opts.path != "memory"
                self.delete_document("mint.db", idx, id).await;
            }
        }
        Ok(Value::Null)
    }
    pub async fn merge(&self, idx: &str, id: &str, data: &Value) -> Result<Value> {
        let mut lk = self.collections.write().await;
        if let Some(collection) = lk.get_mut(idx) {
            if let Some(doc) = collection.get_mut(&format!("{idx}:{id}")) {
                if let Some(document) = doc.as_object_mut() {
                    if let Some(data) = data.as_object() {
                        for (key, value) in data {
                            document.insert(key.to_string(), value.clone());
                        }
                        return Ok(json!(document));
                    }
                }
            }
        }
        Ok(Value::Null)
    }
}
