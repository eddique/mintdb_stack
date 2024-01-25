use nalgebra::DVector;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{Datastore, Result, err::Error};
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Statement {
    Select,
    Insert,
    Delete,
    Drop,
    Migrate,
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
                    return Ok(res);
                }
                let res = self.get_many(&sql.tb).await?;
                Ok(json!(res))
                
            }
            Statement::Insert => {
                if let Some(data) = &sql.data {
                    if let Some(id) = &sql.doc {
                        return self.merge(&sql.tb, id, data).await;
                    }
                    let res = self.insert(&sql.tb, data).await;
                    return Ok(res)
                }
                Err(Error::MissingKey(format!("data")))
            }
            Statement::Delete => {
                if let Some(id) = &sql.doc {
                    return self.delete(&sql.tb, &id).await;
                }
                Err(Error::MissingKey(format!("doc")))
            }
            Statement::Drop => {
                self.drop(&sql.tb).await
            }
            Statement::Migrate => {
                self.create_collection(&sql.tb).await
            }
            Statement::Query => {
                if let Some(emb) = &sql.embedding {
                    let embedding = DVector::from_vec(emb.clone());
                    let res = self.query_vectors(&sql.tb, &embedding).await;
                    return Ok(json!(res));
                }
                Err(Error::MissingKey(format!("embedding")))
            }
        }
    }
}