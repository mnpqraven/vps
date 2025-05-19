use schema::EnvSchema;
use utils::EnvError;

pub mod schema;
pub mod utils;

pub fn load_env() -> Result<EnvSchema, EnvError> {
    EnvSchema::new()
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::schema::EnvSchema;

    #[test]
    fn serde_works_no_file() {
        let parsed = EnvSchema::new().unwrap();
        assert_eq!(parsed.database.url, "http://127.0.0.1:4010".to_string());
    }

    #[test]
    fn finds_file() {
        let _config = r#"
            [database]
            url = "http://127.0.0.1:4010"
            auth_token = "anytoken"
        "#;
    }
}
