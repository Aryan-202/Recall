use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Note {
    pub note_id: i32,
    pub user_id: i32,
    pub folder_id: Option<i32>,
    pub title: String,
    pub content: String,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteWithRelations {
    pub note: Note,
    pub folder: Option<FolderInfo>,
    pub tags: Vec<TagInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderInfo {
    pub folder_id: i32,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagInfo {
    pub tag_id: i32,
    pub name: String,
    pub color: Option<String>,
}