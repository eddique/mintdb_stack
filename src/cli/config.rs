use std::path::PathBuf;
use std::sync::OnceLock;

pub static CF: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub path: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub crt: Option<String>,
    pub key: Option<String>,
    pub log: bool,
    pub log_level: Option<String>,
    pub wal: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config { 
            path: format!("mint.db"), 
            username: Some(format!("mint.db")), 
            password: Some(format!("mint.db")), 
            crt: None, 
            key: None,
            log: false,
            log_level: None,
            wal: false,
        }
    }
}