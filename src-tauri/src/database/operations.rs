use crate::database::models::{Note, Tag, NoteWithTags, NoteTag};
use crate::utils::error_handling::AppError;
use chrono::Utc;
use rusqlite::{params, Row, OptionalExtension};
use uuid::Uuid;
use std::sync::Arc;
use log::{info, error};

pub struct NoteRepository {
    db: Arc<Database>,
}

impl NoteRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
    
    pub fn create_note(&self, title: &str, content: &str) -> Result<Note, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        let note = Note {
            id,
            title: title.to_string(),
            content: content.to_string(),
            created_at: now,
            updated_at: now,
            is_pinned: false,
            is_archived: false,
            color: None,
        };
        
        self.db.transaction(|conn| {
            conn.execute(
                "INSERT INTO notes (id, title, content, created_at, updated_at, is_pinned, is_archived, color) 
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                note.as_params(),
            )?;
            
            Ok(note)
        })
    }
    
    pub fn get_note(&self, id: &str) -> Result<Option<Note>, AppError> {
        self.db.transaction(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, title, content, created_at, updated_at, is_pinned, is_archived, color 
                 FROM notes WHERE id = ?1"
            )?;
            
            let note = stmt
                .query_row([id], |row| Self::row_to_note(row))
                .optional()?;
            
            Ok(note)
        })
    }
    
    pub fn get_all_notes(&self) -> Result<Vec<NoteWithTags>, AppError> {
        self.db.transaction(|conn| {
            let mut stmt = conn.prepare(
                "SELECT n.id, n.title, n.content, n.created_at, n.updated_at, 
                        n.is_pinned, n.is_archived, n.color,
                        t.id as tag_id, t.name as tag_name, t.color as tag_color
                 FROM notes n
                 LEFT JOIN note_tags nt ON n.id = nt.note_id
                 LEFT JOIN tags t ON nt.tag_id = t.id
                 WHERE n.is_archived = 0
                 ORDER BY n.is_pinned DESC, n.updated_at DESC"
            )?;
            
            let mut notes_map = std::collections::HashMap::new();
            let rows = stmt.query_map([], |row| {
                let note = Self::row_to_note(row)?;
                let tag_id: Option<String> = row.get(8)?;
                let tag_name: Option<String> = row.get(9)?;
                let tag_color: Option<String> = row.get(10)?;
                
                Ok((note, tag_id, tag_name, tag_color))
            })?;
            
            for row in rows {
                let (note, tag_id, tag_name, tag_color) = row?;
                let entry = notes_map.entry(note.id.clone()).or_insert_with(|| {
                    NoteWithTags {
                        note,
                        tags: Vec::new(),
                    }
                });
                
                if let (Some(tag_id), Some(tag_name)) = (tag_id, tag_name) {
                    entry.tags.push(Tag {
                        id: tag_id,
                        name: tag_name,
                        color: tag_color,
                    });
                }
            }
            
            Ok(notes_map.into_values().collect())
        })
    }
    
    pub fn update_note(&self, id: &str, title: &str, content: &str) -> Result<Note, AppError> {
        self.db.transaction(|conn| {
            let now = Utc::now();
            
            conn.execute(
                "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
                params![title, content, now.to_rfc3339(), id],
            )?;
            
            // Fetch the updated note
            let mut stmt = conn.prepare(
                "SELECT id, title, content, created_at, updated_at, is_pinned, is_archived, color 
                 FROM notes WHERE id = ?1"
            )?;
            
            stmt.query_row([id], |row| Self::row_to_note(row))
                .map_err(|e| e.into())
        })
    }
    
    pub fn delete_note(&self, id: &str) -> Result<(), AppError> {
        self.db.transaction(|conn| {
            conn.execute("DELETE FROM notes WHERE id = ?1", [id])?;
            Ok(())
        })
    }
    
    pub fn search_notes(&self, query: &str) -> Result<Vec<NoteWithTags>, AppError> {
        let search_term = format!("%{}%", query);
        
        self.db.transaction(|conn| {
            let mut stmt = conn.prepare(
                "SELECT DISTINCT n.id, n.title, n.content, n.created_at, n.updated_at, 
                        n.is_pinned, n.is_archived, n.color,
                        t.id as tag_id, t.name as tag_name, t.color as tag_color
                 FROM notes n
                 LEFT JOIN note_tags nt ON n.id = nt.note_id
                 LEFT JOIN tags t ON nt.tag_id = t.id
                 WHERE (n.title LIKE ?1 OR n.content LIKE ?1 OR t.name LIKE ?1)
                   AND n.is_archived = 0
                 ORDER BY n.updated_at DESC"
            )?;
            
            // Similar processing as get_all_notes...
            Ok(Vec::new()) // Simplified for brevity
        })
    }
    
    fn row_to_note(row: &Row) -> rusqlite::Result<Note> {
        let created_at_str: String = row.get(3)?;
        let updated_at_str: String = row.get(4)?;
        
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| rusqlite::Error::InvalidQuery(e.to_string()))?
                .with_timezone(&Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                .map_err(|e| rusqlite::Error::InvalidQuery(e.to_string()))?
                .with_timezone(&Utc),
            is_pinned: row.get(5)?,
            is_archived: row.get(6)?,
            color: row.get(7)?,
        })
    }
}