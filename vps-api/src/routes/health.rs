use axum::Json;
use load_env::{EnvSchema, schema::RpcTarget::Cron};
use proto_types::service::{HealthResponse, health_service_client::HealthServiceClient};
use utoipa_axum::{router::OpenApiRouter, routes};
use vps_rpc::RPC_URL;

use crate::utils::error::ApiError;

/// Get health of the API server.
#[utoipa::path(
    method(get),
    path = "/api",
    responses(
        (status = OK, description = "Success", body = HealthResponse)
    )
)]
async fn health_api() -> Result<Json<HealthResponse>, ApiError> {
    Ok(Json(HealthResponse {
        response: "ok from api server".into(),
    }))
}

/// Get health of the RPC server.
#[utoipa::path(
    method(get),
    path = "/rpc",
    responses(
        (status = OK, description = "Success", body = HealthResponse)
    )
)]
async fn health_rpc() -> Result<Json<HealthResponse>, ApiError> {
    let mut client = HealthServiceClient::connect(RPC_URL).await?;
    let res = client.health(()).await?.into_inner();
    Ok(Json(res))
}

/// Get health of the cron worker.
#[utoipa::path(
    method(get),
    path = "/cron",
    responses(
        (status = OK, description = "Success", body = HealthResponse)
    )
)]
async fn health_cron() -> Result<Json<HealthResponse>, ApiError> {
    let env = EnvSchema::load().expect("loading env");
    let mut client = HealthServiceClient::connect(env.rpc.url(&Cron)).await?;
    let res = client.health(()).await?.into_inner();
    Ok(Json(res))
}

/// expose the Customer OpenAPI to parent module
pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(health_api))
        .routes(routes!(health_rpc))
        .routes(routes!(health_cron))
}
