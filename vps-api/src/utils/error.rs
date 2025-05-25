use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("{}", .0.message())]
    RpcStatus(#[from] tonic::Status),

    #[error("")]
    ParseError(String),

    #[error("Bad gRPC connection: {0}")]
    RpcConnection(#[from] tonic::transport::Error),

    #[error("Unknown error: {0}")]
    Unknown(anyhow::Error),
}

impl ApiError {
    pub fn code(&self) -> StatusCode {
        match self {
            ApiError::RpcStatus(rpc_status) => {
                let req: Response<()> = rpc_status.clone().into_http();
                req.status()
            }
            ApiError::RpcConnection(_error) => StatusCode::SERVICE_UNAVAILABLE,
            ApiError::ParseError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let msg = self.to_string();
        tracing::error!(msg);

        let err_body = json!({ "error": msg });
        (self.code(), err_body.to_string()).into_response()
    }
}
