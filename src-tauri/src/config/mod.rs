pub mod settings;

pub use settings::*;

use std::env;
use dotenvy::dotenv;

pub fn load_config() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    Ok(())
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/recall_notes".to_string())
}

pub fn get_app_name() -> String {
    env::var("APP_NAME").unwrap_or_else(|_| "Recall Notes".to_string())
}

pub fn get_app_version() -> String {
    env::var("APP_VERSION").unwrap_or_else(|_| "1.0.0".to_string())
}

pub fn get_data_dir() -> String {
    if cfg!(debug_assertions) {
        "./data".to_string()
    } else {
        let home_dir = dirs::home_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        home_dir.join(".recall_notes").to_string_lossy().to_string()
    }
}

pub fn get_attachments_dir() -> String {
    let data_dir = get_data_dir();
    let attachments_dir = std::path::Path::new(&data_dir).join("attachments");
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(&attachments_dir).ok();
    
    attachments_dir.to_string_lossy().to_string()
}