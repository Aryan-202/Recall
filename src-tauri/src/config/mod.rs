pub mod settings;

#[allow(unused_imports)]
pub use settings::*;

#[allow(dead_code)]
pub fn load_config() -> Result<(), Box<dyn std::error::Error>> {
    // Current setup doesn't really need a config file yet as we use env vars
    Ok(())
}

#[allow(dead_code)]
pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/recall_notes".to_string())
}

#[allow(dead_code)]
pub fn get_app_name() -> String {
    "Recall Notes".to_string()
}

#[allow(dead_code)]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[allow(dead_code)]
pub fn get_data_dir() -> String {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    format!("{}/.recall", home)
}

#[allow(dead_code)]
pub fn get_attachments_dir() -> String {
    format!("{}/attachments", get_data_dir())
}
