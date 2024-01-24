use nalgebra::DVector;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{Datastore, Result};
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Statement {
    Select,
    Insert,
    Delete,
    Query,
}

#[derive(Deserialize, Debug)]
pub struct SQL {
    pub stmt: Statement,
    pub tb: String,
    pub doc: Option<String>,
    pub data: Option<Value>,
    pub query: Option<Vec<String>>,
    pub embedding: Option<Vec<f64>>,
}
impl Datastore {
    pub async fn exec(&self, sql: &SQL) -> Result<Value> {
        match sql.stmt {
            Statement::Select => {
                if let Some(id) = &sql.doc {
                    let res = self.get(&sql.tb, &id).await;
                    return Ok(json!({"ok": true, "result": res}))
                }

                Ok(json!({"ok": false, "error": "id needed"}))
                
            }
            Statement::Insert => {
                if let Some(data) = &sql.data {
                    self.insert(&sql.tb, data).await;
                }
                Ok(json!({"ok": true}))
            }
            Statement::Delete => {
                if let Some(id) = &sql.doc {
                    self.delete(&sql.tb, &id).await;
                    return Ok(json!({"ok": true, "result": format!("{id} deleted")}));
                }
                Ok(json!({"ok": false, "error": "id needed"}))
            }
            Statement::Query => {
                if let Some(emb) = &sql.embedding {
                    let embedding = DVector::from_vec(emb.clone());
                    let res = self.query_vectors(&sql.tb, &embedding).await;
                    return Ok(json!({"ok": true, "results": res}));
                }
                Ok(json!({"ok": false, "error": "embedding needed"}))
            }
        }
    }
}