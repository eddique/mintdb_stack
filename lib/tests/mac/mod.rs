use mintdb_stack::{base64_dec, base64_enc, uuid_v4};

#[test]
fn it_creates_uuid_string() {
    let id = uuid_v4!();
    println!("{id}");
}

#[test]
fn it_base64_encodes() {
    let encoded = base64_enc!("Hello, world!".as_bytes().to_vec());
    println!("{encoded}");
}

#[test]
fn it_base64_decodes() {
    let text = "Hello, world";
    let encoded = base64_enc!(text.as_bytes().to_vec());
    let decoded = base64_dec!(encoded);
    let decoded = String::from_utf8(decoded).unwrap();
    assert_eq!(text, &decoded);
}