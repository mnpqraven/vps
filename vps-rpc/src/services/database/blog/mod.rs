use crate::utils::{TonicResult, error::RpcError};
use database::table::blog::BlogDb;
use proto_types::{
    blog::root::{Blog, BlogShape, blog_service_server::BlogService},
    common::db::Id,
};
use sqlx::{Pool, Postgres};
use tonic::{Request, Status};

pub mod meta;
pub mod tag;

pub struct BlogRpc {
    pub conn: Pool<Postgres>,
}

#[tonic::async_trait]
impl BlogService for BlogRpc {
    async fn create(&self, request: Request<BlogShape>) -> TonicResult<Blog> {
        let BlogShape {
            meta_shape,
            tag_ids,
            file_content,
        } = request.into_inner();
        if let Some(meta_shape) = meta_shape {
            let data = BlogDb::create(&self.conn, &meta_shape, tag_ids, file_content)
                .await
                .map_err(RpcError::db_with_context("uhh idk ???"))?;

            Ok(data.into())
        } else {
            Err(Status::invalid_argument("missing meta_shape".to_string()))
        }
    }

    async fn detail(&self, request: Request<Id>) -> TonicResult<Blog> {
        let Id { id } = request.into_inner();
        let data = BlogDb::detail(&self.conn, &id)
            .await
            .map_err(RpcError::db_with_context(id))?;

        Ok(data.into())
    }
}
