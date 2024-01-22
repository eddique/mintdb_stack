#![allow(unused)]
use ring::digest;
use ring::rand::{SecureRandom, SystemRandom};
use super::b64::b64_enc;

pub fn generate_salt() -> String {
    let mut salt = [0u8; 32];
    let rng = ring::rand::SystemRandom::new();
    rng.fill(&mut salt).unwrap();
    b64_enc(&salt.to_vec())
}

pub fn hash_salt(key: &str, salt: Option<&str>) -> (String, String) {
    let salt = salt.unwrap_or(&generate_salt()).to_string();
    let mut ctx = digest::Context::new(&digest::SHA256);
    ctx.update(salt.as_bytes());
    ctx.update(key.as_bytes());
    let hashed_key = b64_enc(&ctx.finish().as_ref().to_vec());
    (hashed_key, salt)    
}

pub fn hash(key: &str) -> String {
    let mut ctx = digest::Context::new(&digest::SHA256);
    ctx.update(key.as_bytes());
    b64_enc(&ctx.finish().as_ref().to_vec())
}