pub mod connection;
pub mod migrations;
pub mod models;
pub mod repository;

pub use connection::*;
pub use models::*;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use tracing::info;

pub type DbPool = Pool<Postgres>;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    info!("Initializing database connection...");
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/recall_notes".to_string());
    
    info!("Connecting to database at: {}", database_url);
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;
    
    // Run migrations
    migrations::run_migrations(&pool).await?;
    
    info!("Database initialized successfully");
    Ok(pool)
}