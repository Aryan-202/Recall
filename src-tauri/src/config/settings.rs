use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub auto_save: bool,
    pub auto_save_interval: u32,
    pub editor_font_size: u32,
    pub editor_font_family: String,
    pub show_line_numbers: bool,
    pub dark_mode: bool,
    pub default_folder: Option<String>,
    pub default_tags: Vec<String>,
    pub backup_enabled: bool,
    pub backup_interval: u32,
    pub backup_count: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            language: "en".to_string(),
            auto_save: true,
            auto_save_interval: 30,
            editor_font_size: 14,
            editor_font_family: "Monaco, 'Courier New', monospace".to_string(),
            show_line_numbers: true,
            dark_mode: false,
            default_folder: None,
            default_tags: vec![],
            backup_enabled: true,
            backup_interval: 24,
            backup_count: 10,
        }
    }
}

impl AppSettings {
    pub fn load() -> Self {
        let settings_path = Self::get_settings_path();
        
        if let Ok(content) = fs::read_to_string(&settings_path) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            let default_settings = AppSettings::default();
            default_settings.save().ok();
            default_settings
        }
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_path = Self::get_settings_path();
        let settings_dir = Path::new(&settings_path).parent().unwrap();
        
        // Create directory if it doesn't exist
        fs::create_dir_all(settings_dir)?;
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(settings_path, content)?;
        
        Ok(())
    }
    
    fn get_settings_path() -> String {
        if cfg!(debug_assertions) {
            "./settings.json".to_string()
        } else {
            let home_dir = dirs::config_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."));
            let app_dir = home_dir.join("Recall Notes");
            app_dir.join("settings.json").to_string_lossy().to_string()
        }
    }
}