use async_recursion::async_recursion;
use futures::stream::FuturesOrdered;
use futures::StreamExt;
use nalgebra::DVector;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use std::{fmt::format, str::FromStr};

use crate::{err::Error, Datastore, Result};
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Statement {
    Select,
    Insert,
    Delete,
    Drop,
    Migrate,
    Query,
    Tables,
    Count,
    Batch,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Statement::Select => "SELECT",
            Statement::Insert => "INSERT",
            Statement::Delete => "DELETE",
            Statement::Drop => "DROP",
            Statement::Migrate => "MIGRATE",
            Statement::Query => "QUERY",
            Statement::Tables => "TABLES",
            Statement::Count => "COUNT",
            Statement::Batch => "BATCH",
        };
        write!(f, "{}", output)
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SQL {
    pub stmt: Statement,
    pub tb: String,
    pub doc: Option<String>,
    pub key: Option<String>,
    pub data: Option<Value>,
    pub query: Option<Vec<String>>,
    pub embedding: Option<Vec<f64>>,
    pub top_n: Option<usize>,
}
impl Datastore {
    #[async_recursion]
    pub async fn exec(&self, sql: &SQL) -> Result<Value> {
        match sql.stmt {
            Statement::Select => {
                if let Some(id) = &sql.doc {
                    let res = self.get(&sql.tb, &id).await;
                    return Ok(res);
                }
                if let Some(query) = &sql.query {
                    let res = self.query(&sql.tb, query).await?;
                    return Ok(json!(res));
                }
                let res = self.get_many(&sql.tb).await?;
                Ok(json!(res))
            }
            Statement::Insert => {
                if let Some(data) = &sql.data {
                    if let Some(id) = &sql.doc {
                        if let Some(key) = &sql.key {
                            return self.insert_key(&sql.tb, id, key, data).await;
                        }
                        return self.merge(&sql.tb, id, data).await;
                    }
                    return self.insert(&sql.tb, data).await;
                }
                Err(Error::MissingKey(format!("data")))
            }
            Statement::Delete => {
                if let Some(id) = &sql.doc {
                    return self.delete(&sql.tb, &id).await;
                }
                Err(Error::MissingKey(format!("doc")))
            }
            Statement::Drop => self.drop(&sql.tb).await,
            Statement::Migrate => self.create_collection(&sql.tb).await,
            Statement::Query => {
                if let Some(emb) = &sql.embedding {
                    let top_n: usize = sql.top_n.unwrap_or(5).into();
                    let embedding = DVector::from_vec(emb.clone());
                    let res = self.query_embeddings(&sql.tb, &embedding, top_n).await;
                    return Ok(json!(res));
                }
                Err(Error::MissingKey(format!("embedding")))
            }
            Statement::Tables => {
                let res = self.get_collections().await?;
                Ok(json!(res))
            }
            Statement::Count => {
                let res = self.count(&sql.tb).await?;
                Ok(json!(res))
            }
            Statement::Batch => {
                if let Some(data) = &sql.data {
                    if data.is_array() {
                        let stmts: Vec<SQL> = serde_json::from_value(data.clone())?;
                        let futures: Vec<_> = stmts.iter().map(|stmt| async {
                            let log = match self.exec(stmt).await {
                                Ok(v) => {
                                    format!("OK - {} {}", &stmt.stmt, &stmt.tb)
                                }
                                Err(e) => {
                                    format!("ERROR - {} {} log: {}", &stmt.stmt, &stmt.tb, e.to_string())
                                }
                            };
                            json!(log)
                        }).collect();
                        let mut stream = FuturesOrdered::new();
                        for future in futures {
                            stream.push_back(future);
                        }

                        let results: Vec<_> = stream.collect().await;

                        Ok(json!(results))
                    } else {
                        Err(Error::BadRequest(format!(
                            "data key must be and array of sql statements"
                        )))
                    }
                } else {
                    Err(Error::MissingKey(format!("data")))
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub lhs: String,
    pub op: Operation,
    pub rhs: Value,
}
#[derive(Debug, Deserialize)]
pub enum Operation {
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "<=")]
    LessThanOrEqual,
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "icontains")]
    IContains,
}

impl FromStr for Operation {
    type Err = crate::err::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "==" => Ok(Operation::Equal),
            "!=" => Ok(Operation::NotEqual),
            ">" => Ok(Operation::GreaterThan),
            ">=" => Ok(Operation::GreaterThanOrEqual),
            "<" => Ok(Operation::LessThan),
            "<=" => Ok(Operation::LessThanOrEqual),
            "contains" => Ok(Operation::Contains),
            "icontains" => Ok(Operation::IContains),
            _ => Err(Error::InvalidQuery(format!("{s}"))),
        }
    }
}

pub fn parse_query(query: &str) -> Result<Filter> {
    let parts: Vec<&str> = query.split_whitespace().collect();
    let key = parts.first().unwrap_or(&"");
    let op_str = parts.get(1).unwrap_or(&"unknown");
    let rhs = query.split(op_str).last().unwrap_or("default");
    let rhs = rhs.trim();
    println!("{rhs}");
    match Operation::from_str(op_str) {
        Ok(op) => {
            let filter = Filter {
                lhs: key.to_string(),
                op,
                rhs: Value::from_str(rhs).unwrap_or(Value::from(rhs)),
            };
            Ok(filter)
        }
        Err(e) => Err(e),
    }
}

fn compare_json(a: &Value, b: &Value) -> bool {
    a == b
}
