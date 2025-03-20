use crate::handler::error::ApiError;
use utoipa_axum::{router::OpenApiRouter, routes};
use vps_rpc::{
    services::greeter::{greeter_client::GreeterClient, HelloRequest},
    RPC_ADDR,
};

/// Simple greeter communication with the rpc client
#[utoipa::path(
    method(get),
    path = "",
    responses(
        (status = OK, description = "Success", body = str, content_type = "text/plain")
    )
)]
async fn rpcgreet() -> Result<String, ApiError> {
    let addr = format!("grpc://{RPC_ADDR}");
    let mut client = GreeterClient::connect(addr)
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
