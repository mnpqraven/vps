use load_env::EnvError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CronDdnsError {
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("{0}")]
    Env(#[from] EnvError),
}
