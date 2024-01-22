#[macro_export]
macro_rules! uuid_v4 {
    () => {
        uuid::Uuid::new_v4().to_string()
    };
}

#[macro_export]
macro_rules! base64_enc {
    ($input:expr) => {{
        use base64::{engine::general_purpose, Engine};
        let mut buf = String::new();
        general_purpose::STANDARD.encode_string($input, &mut buf);
        buf
    }};
}

#[macro_export]
macro_rules! base64_dec {
    ($input:expr) => {{
        use base64::{engine::general_purpose, Engine};
        general_purpose::STANDARD
            .decode($input)
            .unwrap_or_default()
    }};
}