use base64::{engine::general_purpose, Engine};


pub fn b64_enc(input: &Vec<u8>) -> String {
    let mut buf = String::new();
    general_purpose::STANDARD.encode_string(input, &mut buf);
    buf
}

pub fn b64_dec(input: &str) -> Vec<u8> {
    general_purpose::STANDARD
        .decode(input)
        .unwrap_or_default()
}