use proto_types::{
    blog::{
        meta::{BlogMeta, BlogMetaShape},
        tag::BlogTag,
    },
    derived::blog::Blog,
};
use sqlx::{Pool, Postgres, QueryBuilder};

use crate::DbError;

use super::blog_meta::BlogMetaDb;

type Db = Pool<Postgres>;

pub struct BlogDb;

impl BlogDb {
    pub async fn create(
        conn: &Db,
        meta_shape: &BlogMetaShape,
        tag_ids: Vec<&str>,
    ) -> Result<(), DbError> {
        // insert meta (left)
        let created = BlogMetaDb::create(conn, meta_shape).await?;

        // insert mapping (left right)
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO blog_meta_tag_map (blog_meta_id, blog_tag_id) ");
        let query = query_builder
            .push_values(tag_ids, |mut row, tag_id| {
                row.push_bind(created.id.clone()).push_bind(tag_id);
            })
            .build();
        let _ = query.execute(conn).await?;

        Ok(())
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

        let tags = sqlx::query!(
            "
                select *
                from blog_meta_tag_map
                join blog_tag on blog_tag.id = blog_meta_tag_map.blog_tag_id
                where blog_meta_tag_map.blog_meta_id = $1
            ",
            id
        );
        let tags: Vec<BlogTag> = tags
            .fetch_all(conn)
            .await?
            .into_iter()
            .map(|e| BlogTag {
                id: e.id,
                code: e.code,
                label: e.label,
                created_at: e.created_at,
                updated_at: e.updated_at,
            })
            .collect();

        let content = read_markdown_file(&meta.file_name).await?;

        Ok(Blog {
            meta,
            tags,
            content,
        })
    }
}

async fn read_markdown_file(filename: &str) -> Result<String, DbError> {
    let env = load_env::EnvSchema::load()?;
    let path = env.database.blob_storage()?;
    let path = path.join(filename);

    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
