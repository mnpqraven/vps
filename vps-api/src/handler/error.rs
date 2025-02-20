use std::fmt::Display;

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde_json::json;

/// general error type of the application, this should be used in junctions
/// between api handler methods (axum, vercel runtime etc.)
/// Within the application, for deep/small functions it is okay to use
/// `anyhow::Error` then convert to `WorkerError` by using
/// `Into<WorkerError>`
#[derive(Debug)]
pub enum ApiError {
    ParseData(String),
    WrongMethod,
    EmptyBody,
    ServerSide,
    NotFound(String),
    Unknown(anyhow::Error),
}

impl ApiError {
    pub fn code(&self) -> StatusCode {
        match self {
            ApiError::ParseData(_) => StatusCode::BAD_REQUEST,
            ApiError::EmptyBody => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::BAD_REQUEST,
            ApiError::WrongMethod => StatusCode::METHOD_NOT_ALLOWED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt = match self {
            Self::ParseData(reason) => format!("Incorrect Data\nReason: {}", reason),
            Self::NotFound(resource) => format!("Resrouce/ID {resource} not found"),
            Self::WrongMethod => "Method is not supported".to_owned(),
            Self::EmptyBody => "Missing body data".to_owned(),
            Self::Unknown(err) => format!("Unknown error: {}", err),
            Self::ServerSide => "Unknown server error".to_owned(),
        };
        write!(f, "{}", fmt)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let code = self.code();
        let msg = self.to_string();
        if let ApiError::Unknown(inner) = self {
            tracing::error!("stacktrace: {}", &inner.backtrace());
            inner.chain().for_each(|er| {
                tracing::error!("chain: {}", er);
            });
        } else {
            tracing::error!(msg);
        };
        let err_body = json!({"error": msg});
        (code, err_body.to_string()).into_response()
    }
}
