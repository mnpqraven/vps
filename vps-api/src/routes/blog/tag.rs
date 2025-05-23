use crate::handler::error::ApiError;
use axum::Json;
use utoipa_axum::{router::OpenApiRouter, routes};
use vps_rpc::rpc::service::tag_action_client::TagActionClient;
use vps_rpc::rpc::service::{Id, Pagination, TagDeleteResponse, TagListResponse, TagSchema};
use vps_rpc::RPC_URL;

// INFO: API GATEWAY ------------------------------------------------------

// INFO: swagger stuff
#[utoipa::path(
    post,
    path = "",
    request_body = Pagination,
    responses(
        (status = OK, description = "Success", body = TagListResponse)
    )
)]
// INFO: this is json data from the frontend
async fn list(Json(pagination): Json<Pagination>) -> Result<Json<TagListResponse>, ApiError> {
    // INFO: talk to rpc
    let mut client = TagActionClient::connect(RPC_URL).await?;
    let res = client.list(pagination).await.unwrap();

    // INFO: this is data sent to the frontend
    Ok(Json(res.into_inner()))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
     ("id" = String, Path, description = "Tag id")   
    ),
    responses(
        (status = OK, description = "Success", body = TagSchema)
    )
)]
async fn get_by_id(Json(id): Json<Id>) -> Result<Json<TagSchema>, ApiError> {
    let mut client = TagActionClient::connect(RPC_URL).await?;
    let res = client.get_by_id(id).await?;
    dbg!(&res);

    Ok(Json(res.into_inner()))
}

#[utoipa::path(
    put,
    path = "",
    request_body = TagSchema,
    responses(
        (status = OK, description = "Success", body = TagSchema)
    )
)]
async fn update(Json(payload): Json<TagSchema>) -> Result<Json<TagSchema>, ApiError> {
    let mut client = TagActionClient::connect(RPC_URL).await?;
    let res = client.update(payload).await?;

    Ok(Json(res.into_inner()))
}

#[utoipa::path(
    delete,
    path = "",
    request_body = Id,
    responses(
        (status = OK, description = "Success", body = TagDeleteResponse)
    )
)]
async fn delete(Json(id): Json<Id>) -> Result<Json<TagDeleteResponse>, ApiError> {
    let mut client = TagActionClient::connect(RPC_URL).await?;
    let res = client.delete(id).await?;

    Ok(Json(res.into_inner()))
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list, get_by_id, update, delete))
}
