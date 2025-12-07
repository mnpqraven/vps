use super::blog_meta::BlogMetaDb;
use crate::{DbError, utils::common_structs::Db};
use proto_types::{
    blog::{
        meta::{BlogMeta, BlogMetaShape},
        root::Blog,
        tag::BlogTag,
    },
    impls::Pagination,
};
use std::fs;
use tracing::instrument;

pub struct BlogDb;

#[derive(Clone)]
struct FlatBlogQuery {
    tag_id: Option<String>,
    tag_label: String,
    tag_code: String,
    tag_created_at: i64,
    tag_updated_at: i64,
    meta_id: Option<String>,
    meta_title: String,
    meta_file_name: String,
    meta_is_publish: bool,
    meta_created_at: i64,
    meta_updated_at: i64,
}

impl From<FlatBlogQuery> for BlogMeta {
    fn from(value: FlatBlogQuery) -> Self {
        Self {
            id: value
                .meta_id
                .clone()
                .expect("mapping ids should be defined on create"),
            title: value.meta_title.clone(),
            file_name: value.meta_file_name.clone(),
            is_publish: value.meta_is_publish,
            created_at: value.meta_created_at,
            updated_at: value.meta_updated_at,
        }
    }
}

impl From<FlatBlogQuery> for BlogTag {
    fn from(value: FlatBlogQuery) -> Self {
        BlogTag {
            id: value
                .tag_id
                .clone()
                .expect("mapping ids should be defined on create"),
            code: value.tag_code.clone(),
            label: value.tag_label.clone(),
            created_at: value.tag_created_at,
            updated_at: value.tag_updated_at,
        }
    }
}

