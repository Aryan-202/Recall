pub mod commands;
pub mod database;
pub mod utils;
pub mod config;
pub mod menu;

// Re-exports for easier access
pub use commands::*;
pub use database::*;
pub use utils::*;
pub use config::*;

// Application state
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppState {
    pub db_pool: Arc<PgPool>,
}

impl AppState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = config::get_database_url();
        let pool = PgPool::connect(&database_url).await?;
        
        Ok(AppState {
            db_pool: Arc::new(pool),
        })
    }
}