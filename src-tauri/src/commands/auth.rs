use crate::db::{models::AuthResponse, supabase_clients::SUPABASE_CLIENT};
use tauri::command;

#[command]
pub async fn login(email: String, password: String) -> Result<AuthResponse, String> {
    let client = SUPABASE_CLIENT
        .get()
        .ok_or("Supabase client not initialized")?;

    let res = client.login(&email, &password).await;
    match res {
        Ok(auth) => Ok(auth),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn signup(email: String, password: String) -> Result<AuthResponse, String> {
    let client = SUPABASE_CLIENT
        .get()
        .ok_or("Supabase client not initialized")?;

    let res = client.signup(&email, &password).await;
    match res {
        Ok(auth) => Ok(auth),
        Err(e) => Err(e.to_string()),
    }
}
