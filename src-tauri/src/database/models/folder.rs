use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Folder {
    pub folder_id: i32,
    pub user_id: i32,
    pub name: String,
    pub parent_folder_id: Option<i32>,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderWithChildren {
    pub folder: Folder,
    pub children: Vec<FolderWithChildren>,
    pub note_count: i64,
}