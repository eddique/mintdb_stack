use std::path::PathBuf;
use std::sync::OnceLock;

pub static CF: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub path: String,
    pub user: Option<String>,
    pub pass: Option<String>,
    pub crt: Option<PathBuf>,
    pub key: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Config { 
            path: format!("mint.db"), 
            user: Some(format!("mint.db")), 
            pass: Some(format!("mint.db")), 
            crt: None, 
            key: None 
        }
    }
}