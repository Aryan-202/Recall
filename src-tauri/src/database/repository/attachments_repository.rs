use super::super::models::attachment::Attachment;
use crate::utils::error::Result;
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct AttachmentRepository {
    pool: Pool<Postgres>,
}

impl AttachmentRepository {
    #[allow(dead_code)]
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_attachment(&self, attachment: Attachment) -> Result<Attachment> {
        let attachment = sqlx::query_as::<_, Attachment>(
            r#"
            INSERT INTO attachments (note_id, file_name, file_path, file_size, mime_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(attachment.note_id)
        .bind(&attachment.file_name)
        .bind(&attachment.file_path)
        .bind(attachment.file_size)
        .bind(&attachment.mime_type)
        .fetch_one(&self.pool)
        .await?;

        Ok(attachment)
    }

    pub async fn get_note_attachments(&self, note_id: i32) -> Result<Vec<Attachment>> {
        let attachments = sqlx::query_as::<_, Attachment>(
            "SELECT * FROM attachments WHERE note_id = $1 ORDER BY uploaded_at DESC",
        )
        .bind(note_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(attachments)
    }

    pub async fn delete_attachment(&self, attachment_id: i32) -> Result<()> {
        let attachment =
            sqlx::query_as::<_, Attachment>("SELECT * FROM attachments WHERE attachment_id = $1")
                .bind(attachment_id)
                .fetch_one(&self.pool)
                .await?;

        let _ = std::fs::remove_file(&attachment.file_path);

        sqlx::query("DELETE FROM attachments WHERE attachment_id = $1")
            .bind(attachment_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
