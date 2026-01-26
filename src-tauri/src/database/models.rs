use chrono::{DateTime, Utc};
use rusqlite::types::ToSql;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteTag {
    pub note_id: String,
    pub tag_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteWithTags {
    #[serde(flatten)]
    pub note: Note,
    pub tags: Vec<Tag>,
}

// Implement From<&Note> for &[&dyn ToSql] for batch operations
impl Note {
    pub fn as_params(&self) -> [&dyn ToSql; 8] {
        [
            &self.id,
            &self.title,
            &self.content,
            &self.created_at.to_rfc3339(),
            &self.updated_at.to_rfc3339(),
            &self.is_pinned,
            &self.is_archived,
            &self.color,
        ]
    }
}