use serde_json::{json, Value};
use std::{collections::BTreeMap, ops::Index, sync::Arc};
use tokio::sync::RwLock;

use crate::err::Error;
use crate::{Result, exe::Filter};
use crate::exe::parse_query;

pub type Document = BTreeMap<String, Value>;

pub type Collection = BTreeMap<String, Value>;
pub type Collections = BTreeMap<String, Collection>;

#[derive(Clone, Debug)]
struct Options {
    path: String,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            path: format!("mint.db"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Datastore {
    pub collections: Arc<RwLock<Collections>>,
    opt: Options,
}

impl Datastore {
    pub fn new(path: &str) -> Self {
        let opt = Options{
            path: format!("{path}"),
        };
        Self { opt, ..Default::default()  }
    }
    pub async fn init(path: &str) -> Self {
        let db = Self::new(path);

        if db.options().path != "memory" {
            db.read_dir(path).await;
        }
        db
    }
    fn options(&self) -> &Options {
        &self.opt
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
    pub async fn create_collection(&self, name: &str) -> Result<Value> {
        let collection = Collection::new();
        let mut lk = self.collections.write().await;
        lk.insert(name.to_string(), collection);
        Ok(json!({"name": name}))
    }
    pub async fn insert(&self, idx: &str, data: &Value) -> Value {
        let mut id = String::new();
        if let Some(Value::String(_id)) = data.get("id") {
            id = _id.into();
        } else {
            id = crate::uuid_v4!()
        }
        let mut data = data.clone();
        let data = if let Some(obj) = data.as_object_mut() {
                obj.insert(format!("id"), json!(id));
                json!(obj)
        } else {
            data
        };
        let mut lk = self.collections.write().await;
        if let Some(collection) = lk.get_mut(idx) {
            collection.insert(format!("{id}"), data.clone());
            drop(lk);

            // TODO: use stateful pattern?
            if self.opt.path != "memory" {
                self.write_document(&self.opt.path, idx, &id, data.clone()).await;
            }
            data.clone()
        } else {
            let collection = Collection::from([(format!("{id}"), data.clone())]);
            lk.insert(idx.to_string(), collection);
            if self.opt.path != "memory" {
                self.write_document(&self.opt.path, idx, &id, data.clone()).await;
            }
            data.clone()
        }
    }
    pub async fn insert_key(&self, idx: &str, id: &str, key: &str, data: &Value) -> Result<Value> {
        let mut lk = self.collections.write().await;
        if let Some(tb) = lk.get_mut(idx) {
            if let Some(doc) = tb.get_mut(id) {
                doc.as_object_mut()
                    .ok_or(Error::BadRequest(format!("invalid document type")))?
                    .insert(key.into(), data.clone());
                Ok(doc.clone())
            } else {
                Err(Error::NotFound(id.into()))
            }
        } else {
            Err(Error::NotFound(idx.into()))
        }
    }
    pub async fn get(&self, idx: &str, id: &str) -> Value {
        let lk = self.collections.read().await;
        if let Some(collection) = lk.get(idx) {
            // TODO: Fix id setting/ getting
            if let Some(doc) = collection.get(&format!("{id}")) {
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
                    // TODO: Fix id setting/ getting
                    let _ = doc.remove("embedding");
                    doc.insert(format!("id"), json!(key));
                    results.push(json!(doc));
                    continue;
                }
                results.push(value.clone());
            }
        }
        Ok(results)
    }
    pub async fn query(&self, idx: &str, query: &Vec<String>) -> Result<Vec<Value>> {
        let mut filters = vec![];
        for q in query {
            match parse_query(q) {
                Ok(filter) => {
                    filters.push(filter);
                }
                Err(e) => {
                    println!("{e}");
                    continue;
                }
            }
        }
        let lk = self.collections.read().await;
        let mut results = vec![];
        if let Some(tb) = lk.get(idx) {
            for (key, value) in tb {
                if let Some(mut doc) = value.clone().as_object_mut() {
                    let matches = filters.iter().any(|q| {
                        if let Some(val) = doc.get(&q.lhs) {
                            &q.rhs == val
                        } else {
                            false
                         }
                    });
                    if matches {
                        let _ = doc.remove("embedding");
                        results.push(json!(doc));
                        continue;
                    }
                }
            }
        }
        Ok(results)
    }
    pub async fn drop(&self, idx: &str) -> Result<Value> {
        let mut lk = self.collections.write().await;
        if let Some((id, entry)) = lk.remove_entry(idx) {
            return Ok(json!(id));
        }
        Ok(Value::Null)
    }
    pub async fn delete(&self, idx: &str, id: &str) -> Result<Value> {
        let mut lk = self.collections.write().await;
        if let Some(collection) = lk.get_mut(idx) {
            if let Some((key, entry)) = collection.remove_entry(id) {
                return Ok(entry);
                // TODO: use stateful pattern?
                if self.opt.path != "memory" {
                    self.delete_document(&self.opt.path, idx, id).await;
                }
            }
        }
        Ok(Value::Null)
    }
    pub async fn merge(&self, idx: &str, id: &str, data: &Value) -> Result<Value> {
        let mut lk = self.collections.write().await;
        if let Some(collection) = lk.get_mut(idx) {
            if let Some(doc) = collection.get_mut(&format!("{id}")) {
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
