use crate::{DbError, utils::time::now};
use proto_types::{
    blog::tag::{BlogTag, BlogTagShape},
    common::db::{Id, Pagination},
};
use sqlx::{Pool, Postgres};
use tracing::instrument;
use uuid::Uuid;

type Db = Pool<Postgres>;

pub struct BlogTagDb {}

// TODO: return type on all fns
impl BlogTagDb {
    #[instrument(skip(conn), ret)]
    pub async fn list(conn: &Db, pg: &Pagination) -> Result<Vec<BlogTag>, DbError> {
        let offset = pg.page_index * pg.page_size;
        let data = sqlx::query_as!(
            BlogTag,
            "
            SELECT *
            FROM blog_tag
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
    pub async fn detail(conn: &Db, id: &str) -> Result<BlogTag, DbError> {
        let data = sqlx::query_as!(
            BlogTag,
            "
            SELECT *
            FROM blog_tag
            WHERE id = $1
            ",
            id
        )
        .fetch_one(conn)
        .await?;
        Ok(data)
    }

    #[instrument(skip(conn), ret)]
    pub async fn create(conn: &Db, payload: &BlogTagShape) -> Result<BlogTag, DbError> {
        let id = Uuid::now_v7().to_string();
        let now = now();
        let data = sqlx::query_as!(
            BlogTag,
            "
            INSERT INTO blog_tag (id, code, label, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            ",
            id,
            payload.code,
            payload.label,
            now,
            now
        )
        .fetch_one(conn)
        .await?;

        Ok(data)
    }

    #[instrument(skip(conn), ret)]
    pub async fn update(conn: &Db, payload: &BlogTag) -> Result<BlogTag, DbError> {
        let BlogTag {
            id, code, label, ..
        } = payload;
        let data = sqlx::query_as!(
            BlogTag,
            "
            UPDATE blog_tag
            SET code = $2, label = $3, updated_at = $4
            WHERE id = $1
            RETURNING *
            ",
            id,
            code,
            label,
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
            DELETE FROM blog_tag
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
