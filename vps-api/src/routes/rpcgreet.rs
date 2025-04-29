use crate::handler::error::ApiError;
use utoipa_axum::{router::OpenApiRouter, routes};
use vps_rpc::services::greeter::{greeter_client::GreeterClient, HelloRequest};
use vps_rpc::RPC_URL;

/// Simple greeter communication with the rpc client
#[utoipa::path(
    method(get),
    path = "",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn rpcgreet() -> Result<String, ApiError> {
    let mut client = GreeterClient::connect(RPC_URL)
        .await
        .map_err(|e| ApiError::Unknown(e.into()))?;
    let said = client
        .say_hello(HelloRequest {
            name: "Othi@APICLIENT".into(),
        })
        .await
        .map_err(|e| ApiError::Unknown(e.into()))?;
    Ok(said.into_inner().message)
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(rpcgreet))
}
