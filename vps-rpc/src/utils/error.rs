use thiserror::Error;
use tonic::Status;

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("database connection error")]
    DatabaseConnectionError,

    #[error("{}", .context)]
    DbError {
        context: String,
        source: database::DbError,
    },
}

impl RpcError {
    pub fn db_with_context(
        context: impl Into<String>,
    ) -> impl FnOnce(database::DbError) -> RpcError {
        |source: database::DbError| Self::DbError {
            context: context.into(),
            source,
        }
    }
}

#[allow(unreachable_patterns)]
impl From<RpcError> for Status {
    fn from(val: RpcError) -> Self {
        match val {
            RpcError::DatabaseConnectionError => {
                tonic::Status::resource_exhausted("bad database connection".to_string())
            }
            RpcError::DbError { context, source } => {
                // FIXME: all error are 404 for now, need to split cases
                let fmt = context.to_string();
                tracing::error!("{fmt}\n{}", source.to_string());
                // TODO:
                tonic::Status::unknown(fmt)
            }
            _ => {
                tracing::error!("{val:?}");
                tonic::Status::internal("Unknown rpc error")
            }
        }
    }
}