impl BlogDb {
    // TODO: pagination logic
    pub async fn list(conn: &Db, pg: &Pagination) -> Result<Vec<Blog>, DbError> {
        let temp_flat = if pg.search.is_none() {
            sqlx::query_as!(
                FlatBlogQuery,
                "
                    SELECT 
                        blog_meta_tag_map.blog_meta_id AS meta_id,
                        blog_meta_tag_map.blog_tag_id AS tag_id,
                        blog_tag.label AS tag_label,
                        blog_tag.code AS tag_code,
                        blog_tag.created_at AS tag_created_at,
                        blog_tag.updated_at AS tag_updated_at,
                        blog_meta.title AS meta_title,
                        blog_meta.file_name AS meta_file_name,
                        blog_meta.is_publish AS meta_is_publish,
                        blog_meta.created_at AS meta_created_at,
                        blog_meta.updated_at AS meta_updated_at
                    FROM blog_meta_tag_map
                    INNER JOIN blog_tag ON
                    	blog_tag.id = blog_meta_tag_map.blog_tag_id
                    INNER JOIN blog_meta ON
                    	blog_meta.id = blog_meta_tag_map.blog_meta_id
                	ORDER BY meta_id, tag_id
                ",
            )
            .fetch_all(conn)
            .await?
        } else {
            sqlx::query_as!(
                FlatBlogQuery,
                "
                    SELECT 
                        blog_meta_tag_map.blog_meta_id AS meta_id,
                        blog_meta_tag_map.blog_tag_id AS tag_id,
                        blog_tag.label AS tag_label,
                        blog_tag.code AS tag_code,
                        blog_tag.created_at AS tag_created_at,
                        blog_tag.updated_at AS tag_updated_at,
                        blog_meta.title AS meta_title,
                        blog_meta.file_name AS meta_file_name,
                        blog_meta.is_publish AS meta_is_publish,
                        blog_meta.created_at AS meta_created_at,
                        blog_meta.updated_at AS meta_updated_at
                    FROM blog_meta_tag_map
                    INNER JOIN blog_tag ON
                    	blog_tag.id = blog_meta_tag_map.blog_tag_id
                    INNER JOIN blog_meta ON
                    	blog_meta.id = blog_meta_tag_map.blog_meta_id
                    WHERE similarity(blog_meta.title, $1) >= 0.4
                	ORDER BY similarity(blog_meta.title, $1) DESC, meta_id, tag_id
                ",
                pg.search
            )
            .fetch_all(conn)
            .await?
        };

        let data = temp_flat
            .chunk_by(|a, b| a.meta_id == b.meta_id)
            .map(|with_diffing_tags| {
                let peek = with_diffing_tags.first();
                let meta: Option<BlogMeta> = peek.cloned().map(Into::into);
                let tags: Vec<BlogTag> =
                    with_diffing_tags.iter().cloned().map(Into::into).collect();
                let content = read_markdown_file(
                    &meta
                        .as_ref()
                        .expect("meta should be defined on mapping create")
                        .file_name,
                )
                .unwrap();

                Blog {
                    meta,
                    tags,
                    content,
                }
            })
            .collect();

        Ok(data)
    }
    // TODO: rpc to replace meta
    // meta won't be public in rpc endpoint
    pub async fn create(
        conn: &Db,
        meta_shape: &BlogMetaShape,
        tag_ids: Vec<String>,
        file_content: String,
    ) -> Result<Blog, DbError> {
        // insert meta (left)
        let created = BlogMetaDb::create(conn, meta_shape).await?;

        create_markdown_file(&meta_shape.file_name, &file_content).await?;

        // insert mapping (left right)
        let meta_id_zip: Vec<String> = vec![created.id.clone(); tag_ids.len()];
        let _data = sqlx::query!(
            "
                INSERT INTO blog_meta_tag_map (blog_meta_id, blog_tag_id)
                    SELECT * FROM UNNEST($1::text[], $2::text[])
                RETURNING *
            ",
            &meta_id_zip,
            &tag_ids
        )
        .fetch_all(conn)
        .await?;

        let tags = sqlx::query_as!(
            BlogTag,
            "
                SELECT *
                FROM blog_tag
                WHERE id = ANY($1)
            ",
            &tag_ids
        )
        .fetch_all(conn)
        .await?;

        Ok(Blog {
            meta: Some(created),
            tags,
            content: file_content,
        })
    }
    pub async fn detail(conn: &Db, id: &str) -> Result<Blog, DbError> {
        let meta = sqlx::query_as!(
            BlogMeta,
            "
                SELECT *
                FROM blog_meta
                WHERE id = $1
            ",
            id
        );
        let meta = meta.fetch_one(conn).await?;

        let tags = sqlx::query_as!(
            BlogTag,
            "
                SELECT id, code, label, created_at, updated_at
                FROM blog_meta_tag_map
                JOIN blog_tag ON blog_tag.id = blog_meta_tag_map.blog_tag_id
                WHERE blog_meta_tag_map.blog_meta_id = $1
            ",
            id
        )
        .fetch_all(conn)
        .await?;

        let content = read_markdown_file(&meta.file_name)?;

        Ok(Blog {
            meta: Some(meta),
            tags,
            content,
        })
    }

    // TODO:
    // - return type
    // - delete file as well
    pub async fn delete() -> Result<(), DbError> {
        Ok(())
    }
}

fn read_markdown_file(filename: &str) -> Result<String, DbError> {
    let env = load_env::EnvSchema::load()?;
    let path = env.database.blob_storage()?;
    let path = path.join(filename);

    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

// TODO: standalone module
#[instrument(ret)]
pub async fn create_markdown_file(filename: &str, content: &str) -> Result<(), DbError> {
    let env = load_env::EnvSchema::load()?;
    let path = env.database.blob_storage()?;
    let path = path.join(filename);

    fs::write(path, content)?;

    Ok(())
}

#[cfg(test)]
#[serial_test::serial]
mod tests {
    use crate::DbError;

    #[tokio::test]
    async fn blog_1_test_records_not_present() -> Result<(), DbError> {
        // test records not present
        Ok(())
    }
}
