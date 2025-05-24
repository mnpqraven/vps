use crate::{
    DbError,
    utils::common_structs::{Db, Pagination},
};
use tracing::instrument;
use uuid::Uuid;

// TODO: return type on all fns
impl BlogTag {
    // TODO: wrap in summary object (incl. total, has_next etc)
    #[instrument(ret)]
    async fn list(conn: Db, pg: Pagination) -> Result<Vec<BlogTag>, DbError> {
        let data = sqlx::query_as!(
            BlogTag,
            "
            SELECT id, code, label
            FROM blog_tag
            LIMIT $1 OFFSET $2
            ",
            pg.size(),
            pg.offset()
        )
        .fetch_all(&conn)
        .await?;

        tracing::info!("{data:?}");

        Ok(data)
    }

    #[instrument(ret)]
    async fn detail(conn: Db, id: String) -> Result<BlogTag, DbError> {
        let data = sqlx::query_as!(
            BlogTag,
            "
            SELECT id, code, label
            FROM blog_tag
            WHERE id = $1
            ",
            id
        )
        .fetch_one(&conn)
        .await?;
        Ok(data)
    }

    #[instrument(ret)]
    async fn create(conn: Db, payload: BlogTagCreate) -> Result<BlogTag, DbError> {
        let id = Uuid::now_v7().to_string();
        let data = sqlx::query_as!(
            BlogTag,
            "
            INSERT INTO blog_tag (id, code, label)
            VALUES ($1, $2, $3)
            RETURNING id, code, label
            ",
            id,
            payload.code,
            payload.label
        )
        .fetch_one(&conn)
        .await?;

        Ok(data)
    }

    #[instrument(ret)]
    async fn update(conn: Db, payload: BlogTag) -> Result<BlogTag, DbError> {
        let BlogTag { id, code, label } = payload;
        let data = sqlx::query_as!(
            BlogTag,
            "
            UPDATE blog_tag
            SET code = $2, label = $3
            WHERE id = $1
            RETURNING id, code, label
            ",
            id,
            code,
            label
        )
        .fetch_one(&conn)
        .await?;
        Ok(data)
    }

    #[instrument(ret)]
    async fn delete(conn: Db, id: String) -> Result<(), DbError> {
        sqlx::query!("DELETE FROM blog_tag WHERE id = $1", id)
            .fetch_one(&conn)
            .await?;

        Ok(())
    }
}
