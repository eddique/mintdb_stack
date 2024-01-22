use mintdb_stack::{encryption, hash, uuid_v4};
use mintdb_stack::base64_enc;
use mintdb_stack::Result;

#[test]
fn create_key() {
    let key_bytes = encryption::generate_key();
    let key = base64_enc!(&key_bytes.to_vec());
    println!("key: {key:?}");
}

#[test]
fn encrypt_str() -> Result<()> {
    let data = String::from("Hello, World!");
    let key_bytes = encryption::generate_key();
    let key = base64_enc!(&key_bytes.to_vec());
    let (enc_data, nonce) = encryption::encrypt(&data, &key)?;
    println!("encrypted_data: {enc_data} nonce: {nonce}");
    Ok(())
}

#[test]
fn decrypt_str() -> Result<()> {
    let key_bytes = encryption::generate_key();
    let key = base64_enc!(&key_bytes.to_vec());
    let data = String::from("Hello, World!");
    println!("data: {data}");
    let (enc_data, nonce) = encryption::encrypt(&data, &key)?;
    println!("encrypted_data: {enc_data}");
    let dec_data = encryption::decrypt(&enc_data, &nonce, &key)?;
    println!("decrypted_data: {dec_data}");
    assert_eq!(data, dec_data);
    Ok(())
}

#[test]
fn hash_key() {
    let key = uuid_v4!();
    println!("key: {key}");
    let hashed_key = hash::hash(&key);
    println!("hashed_key: {hashed_key}");
    let hashed_key_2 = hash::hash(&key);
    println!("hashed_key_2: {hashed_key_2}");
    assert_eq!(hashed_key, hashed_key_2);
}

#[test]
fn salt() {
    let key = uuid_v4!();
    println!("key: {key}");
    let (hashed_key, salt) = hash::hash_salt(&key, None);
    println!("salt: {salt}");
    println!("hashed_key: {hashed_key}");
    let (hashed_key_2, _) = hash::hash_salt(&key, Some(&salt));
    println!("hashed_key: {hashed_key_2}");
    assert_eq!(hashed_key, hashed_key_2);
}