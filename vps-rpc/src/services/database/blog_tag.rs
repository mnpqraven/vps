use database::table::blog_tag::BlogTagDb;
use proto_types::{
    blog::tag::{blog_tag_service_server::BlogTagService, BlogTag, BlogTagList, BlogTagShape},
    common::db::{Id, Pagination},
};
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};
use tracing::instrument;

use crate::utils::error::RpcError;

#[derive(Debug)]
pub struct BlogTagRpc {
    pub conn: Pool<Postgres>,
}

// INFO: implement logic for rpc signatures
#[tonic::async_trait]
impl BlogTagService for BlogTagRpc {
    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn list(&self, request: Request<Pagination>) -> Result<Response<BlogTagList>, Status> {
        let pagination = request.into_inner();
        let data = BlogTagDb::list(&self.conn, &pagination)
            .await
            .map_err(RpcError::db_with_context("uhh idk ???"))?;

        Ok(Response::new(BlogTagList {
            pagination: Some(pagination),
            total: data.len() as i32,
            data,
        }))
    }

    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn get_by_id(&self, request: Request<Id>) -> Result<Response<BlogTag>, Status> {
        let id = &request.into_inner().id;
        let data = BlogTagDb::detail(&self.conn, id)
            .await
            .map_err(RpcError::db_with_context(id))?;
        Ok(Response::new(data))
    }

    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn create(&self, request: Request<BlogTagShape>) -> Result<Response<BlogTag>, Status> {
        let req = request.into_inner();
        let data = BlogTagDb::create(&self.conn, &req)
            .await
            // FIXME: unwrap
            .unwrap();
        Ok(Response::new(data))
    }

    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn update(&self, request: Request<BlogTag>) -> Result<Response<BlogTag>, Status> {
        let req = &request.into_inner();
        let data = BlogTagDb::update(&self.conn, req)
            .await
            .map_err(RpcError::db_with_context(&req.id))?;
        Ok(Response::new(data))
    }

    #[instrument(skip(self, request), level = "DEBUG", ret)]
    async fn delete(&self, request: Request<Id>) -> Result<Response<Id>, Status> {
        let req = &request.into_inner();
        let data = BlogTagDb::delete(&self.conn, &req.id)
            .await
            .map_err(RpcError::db_with_context(&req.id))?;
        Ok(Response::new(data))
    }
}
