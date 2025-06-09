use crate::{DbError, utils::time::now};
use proto_types::{
    blog::meta::{BlogMeta, BlogMetaShape},
    common::db::{Id, Pagination},
};
use sqlx::{Pool, Postgres};
use tracing::instrument;
use uuid::Uuid;

type Db = Pool<Postgres>;

pub struct BlogMetaDb {}

// TODO: return type on all fns
impl BlogMetaDb {
    #[instrument(skip(conn), ret)]
    pub async fn list(conn: &Db, pg: &Pagination) -> Result<Vec<BlogMeta>, DbError> {
        let offset = pg.page_index * pg.page_size;
        let data = sqlx::query_as!(
            BlogMeta,
            "
            SELECT *
            FROM blog_meta
            LIMIT $1 OFFSET $2
            ",
            pg.page_size as i64,
            offset as i64
        )
        .fetch_all(conn)
        .await?;

        tracing::info!("{data:?}");

        Ok(data)
    }

    #[instrument(skip(conn), ret)]
    pub async fn detail(conn: &Db, id: &str) -> Result<BlogMeta, DbError> {
        let data = sqlx::query_as!(
            BlogMeta,
            "
            SELECT *
            FROM blog_meta
            WHERE id = $1
            ",
            id
        )
        .fetch_one(conn)
        .await?;
        Ok(data)
    }

    #[instrument(skip(conn), ret)]
    pub async fn create(conn: &Db, payload: &BlogMetaShape) -> Result<BlogMeta, DbError> {
        let id = Uuid::now_v7().to_string();
        let now = now();
        let data = sqlx::query_as!(
            BlogMeta,
            "
            INSERT INTO blog_meta (id, title, file_name, is_publish, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            ",
            id,
            payload.title,
            payload.file_name,
            payload.is_publish,
            now,
            now
        )
        .fetch_one(conn)
        .await?;

        // TODO: create the markdown file as well

        Ok(data)
    }

    #[instrument(skip(conn), ret)]
    pub async fn update(conn: &Db, id: &str, payload: &BlogMetaShape) -> Result<BlogMeta, DbError> {
        let BlogMetaShape {
            title,
            file_name,
            is_publish,
        } = payload;
        let data = sqlx::query_as!(
            BlogMeta,
            "
            UPDATE blog_meta
            SET title = $2, file_name = $3, is_publish = $4, updated_at = $5
            WHERE id = $1
            RETURNING *
            ",
            id,
            title,
            file_name,
            is_publish,
            now()
        )
        .fetch_one(conn)
        .await?;
        Ok(data)
    }

    #[instrument(skip(conn), ret)]
    pub async fn delete(conn: &Db, id: &str) -> Result<Id, DbError> {
        let id = sqlx::query_as!(
            Id,
            "
            DELETE FROM blog_meta
            WHERE id = $1
            RETURNING id
            ",
            id
        )
        .fetch_one(conn)
        .await?;

        Ok(id)
    }
}
