use crate::utils::error::ApiError;
use axum::{extract::Path, Json};
use proto_types::{
    blog::tag::{
        blog_tag_service_client::BlogTagServiceClient, BlogTag, BlogTagList, BlogTagShape,
    },
    common::db::{Id, Pagination},
    impls::DefaultState,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use vps_rpc::RPC_URL;

/// List
#[utoipa::path(
    post,
    path = "",
    request_body(
        content = Pagination,
        example = json!(Pagination::default_state())
    ),
    responses(
        (status = OK, description = "Success", body = BlogTagList)
    )
)]
async fn list(Json(pagination): Json<Pagination>) -> Result<Json<BlogTagList>, ApiError> {
    let mut client = BlogTagServiceClient::connect(RPC_URL).await?;
    let res = client.list(pagination).await?;
    Ok(Json(res.into_inner()))
}

#[utoipa::path(
    post,
    path = "/create",
    request_body = BlogTagShape,
    responses(
        (status = OK, description = "Success", body = BlogTag)
    )
)]
async fn create(Json(payload): Json<BlogTagShape>) -> Result<Json<BlogTag>, ApiError> {
    let mut client = BlogTagServiceClient::connect(RPC_URL).await?;
    let res = client.create(payload).await?;
    Ok(Json(res.into_inner()))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Tag id")   
    ),
    responses(
        (status = OK, description = "Success", body = BlogTag)
    )
)]
async fn get_by_id(Path(id): Path<Id>) -> Result<Json<BlogTag>, ApiError> {
    let mut client = BlogTagServiceClient::connect(RPC_URL).await?;
    let res = client.get_by_id(id).await?;
    Ok(Json(res.into_inner()))
}

#[utoipa::path(
    patch,
    path = "/{id}",
    request_body = BlogTag,
    responses(
        (status = OK, description = "Success", body = BlogTag)
    )
)]
async fn update(
    Path(Id { id }): Path<Id>,
    Json(payload): Json<BlogTag>,
) -> Result<Json<BlogTag>, ApiError> {
    let mut client = BlogTagServiceClient::connect(RPC_URL).await?;
    let mut req = payload.clone();
    req.id = id;

    let res = client.update(payload).await?;
    Ok(Json(res.into_inner()))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    params(
        ("id" = String, Path, description = "Tag id")   
    ),
    responses(
        (status = OK, description = "Success", body = Id)
    )
)]
async fn delete(Path(id): Path<Id>) -> Result<Json<Id>, ApiError> {
    let mut client = BlogTagServiceClient::connect(RPC_URL).await?;
    let res = client.delete(id).await?;
    Ok(Json(res.into_inner()))
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list)) // /
        .routes(routes!(create)) // /create
        .routes(routes!(get_by_id, update, delete)) // /{id}
}
