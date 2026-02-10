use super::super::models::attachment::Attachment;
use crate::utils::error::{AppError, Result};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct AttachmentRepository {
    pool: Pool<Postgres>,
}

impl AttachmentRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    
    pub async fn create_attachment(
        &self,
        note_id: i32,
        file_name: &str,
        file_path: &str,
        file_size: i64,
        mime_type: &str,
    ) -> Result<Attachment> {
        let attachment = sqlx::query_as::<_, Attachment>(
            r#"
            INSERT INTO attachments (note_id, file_name, file_path, file_size, mime_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(note_id)
        .bind(file_name)
        .bind(file_path)
        .bind(file_size)
        .bind(mime_type)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(attachment)
    }
    
    pub async fn delete_attachment(&self, attachment_id: i32) -> Result<()> {
        // Get file path first
        let attachment = self.get_attachment_by_id(attachment_id).await?;
        
        sqlx::query("DELETE FROM attachments WHERE attachment_id = $1")
            .bind(attachment_id)
            .execute(&self.pool)
            .await?;
        
        // Delete the actual file
        if let Ok(attachment) = attachment {
            let _ = std::fs::remove_file(&attachment.file_path);
        }
        
        Ok(())
    }
    
    pub async fn get_note_attachments(&self, note_id: i32) -> Result<Vec<Attachment>> {
        let attachments = sqlx::query_as::<_, Attachment>(
            "SELECT * FROM attachments WHERE note_id = $1 ORDER BY uploaded_at DESC"
        )
        .bind(note_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(attachments)
    }
    
    async fn get_attachment_by_id(&self, attachment_id: i32) -> Result<Attachment> {
        let attachment = sqlx::query_as::<_, Attachment>(
            "SELECT * FROM attachments WHERE attachment_id = $1"
        )
        .bind(attachment_id)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(attachment)
    }
}