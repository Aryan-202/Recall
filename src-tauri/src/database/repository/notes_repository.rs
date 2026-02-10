use super::super::models::note::{Note, NoteWithRelations, FolderInfo, TagInfo};
use crate::utils::error::{AppError, Result};
use sqlx::{Pool, Postgres};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct NoteRepository {
    pool: Pool<Postgres>,
}

impl NoteRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    
    pub async fn create_note(&self, dto: CreateNoteDto, tags: &[String]) -> Result<NoteWithRelations> {
        let mut tx = self.pool.begin().await?;
        
        // Create note
        let note = sqlx::query_as::<_, Note>(
            r#"
            INSERT INTO notes (user_id, folder_id, title, content, is_pinned)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(dto.user_id)
        .bind(dto.folder_id)
        .bind(&dto.title)
        .bind(&dto.content)
        .bind(dto.is_pinned)
        .fetch_one(&mut *tx)
        .await?;
        
        // Add tags
        for tag_name in tags {
            if let Some(tag) = self.get_or_create_tag(dto.user_id, tag_name, &mut tx).await? {
                sqlx::query(
                    "INSERT INTO note_tags (note_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
                )
                .bind(note.note_id)
                .bind(tag.tag_id)
                .execute(&mut *tx)
                .await?;
            }
        }
        
        tx.commit().await?;
        
        // Fetch with relations
        self.get_note_with_relations(note.note_id).await
    }
    
    pub async fn get_note_by_id(&self, note_id: i32) -> Result<Option<NoteWithRelations>> {
        let note = sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE note_id = $1 AND is_deleted = FALSE"
        )
        .bind(note_id)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(note) = note {
            self.get_note_with_relations(note.note_id).await.map(Some)
        } else {
            Ok(None)
        }
    }
    
    pub async fn get_user_notes(&self, user_id: i32) -> Result<Vec<NoteWithRelations>> {
        let notes = sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE user_id = $1 AND is_deleted = FALSE ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut result = Vec::new();
        for note in notes {
            let with_relations = self.get_note_with_relations(note.note_id).await?;
            result.push(with_relations);
        }
        
        Ok(result)
    }
    
    pub async fn update_note(&self, dto: UpdateNoteDto, tags: Option<Vec<String>>) -> Result<NoteWithRelations> {
        let mut tx = self.pool.begin().await?;
        
        // Build update query dynamically
        let mut query = "UPDATE notes SET updated_at = $1".to_string();
        let mut params: Vec<String> = vec![Utc::now().to_string()];
        let mut param_count = 2;
        
        if let Some(title) = &dto.title {
            query.push_str(&format!(", title = ${}", param_count));
            params.push(title.clone());
            param_count += 1;
        }
        
        if let Some(content) = &dto.content {
            query.push_str(&format!(", content = ${}", param_count));
            params.push(content.clone());
            param_count += 1;
        }
        
        if let Some(folder_id) = dto.folder_id {
            query.push_str(&format!(", folder_id = ${}", param_count));
            params.push(folder_id.map(|id| id.to_string()).unwrap_or("NULL".to_string()));
            param_count += 1;
        }
        
        if let Some(is_pinned) = dto.is_pinned {
            query.push_str(&format!(", is_pinned = ${}", param_count));
            params.push(is_pinned.to_string());
            param_count += 1;
        }
        
        if let Some(is_archived) = dto.is_archived {
            query.push_str(&format!(", is_archived = ${}", param_count));
            params.push(is_archived.to_string());
            param_count += 1;
        }
        
        query.push_str(&format!(" WHERE note_id = ${} AND is_deleted = FALSE RETURNING *", param_count));
        params.push(dto.note_id.to_string());
        
        // Execute update
        let note = sqlx::query_as::<_, Note>(&query);
        let mut query_builder = note;
        
        for param in &params {
            query_builder = query_builder.bind(param);
        }
        
        let note = query_builder.fetch_one(&mut *tx).await?;
        
        // Update tags if provided
        if let Some(tag_names) = tags {
            // Clear existing tags
            sqlx::query("DELETE FROM note_tags WHERE note_id = $1")
                .bind(note.note_id)
                .execute(&mut *tx)
                .await?;
            
            // Add new tags
            for tag_name in tag_names {
                if let Some(tag) = self.get_or_create_tag(note.user_id, &tag_name, &mut tx).await? {
                    sqlx::query(
                        "INSERT INTO note_tags (note_id, tag_id) VALUES ($1, $2)"
                    )
                    .bind(note.note_id)
                    .bind(tag.tag_id)
                    .execute(&mut *tx)
                    .await?;
                }
            }
        }
        
        tx.commit().await?;
        
        self.get_note_with_relations(note.note_id).await
    }
    
    pub async fn soft_delete_note(&self, note_id: i32, user_id: i32) -> Result<()> {
        sqlx::query(
            "UPDATE notes SET is_deleted = TRUE WHERE note_id = $1 AND user_id = $2"
        )
        .bind(note_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn search_notes(&self, user_id: i32, query: &str) -> Result<Vec<NoteWithRelations>> {
        let notes = sqlx::query_as::<_, Note>(
            r#"
            SELECT n.* FROM notes n
            WHERE n.user_id = $1 
            AND n.is_deleted = FALSE
            AND (n.title ILIKE $2 OR n.content ILIKE $2)
            ORDER BY n.updated_at DESC
            "#
        )
        .bind(user_id)
        .bind(format!("%{}%", query))
        .fetch_all(&self.pool)
        .await?;
        
        let mut result = Vec::new();
        for note in notes {
            let with_relations = self.get_note_with_relations(note.note_id).await?;
            result.push(with_relations);
        }
        
        Ok(result)
    }
    
    pub async fn get_notes_by_folder(&self, user_id: i32, folder_id: i32) -> Result<Vec<NoteWithRelations>> {
        let notes = sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE user_id = $1 AND folder_id = $2 AND is_deleted = FALSE ORDER BY created_at DESC"
        )
        .bind(user_id)
        .bind(folder_id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut result = Vec::new();
        for note in notes {
            let with_relations = self.get_note_with_relations(note.note_id).await?;
            result.push(with_relations);
        }
        
        Ok(result)
    }
    
    pub async fn get_pinned_notes(&self, user_id: i32) -> Result<Vec<NoteWithRelations>> {
        let notes = sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE user_id = $1 AND is_pinned = TRUE AND is_deleted = FALSE ORDER BY updated_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut result = Vec::new();
        for note in notes {
            let with_relations = self.get_note_with_relations(note.note_id).await?;
            result.push(with_relations);
        }
        
        Ok(result)
    }
    
    pub async fn get_archived_notes(&self, user_id: i32) -> Result<Vec<NoteWithRelations>> {
        let notes = sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE user_id = $1 AND is_archived = TRUE AND is_deleted = FALSE ORDER BY updated_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut result = Vec::new();
        for note in notes {
            let with_relations = self.get_note_with_relations(note.note_id).await?;
            result.push(with_relations);
        }
        
        Ok(result)
    }
    
    async fn get_note_with_relations(&self, note_id: i32) -> Result<NoteWithRelations> {
        // Get note
        let note = sqlx::query_as::<_, Note>(
            "SELECT * FROM notes WHERE note_id = $1"
        )
        .bind(note_id)
        .fetch_one(&self.pool)
        .await?;
        
        // Get folder if exists
        let folder = if let Some(folder_id) = note.folder_id {
            sqlx::query_as::<_, FolderInfo>(
                "SELECT folder_id, name, color FROM folders WHERE folder_id = $1"
            )
            .bind(folder_id)
            .fetch_optional(&self.pool)
            .await?
        } else {
            None
        };
        
        // Get tags
        let tags = sqlx::query_as::<_, TagInfo>(
            r#"
            SELECT t.tag_id, t.name, t.color 
            FROM tags t
            INNER JOIN note_tags nt ON t.tag_id = nt.tag_id
            WHERE nt.note_id = $1
            "#
        )
        .bind(note_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(NoteWithRelations {
            note,
            folder,
            tags,
        })
    }
    
    async fn get_or_create_tag(
        &self, 
        user_id: i32, 
        tag_name: &str, 
        executor: &mut sqlx::Transaction<'_, Postgres>
    ) -> Result<Option<crate::models::tag::Tag>> {
        // Try to get existing tag
        let tag = sqlx::query_as::<_, crate::models::tag::Tag>(
            "SELECT * FROM tags WHERE user_id = $1 AND LOWER(name) = LOWER($2)"
        )
        .bind(user_id)
        .bind(tag_name)
        .fetch_optional(&mut **executor)
        .await?;
        
        if let Some(tag) = tag {
            Ok(Some(tag))
        } else {
            // Create new tag
            let tag = sqlx::query_as::<_, crate::models::tag::Tag>(
                "INSERT INTO tags (user_id, name) VALUES ($1, $2) RETURNING *"
            )
            .bind(user_id)
            .bind(tag_name)
            .fetch_one(&mut **executor)
            .await?;
            
            Ok(Some(tag))
        }
    }
}

#[derive(Debug)]
pub struct CreateNoteDto {
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub folder_id: Option<i32>,
    pub is_pinned: bool,
}

#[derive(Debug)]
pub struct UpdateNoteDto {
    pub note_id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub folder_id: Option<Option<i32>>,
    pub is_pinned: Option<bool>,
    pub is_archived: Option<bool>,
}