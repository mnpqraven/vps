pub mod table;
pub mod utils;

use load_env::schema::EnvSchema;
use sqlx::postgres::PgPoolOptions;

pub use utils::error::DbError;

pub async fn get_db() -> Result<sqlx::Pool<sqlx::Postgres>, DbError> {
    let db_url = EnvSchema::load()?.db_url();
    // TODO: omit
    tracing::info!("Connecting to database @ {}", &db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/mydatabase")
        .await?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use load_env::schema::EnvSchema;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn valid_pool() {
        let db_url = EnvSchema::load().unwrap_or_default().db_url();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await;

        assert!(pool.is_ok())
    }
}
