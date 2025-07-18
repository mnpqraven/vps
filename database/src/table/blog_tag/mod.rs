use crate::{
    DbError,
    utils::{common_structs::Db, time::now},
};
use proto_types::{
    blog::tag::{BlogTag, BlogTagShape},
    common::db::{Id, Pagination},
};
use tracing::instrument;
use uuid::Uuid;

pub struct BlogTagDb;

impl BlogTagDb {
    #[instrument(skip(conn), ret)]
    pub async fn list(conn: &Db, pg: &Pagination) -> Result<Vec<BlogTag>, DbError> {
        let offset = pg.page_index * pg.page_size;
        // TODO: not very friendly on reusability
        let data = if pg.search.is_empty() {
            sqlx::query_as!(
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
            .await?
        } else {
            sqlx::query_as!(
                BlogTag,
                "
                    SELECT *
                    FROM blog_tag
                    WHERE similarity(label, $3) >= 0.4
                        OR similarity(code, $3) >= 0.4
                    ORDER BY similarity(label, $3) + similarity(code, $3) DESC
                    LIMIT $1 OFFSET $2
                ",
                pg.page_size as i64,
                offset as i64,
                pg.search
            )
            .fetch_all(conn)
            .await?
        };

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

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use proto_types::{blog::tag::BlogTagShape, common::db::Pagination};

    use crate::{DbError, get_db, table::blog_tag::BlogTagDb};

    #[tokio::test]
    async fn blog_tag_1_no_row_pre_test() -> Result<(), DbError> {
        let db = get_db().await?;
        let list = BlogTagDb::list(
            &db,
            &Pagination {
                page_index: 0,
                page_size: 100,
                search: "__cargo_test".into(),
            },
        )
        .await?;
        assert!(list.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn blog_tag_2_create() -> Result<(), DbError> {
        let db = get_db().await?;
        let act = BlogTagDb::create(
            &db,
            &BlogTagShape {
                code: "__cargo_test_hello_code".into(),
                label: "__cargo_test_hello_title".into(),
            },
        )
        .await;
        assert!(act.is_ok());

        // populated in list
        let list = BlogTagDb::list(
            &db,
            &Pagination {
                page_index: 0,
                page_size: 100,
                search: "__cargo_test".into(),
            },
        )
        .await?;
        assert_eq!(list.len(), 1);
        let first_code = list.first().map(|e| e.code.clone());
        assert_eq!(first_code, Some("__cargo_test_hello_code".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn blog_tag_3_update() -> Result<(), DbError> {
        let db = get_db().await?;
        let pg = Pagination {
            page_index: 0,
            page_size: 100,
            search: "__cargo_test".into(),
        };
        let update_str = "__cargo_test_hello_title_updated";

        let list = BlogTagDb::list(&db, &pg).await?;
        assert_eq!(list.len(), 1);
        assert_eq!(
            list.first().map(|e| e.label.as_str()),
            Some("__cargo_test_hello_title")
        );

        let mut next = list.first().cloned().unwrap();
        next.label = update_str.into();

        let updated = BlogTagDb::update(&db, &next).await;
        assert!(updated.is_ok());

        let list = BlogTagDb::list(&db, &pg).await?;
        assert_eq!(list.len(), 1);
        assert_eq!(list.first().map(|e| e.label.as_str()), Some(update_str));
        Ok(())
    }

    #[tokio::test]
    async fn blog_tag_4_delete() -> Result<(), DbError> {
        let db = get_db().await?;
        let pg = Pagination {
            page_index: 0,
            page_size: 100,
            search: "__cargo_test".into(),
        };
        let list = BlogTagDb::list(&db, &pg).await?;
        let try_find = list.first();

        assert!(try_find.is_some());
        let find = try_find.unwrap();

        let deleted = BlogTagDb::delete(&db, &find.id).await;
        assert!(deleted.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn blog_tag_5_cleanup() -> Result<(), DbError> {
        let db = get_db().await?;
        let list = BlogTagDb::list(
            &db,
            &Pagination {
                page_index: 0,
                page_size: 100,
                search: "__cargo_test".into(),
            },
        )
        .await?;
        assert!(list.is_empty());
        Ok(())
    }
}
