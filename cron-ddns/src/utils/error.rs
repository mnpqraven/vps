use thiserror::Error;

#[derive(Debug, Error)]
pub enum CronDdnsError {
    #[error("Unknown error: {0}")]
    Unknown(String),
}
