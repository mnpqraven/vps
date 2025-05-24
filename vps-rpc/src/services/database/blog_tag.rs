use crate::{
    rpc::service::{
        tag_action_server::TagAction, Id, Pagination, TagDeleteResponse, TagListResponse, TagSchema,
    },
    utils::error::RpcError,
};
use database::get_db;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct TagRpc {}

// INFO: implement logic for rpc signatures
#[tonic::async_trait]
impl TagAction for TagRpc {
    async fn list(
        &self,
        request: Request<Pagination>,
    ) -> Result<Response<TagListResponse>, Status> {
        let Pagination {
            page_index,
            page_size,
        } = request.into_inner();
        let db = get_db().await.map_err(RpcError::DbError)?;
        tracing::info!("trying to connect");
        let data = sqlx::query_as!(
            TagSchema,
            "
                SELECT id, code, label
                FROM blog_tag
                LIMIT $1 OFFSET $2
        ",
            page_size as i64,
            (page_index * page_size) as i64
        )
        .fetch_all(&db)
        .await
        .unwrap();

        Ok(Response::new(TagListResponse {
            total: data.len() as i32,
            tags: data,
        }))
    }
    async fn get_by_id(&self, request: Request<Id>) -> Result<Response<TagSchema>, Status> {
        let Id { id } = request.into_inner();
        let db = get_db().await.map_err(RpcError::DbError)?;
        let data = sqlx::query_as!(
            TagSchema,
            "
                SELECT id, code, label                
                FROM blog_tag
                WHERE id = $1
            ",
            id,
        )
        .fetch_one(&db)
        .await
        .unwrap();

        Ok(Response::new(data))
    }
    async fn update(&self, request: Request<TagSchema>) -> Result<Response<TagSchema>, Status> {
        let req = request.into_inner();
        let TagSchema { id, code, label } = req.clone();
        let db = get_db().await.map_err(RpcError::DbError)?;

        let _data = sqlx::query!(
            "
                UPDATE blog_tag
                SET code = $2, label = $3
                WHERE id = $1
            ",
            id,
            code,
            label
        )
        .execute(&db)
        .await
        .unwrap();
        Ok(Response::new(req))
    }
    async fn delete(&self, request: Request<Id>) -> Result<Response<TagDeleteResponse>, Status> {
        let Id { id } = request.into_inner();
        let db = get_db().await.map_err(RpcError::DbError)?;

        let _data = sqlx::query!(
            "
                DELETE FROM blog_tag
                WHERE id = $1
            ",
            id
        )
        .execute(&db)
        .await
        .unwrap();

        Ok(Response::new(TagDeleteResponse { ids: vec![id] }))
    }
}
