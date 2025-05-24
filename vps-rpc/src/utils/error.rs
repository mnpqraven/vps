use thiserror::Error;
use tonic::Status;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("database connection error")]
    DatabaseConnectionError,

    #[error(transparent)]
    DbError(#[from] database::DbError),
}

impl From<RpcError> for Status {
    fn from(val: RpcError) -> Self {
        match val {
            RpcError::DatabaseConnectionError => {
                tonic::Status::resource_exhausted("bad database connection".to_string())
            }
            RpcError::DbError(_database_error) => tonic::Status::internal("giga bad".to_string()),
        }
    }
}
