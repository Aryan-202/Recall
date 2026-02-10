use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

#[allow(dead_code)]
pub type DbPool = Pool<Postgres>;

#[allow(dead_code)]
pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/recall_notes".to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

#[allow(dead_code)]
pub async fn test_connection(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
