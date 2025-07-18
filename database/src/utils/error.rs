use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    /// a database connection cannot be made
    #[error("database connection error")]
    NoConnection,

    /// any other database error
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// bad env configuration that makes database unavailable
    #[error("bad env configuration {0}")]
    BadEnv(#[from] load_env::EnvError),
}
