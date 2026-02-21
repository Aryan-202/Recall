use crate::db::models::Note;
use dotenv::dotenv;
use reqwest::{header, Client};
use serde_json::json;
use std::env;

pub struct SupabaseClient {
    client: Client,
    base_url: String,
}

impl SupabaseClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
        let supabase_key = env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set");

        let mut headers = header::HeaderMap::new();
        headers.insert("apikey", header::HeaderValue::from_str(&supabase_key)?);
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", supabase_key))?,
        );
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            client,
            base_url: supabase_url,
        })
    }

    pub async fn get_notes(&self, user_id: &str) -> Result<Vec<Note>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/rest/v1/notes?user_id=eq.{}&order=created_at.desc",
            self.base_url, user_id
        );

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to get notes: {}", error_text).into());
        }

        let notes: Vec<Note> = response.json().await?;
        Ok(notes)
    }

    pub async fn create_note(
        &self,
        user_id: &str,
        title: &str,
        content: &str,
    ) -> Result<Note, Box<dyn std::error::Error>> {
        let url = format!("{}/rest/v1/notes", self.base_url);

        let note_data = json!({
            "user_id": user_id,
            "title": title,
            "content": content,
        });

        let response = self
            .client
            .post(&url)
            .header("Prefer", "return=representation")
            .json(&note_data)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to create note: {}", error_text).into());
        }

        let created_note: Note = response.json().await?;
        Ok(created_note)
    }

    pub async fn update_note(
        &self,
        note_id: &str,
        title: Option<String>,
        content: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/rest/v1/notes?id=eq.{}", self.base_url, note_id);

        let mut update = serde_json::Map::new();
        if let Some(title) = title {
            update.insert("title".to_string(), json!(title));
        }
        if let Some(content) = content {
            update.insert("content".to_string(), json!(content));
        }
        update.insert(
            "updated_at".to_string(),
            json!(chrono::Utc::now().to_rfc3339()),
        );

        let response = self.client.patch(&url).json(&update).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to update note: {}", error_text).into());
        }

        Ok(())
    }

    pub async fn delete_note(&self, note_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/rest/v1/notes?id=eq.{}", self.base_url, note_id);

        let response = self.client.delete(&url).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Failed to delete note: {}", error_text).into());
        }

        Ok(())
    }
}

// Singleton instance
use once_cell::sync::OnceCell;
pub static SUPABASE_CLIENT: OnceCell<SupabaseClient> = OnceCell::new();

pub async fn init_supabase() -> Result<(), Box<dyn std::error::Error>> {
    let client = SupabaseClient::new()?;
    let _ = SUPABASE_CLIENT.set(client);
    Ok(())
}
