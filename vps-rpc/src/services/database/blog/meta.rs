use crate::utils::error::RpcError;
use database::table::blog_meta::BlogMetaDb;
use proto_types::{
    blog::meta::{
        BlogMeta, BlogMetaList, BlogMetaShape, blog_meta_service_server::BlogMetaService,
    },
    common::db::{Id, Pagination},
};
use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct BlogMetaRpc {
    pub conn: Pool<Postgres>,
}

#[tonic::async_trait]
impl BlogMetaService for BlogMetaRpc {
    async fn list(&self, request: Request<Pagination>) -> Result<Response<BlogMetaList>, Status> {
        let pagination = request.into_inner();
        let data = BlogMetaDb::list(&self.conn, &pagination)
            .await
            .map_err(RpcError::db_with_context("uhh idk ???"))?;

        Ok(Response::new(BlogMetaList {
            pagination: Some(pagination),
            total: data.len() as i32,
            data,
        }))
    }
    async fn get_by_id(&self, request: Request<Id>) -> Result<Response<BlogMeta>, Status> {
        let id = &request.into_inner().id;
        let data = BlogMetaDb::detail(&self.conn, id)
            .await
            .map_err(RpcError::db_with_context(id))?;
        Ok(Response::new(data))
    }
    // TODO: filename conflict check
    async fn create(&self, request: Request<BlogMetaShape>) -> Result<Response<BlogMeta>, Status> {
        let req = request.into_inner();
        let data = BlogMetaDb::create(&self.conn, &req)
            .await
            // FIXME: unwrap
            .unwrap();
        Ok(Response::new(data))
    }
    // TODO: filename conflict check
    async fn update(&self, request: Request<BlogMeta>) -> Result<Response<BlogMeta>, Status> {
        let req = &request.into_inner();
        let data = BlogMetaDb::update(&self.conn, &req.id, &req.clone().into_shape())
            .await
            .map_err(RpcError::db_with_context(&req.id))?;
        Ok(Response::new(data))
    }
    async fn delete(&self, request: Request<Id>) -> Result<Response<Id>, Status> {
        let req = &request.into_inner();
        let data = BlogMetaDb::delete(&self.conn, &req.id)
            .await
            .map_err(RpcError::db_with_context(&req.id))?;
        Ok(Response::new(data))
    }
}

// TODO: refactor
trait IntoShape<TShape> {
    fn into_shape(self) -> TShape;
}

impl IntoShape<BlogMetaShape> for BlogMeta {
    fn into_shape(self) -> BlogMetaShape {
        let Self {
            title,
            file_name,
            is_publish,
            ..
        } = self.clone();
        BlogMetaShape {
            title,
            file_name,
            is_publish,
        }
    }
}
