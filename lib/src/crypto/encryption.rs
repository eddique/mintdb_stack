#![allow(unused)]
use base64::{engine::general_purpose, Engine as _};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, CHACHA20_POLY1305};
use ring::rand::{SecureRandom, SystemRandom};
use std::env;

use crate::err::{Result, Error};
use crate::{base64_dec, base64_enc};
const NONCE_LEN: usize = 96 / 8;
const KEY_LEN: usize = 256 / 8;

pub fn generate_key() -> [u8; KEY_LEN] {
    let rng = SystemRandom::new();
    let mut key = [0u8; KEY_LEN];
    rng.fill(&mut key).map_err(|_|Error::Encrypt("error generating key".to_string()));
    key
}

fn generate_nonce() -> Nonce {
    let rng = SystemRandom::new();
    let mut nonce = [0u8; NONCE_LEN];
    rng.fill(&mut nonce).map_err(|_|Error::Encrypt("error generating nonce".to_string()));
    let nonce = Nonce::assume_unique_for_key(nonce);
    nonce
}

pub fn encrypt(data: &str, encryption_key: &str) -> Result<(String, String)> {
    let key_bytes = base64_dec!(encryption_key);
    if key_bytes.len() != KEY_LEN {
        return Err(Error::Encrypt(format!("ENCRYPTION_KEY must be {} bytes long", KEY_LEN)))
    }
    let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &key_bytes)
        .map_err(|e| Error::Decrypt(e.to_string()))?;
    let less_safe_key = LessSafeKey::new(unbound_key);
    let mut in_out = data.as_bytes().to_vec();
    let nonce = generate_nonce();
    let b64_nonce =  base64_enc!(&nonce.as_ref().to_vec());
    let aad = Aad::empty();

    less_safe_key
        .seal_in_place_append_tag(nonce, aad, &mut in_out)
        .map_err(|e| Error::Encrypt(e.to_string()));

    Ok((base64_enc!(&in_out), b64_nonce))
}

pub fn decrypt(data: &str, nonce: &str, encryption_key: &str) -> Result<String> {
    let key_bytes = base64_dec!(encryption_key);

    if key_bytes.len() != KEY_LEN {
        return Err(Error::Encrypt(format!("ENCRYPTION_KEY must be {} bytes long", KEY_LEN)))
    }
    let unbound_key =
        UnboundKey::new(&CHACHA20_POLY1305, &key_bytes).map_err(|e| Error::Decrypt(e.to_string()))?;
    let less_safe_key = LessSafeKey::new(unbound_key);
    let mut in_out = base64_dec!(data);

    if in_out.len() < CHACHA20_POLY1305.tag_len() {
        panic!("Invalid input length");
    }
    let nonce_bytes = base64_dec!(nonce)
        .try_into()
        .map_err(|_| Error::Decrypt("error decrypting nonce".to_string()))?;
    let nonce = Nonce::assume_unique_for_key(nonce_bytes);
    let aad = Aad::empty();

    let decrypted_data = less_safe_key
        .open_in_place(nonce, aad, &mut in_out)
        .map_err(|e| Error::Decrypt(e.to_string()))?;

    let dec_str = String::from_utf8(decrypted_data.to_vec())?;
    Ok(dec_str)
}