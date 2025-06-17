use crate::utils::TonicResult;
use proto_types::blog::root::{Blog, BlogShape, blog_service_server::BlogService};
use sqlx::{Pool, Postgres};
use tonic::Request;

pub mod meta;
pub mod tag;

pub struct BlogRpc {
    pub conn: Pool<Postgres>,
}

#[tonic::async_trait]
impl BlogService for BlogRpc {
    async fn create(&self, request: Request<BlogShape>) -> TonicResult<Blog> {
        todo!()
    }
}
