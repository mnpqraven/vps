use crate::{
    DbError,
    utils::{common_structs::Db, time::now},
};
use proto_types::{
    blog::meta::{BlogMeta, BlogMetaShape},
    common::db::{Id, Pagination},
};
use std::fs;
use tracing::instrument;
use uuid::Uuid;

pub struct BlogMetaDb {}

// TODO: return type on all fns
impl BlogMetaDb {
    #[instrument(skip(conn), ret)]
    pub async fn list(conn: &Db, pg: &Pagination) -> Result<Vec<BlogMeta>, DbError> {
        let offset = pg.page_index * pg.page_size;
        // TODO: not very friendly on reusability
        let data = if pg.search.is_empty() {
            sqlx::query_as!(
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
            .await?
        } else {
            sqlx::query_as!(
                BlogMeta,
                "
                    SELECT * 
                    FROM blog_meta
                    WHERE similarity(title, $3) >= 0.4
                    ORDER BY similarity(title, $3) DESC
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

        Ok(data)
    }

    #[instrument(skip(conn), ret)]
    pub async fn update(conn: &Db, id: &str, payload: &BlogMetaShape) -> Result<BlogMeta, DbError> {
        let BlogMetaShape {
            title,
            file_name,
            is_publish,
        } = payload;
        let old_filename = sqlx::query!(
            "
                SELECT file_name
                FROM blog_meta
                WHERE id = $1
            ",
            id
        )
        .fetch_one(conn)
        .await?
        .file_name;

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

        markdown_rename(&old_filename, file_name).await?;

        Ok(data)
    }

    #[instrument(skip(conn), ret)]
    pub async fn delete(conn: &Db, id: &str) -> Result<Id, DbError> {
        let deleted = sqlx::query_as!(
            BlogMeta,
            "
            DELETE FROM blog_meta
            WHERE id = $1
            RETURNING *
            ",
            id
        )
        .fetch_one(conn)
        .await?;

        delete_markdown_file(&deleted.file_name).await?;

        Ok(Id { id: deleted.id })
    }
}

async fn markdown_rename(old_filename: &str, new_filename: &str) -> Result<(), DbError> {
    let env = load_env::EnvSchema::load()?;
    let path = env.database.blob_storage()?;
    let (old_path, new_path) = (path.join(old_filename), path.join(new_filename));
    fs::rename(old_path, new_path)?;

    Ok(())
}

#[instrument(ret)]
async fn delete_markdown_file(filename: &str) -> Result<(), DbError> {
    let env = load_env::EnvSchema::load()?;
    let path = env.database.blob_storage()?;
    let path = path.join(filename);

    fs::remove_file(path)?;

    Ok(())
}

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use proto_types::{blog::meta::BlogMetaShape, common::db::Pagination};

    use crate::{
        DbError, get_db,
        table::{blog::create_markdown_file, blog_meta::BlogMetaDb},
    };

    #[tokio::test]
    async fn blog_meta_1_blanket_create_md_file() -> Result<(), DbError> {
        create_markdown_file("__cargo_test_filename.md", "abitrary string content").await?;
        Ok(())
    }

    #[tokio::test]
    async fn blog_meta_2_no_row_pre_test() -> Result<(), DbError> {
        let db = get_db().await?;
        let list = BlogMetaDb::list(
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
    // TODO: missing check for file
    async fn blog_meta_3_create() -> Result<(), DbError> {
        let db = get_db().await?;
        let act = BlogMetaDb::create(
            &db,
            &BlogMetaShape {
                title: "__cargo_test_title".into(),
                file_name: "__cargo_test_filename.md".into(),
                is_publish: false,
            },
        )
        .await;
        assert!(act.is_ok());

        // populated in list
        let list = BlogMetaDb::list(
            &db,
            &Pagination {
                page_index: 0,
                page_size: 100,
                search: "__cargo_test".into(),
            },
        )
        .await?;
        assert_eq!(list.len(), 1);
        let first_filename = list.first().map(|e| e.file_name.as_str());
        assert_eq!(first_filename, Some("__cargo_test_filename.md"));

        Ok(())
    }

    #[tokio::test]
    // TODO: missing check for file
    async fn blog_meta_4_update() -> Result<(), DbError> {
        let db = get_db().await?;
        let pg = Pagination {
            page_index: 0,
            page_size: 100,
            search: "__cargo_test".into(),
        };
        let update_filename_str = "__cargo_test_filename_update.md";

        let list = BlogMetaDb::list(&db, &pg).await?;
        assert_eq!(list.len(), 1);
        assert_eq!(
            list.first().map(|e| e.file_name.as_str()),
            Some("__cargo_test_filename.md")
        );

        let find = list.first().cloned().unwrap();
        let next_shape = BlogMetaShape {
            title: find.title,
            file_name: update_filename_str.into(),
            is_publish: find.is_publish,
        };

        let updated = BlogMetaDb::update(&db, &find.id, &next_shape).await;
        assert!(updated.is_ok());

        let list = BlogMetaDb::list(&db, &pg).await?;
        assert_eq!(list.len(), 1);
        assert_eq!(
            list.first().map(|e| e.file_name.as_str()),
            Some(update_filename_str)
        );
        Ok(())
    }

    #[tokio::test]
    // TODO: missing check for file
    async fn blog_meta_5_delete() -> Result<(), DbError> {
        let db = get_db().await?;
        let pg = Pagination {
            page_index: 0,
            page_size: 100,
            search: "__cargo_test".into(),
        };
        let list = BlogMetaDb::list(&db, &pg).await?;
        let try_find = list.first();

        assert!(try_find.is_some());

        let find = try_find.unwrap();
        let deleted = BlogMetaDb::delete(&db, &find.id).await;

        assert!(deleted.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn blog_meta_6_cleanup() -> Result<(), DbError> {
        let db = get_db().await?;
        let list = BlogMetaDb::list(
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
