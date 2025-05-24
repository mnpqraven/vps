use load_env::schema::EnvSchema;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;

// NOTE: WIP
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // TODO: dir
    let db_url = EnvSchema::load().unwrap().db_url();
    dbg!(&db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let recs = sqlx::query!(
        r#"
SELECT id, label
FROM blog_tag
ORDER BY id
        "#
    )
    .fetch_all(&pool)
    .await?;
    for rec in recs {
        println!("{rec:?}");
    }

    Ok(())
}
