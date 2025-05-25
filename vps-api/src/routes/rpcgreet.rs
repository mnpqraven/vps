use crate::utils::error::ApiError;
use axum::Json;
use proto_types::{greeter_client::GreeterClient, HelloReply, HelloRequest};
use utoipa_axum::{router::OpenApiRouter, routes};
use vps_rpc::RPC_URL;

// TODO: reuse rpc struct
/// Simple greeter communication with the rpc client
#[utoipa::path(
    post,
    path = "",
    request_body = HelloRequest,
    responses(
        (status = OK, description = "Success", body = HelloReply)
    )
)]
async fn rpcgreet(
    Json(HelloRequest { name }): Json<HelloRequest>,
) -> Result<Json<HelloReply>, ApiError> {
    let mut client = GreeterClient::connect(RPC_URL).await?;
    let said = client.say_hello(HelloRequest { name }).await?;
    Ok(Json(said.into_inner()))
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(rpcgreet))
}
