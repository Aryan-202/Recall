use super::super::models::user::User;
use crate::utils::error::Result;
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    #[allow(dead_code)]
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_user_by_id(&self, user_id: i32) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn update_profile(
        &self,
        user_id: i32,
        full_name: Option<String>,
        profile_picture_url: Option<String>,
    ) -> Result<User> {
        // Build update query dynamically
        let mut query = "UPDATE users SET updated_at = NOW()".to_string();
        let mut params: Vec<String> = Vec::new();
        let mut param_count = 1;

        if let Some(name) = &full_name {
            query.push_str(&format!(", full_name = ${}", param_count));
            params.push(name.clone());
            param_count += 1;
        }

        if let Some(picture_url) = &profile_picture_url {
            if !params.is_empty() {
                query.push_str(", ");
            }
            query.push_str(&format!("profile_picture_url = ${}", param_count));
            params.push(picture_url.clone());
            param_count += 1;
        }

        query.push_str(&format!(" WHERE user_id = ${} RETURNING *", param_count));
        params.push(user_id.to_string());

        // Execute update
        let mut query_builder = sqlx::query_as::<_, User>(&query);

        for param in &params {
            query_builder = query_builder.bind(param);
        }

        let user = query_builder.fetch_one(&self.pool).await?;

        Ok(user)
    }

    pub async fn update_password(&self, user_id: i32, new_password_hash: &str) -> Result<()> {
        sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE user_id = $2")
            .bind(new_password_hash)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn verify_password(&self, user_id: i32, password: &str) -> Result<bool> {
        // This is a placeholder. In a real app, use bcrypt or argon2
        let stored_hash: String =
            sqlx::query_scalar("SELECT password_hash FROM users WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?;

        // In reality: bcrypt::verify(password, &stored_hash).map_err(|e| AppError::InternalError(e.into()))
        Ok(stored_hash == password) // Don't do this in production!
    }
}
