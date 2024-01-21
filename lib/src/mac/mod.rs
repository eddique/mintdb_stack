#[macro_export]
macro_rules! uuid_v4 {
    () => {
        uuid::Uuid::new_v4().to_string()
    };
}