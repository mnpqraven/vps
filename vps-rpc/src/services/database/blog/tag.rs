use crate::utils::{TonicResult, error::RpcError};
use database::table::blog_tag::BlogTagDb;
use proto_types::{
    blog::tag::{BlogTag, BlogTagList, BlogTagShape, blog_tag_service_server::BlogTagService},
    common::db::{Id, ProtoPagination},
};
use sqlx::{Pool, Postgres};
use tonic::{Request, Response};
use tracing::instrument;

#[derive(Debug)]
pub struct BlogTagRpc {
    pub conn: Pool<Postgres>,
}

// INFO: implement logic for rpc signatures
#[tonic::async_trait]
impl BlogTagService for BlogTagRpc {
    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn list(&self, request: Request<ProtoPagination>) -> TonicResult<BlogTagList> {
        let pagination = request.into_inner();
        let data = BlogTagDb::list(&self.conn, &pagination.into())
            .await
            .map_err(RpcError::db_with_context("uhh idk ???"))?;

        Ok(Response::new(BlogTagList {
            // TODO: correct this
            pagination: None,
            total: data.len() as i32,
            data,
        }))
    }

    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn get_by_id(&self, request: Request<Id>) -> TonicResult<BlogTag> {
        let id = &request.into_inner().id;
        let data = BlogTagDb::detail(&self.conn, id)
            .await
            .map_err(RpcError::db_with_context(id))?;
        Ok(Response::new(data))
    }

    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn create(&self, request: Request<BlogTagShape>) -> TonicResult<BlogTag> {
        let req = request.into_inner();
        let data = BlogTagDb::create(&self.conn, &req)
            .await
            .map_err(RpcError::db_with_context(&req.code))?;
        Ok(Response::new(data))
    }

    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn update(&self, request: Request<BlogTag>) -> TonicResult<BlogTag> {
        let req = &request.into_inner();
        let data = BlogTagDb::update(&self.conn, req)
            .await
            .map_err(RpcError::db_with_context(&req.id))?;
        Ok(Response::new(data))
    }

    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn delete(&self, request: Request<Id>) -> TonicResult<Id> {
        let req = &request.into_inner();
        let data = BlogTagDb::delete(&self.conn, &req.id)
            .await
            .map_err(RpcError::db_with_context(&req.id))?;
        Ok(Response::new(data))
    }
}
