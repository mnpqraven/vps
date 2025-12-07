use crate::utils::error::ApiError;
use axum::{Json, extract::Query};
use proto_types::{
    blog::meta::{BlogMetaList, blog_meta_service_client::BlogMetaServiceClient},
    common::db::ProtoPagination,
};
use utoipa_axum::{router::OpenApiRouter, routes};
use vps_rpc::RPC_URL;

#[utoipa::path(
    get,
    path = "",
    params(ProtoPagination),
    responses(
        (status = OK, description = "Success", body = BlogMetaList)
    )
)]
async fn list(Query(params): Query<ProtoPagination>) -> Result<Json<BlogMetaList>, ApiError> {
    let mut client = BlogMetaServiceClient::connect(RPC_URL).await?;
    let res = client.list(params).await?;
    Ok(Json(res.into_inner()))
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list))
}
