use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrontendError {
    #[error("Unknown Error: {0}")]
    Unknown(String),
}
