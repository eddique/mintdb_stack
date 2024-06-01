pub type Result<R> = std::result::Result<R, Error>;

use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] tokio::io::Error),

    // #[error("io error: {0}")]
    // IOERR(#[from] std::io::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Bad config: {0}")]
    BadConfig(String),
    
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("'{0}' required for execution")]
    MissingKey(String),

    #[error("'{0}' is an invalid operation")]
    InvalidQuery(String),
    
    #[error("Not authorized: {0}")]
    NotAuthorized(String),
    
    #[error("Require Admin privileges for {0}")]
    RequireAdmin(String),
    
    #[error(transparent)]
    EnvironmentVar(#[from] std::env::VarError),
    
	#[error("GraphQL error fetching data")]
    GraphQL,

    #[error("Join error fetching data")]
    Join(#[from] tokio::task::JoinError),

    #[error("Encryption error: {0}")]
    Encrypt(String),

    #[error("Decryption error: {0}")]
    Decrypt(String),

    #[error("base64 decode error: {0}")]
    B64Decode(#[from]base64::DecodeError),

    #[error("from UTF-8 error: {0}")]
    UTF8(#[from] std::string::FromUtf8Error),

    #[error("from UTF-8 error: {0}")]
    CBOR(#[from] serde_cbor::error::Error),
}