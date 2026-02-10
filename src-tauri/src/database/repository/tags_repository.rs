use super::super::models::tag::{Tag, TagWithNotes};
use super::super::models::note::Note;
use crate::utils::error::{AppError, Result};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct TagRepository {
    pool: Pool<Postgres>,
}

impl TagRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    
    pub async fn create_tag(&self, dto: CreateTagDto) -> Result<TagWithNotes> {
        let tag = sqlx::query_as::<_, Tag>(
            r#"
            INSERT INTO tags (user_id, name, color)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(dto.user_id)
        .bind(&dto.name)
        .bind(&dto.color)
        .fetch_one(&self.pool)
        .await?;
        
        self.get_tag_with_notes(tag.tag_id).await
    }
    
    pub async fn get_tag_by_id(&self, tag_id: i32) -> Result<Option<TagWithNotes>> {
        let tag = sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags WHERE tag_id = $1"
        )
        .bind(tag_id)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(tag) = tag {
            self.get_tag_with_notes(tag.tag_id).await.map(Some)
        } else {
            Ok(None)
        }
    }
    
    pub async fn get_user_tags(&self, user_id: i32) -> Result<Vec<TagWithNotes>> {
        let tags = sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags WHERE user_id = $1 ORDER BY name"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut result = Vec::new();
        for tag in tags {
            let with_notes = self.get_tag_with_notes(tag.tag_id).await?;
            result.push(with_notes);
        }
        
        Ok(result)
    }
    
    pub async fn update_tag(&self, dto: UpdateTagDto) -> Result<TagWithNotes> {
        // Build update query dynamically
        let mut query = "UPDATE tags SET ".to_string();
        let mut params: Vec<String> = Vec::new();
        let mut param_count = 1;
        
        if let Some(name) = &dto.name {
            query.push_str(&format!("name = ${}", param_count));
            params.push(name.clone());
            param_count += 1;
        }
        
        if let Some(color) = &dto.color {
            if !params.is_empty() {
                query.push_str(", ");
            }
            query.push_str(&format!("color = ${}", param_count));
            params.push(color.clone());
            param_count += 1;
        }
        
        query.push_str(&format!(" WHERE tag_id = ${} RETURNING *", param_count));
        params.push(dto.tag_id.to_string());
        
        // Execute update
        let tag = sqlx::query_as::<_, Tag>(&query);
        let mut query_builder = tag;
        
        for param in &params {
            query_builder = query_builder.bind(param);
        }
        
        let tag = query_builder.fetch_one(&self.pool).await?;
        
        self.get_tag_with_notes(tag.tag_id).await
    }
    
    pub async fn delete_tag(&self, tag_id: i32, user_id: i32) -> Result<()> {
        sqlx::query("DELETE FROM tags WHERE tag_id = $1 AND user_id = $2")
            .bind(tag_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn assign_tag_to_note(&self, note_id: i32, tag_id: i32) -> Result<()> {
        sqlx::query(
            "INSERT INTO note_tags (note_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(note_id)
        .bind(tag_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn remove_tag_from_note(&self, note_id: i32, tag_id: i32) -> Result<()> {
        sqlx::query("DELETE FROM note_tags WHERE note_id = $1 AND tag_id = $2")
            .bind(note_id)
            .bind(tag_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    pub async fn get_notes_by_tag(&self, tag_id: i32) -> Result<Vec<Note>> {
        let notes = sqlx::query_as::<_, Note>(
            r#"
            SELECT n.* 
            FROM notes n
            INNER JOIN note_tags nt ON n.note_id = nt.note_id
            WHERE nt.tag_id = $1 AND n.is_deleted = FALSE
            ORDER BY n.updated_at DESC
            "#
        )
        .bind(tag_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(notes)
    }
    
    async fn get_tag_with_notes(&self, tag_id: i32) -> Result<TagWithNotes> {
        let tag = sqlx::query_as::<_, Tag>(
            "SELECT * FROM tags WHERE tag_id = $1"
        )
        .bind(tag_id)
        .fetch_one(&self.pool)
        .await?;
        
        let note_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM note_tags WHERE tag_id = $1"
        )
        .bind(tag_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(TagWithNotes {
            tag,
            note_count,
        })
    }
}

#[derive(Debug)]
pub struct CreateTagDto {
    pub user_id: i32,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug)]
pub struct UpdateTagDto {
    pub tag_id: i32,
    pub name: Option<String>,
    pub color: Option<String>,
}