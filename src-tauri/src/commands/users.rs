use crate::database::repository::users_repository::UserRepository;
use crate::utils::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub profile_picture_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub full_name: Option<String>,
    pub profile_picture_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[tauri::command]
pub async fn get_current_user(
    repository: State<'_, UserRepository>,
) -> Result<UserProfile> {
    let user_id = 1; // TODO: Get from auth/session
    let user = repository.get_user_by_id(user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    
    Ok(UserProfile {
        user_id: user.user_id,
        username: user.username,
        email: user.email,
        full_name: user.full_name,
        profile_picture_url: user.profile_picture_url,
        created_at: user.created_at,
    })
}

#[tauri::command]
pub async fn update_user_profile(
    request: UpdateProfileRequest,
    repository: State<'_, UserRepository>,
) -> Result<UserProfile> {
    let user_id = 1; // TODO: Get from auth
    let user = repository.update_profile(user_id, request.full_name, request.profile_picture_url).await?;
    
    Ok(UserProfile {
        user_id: user.user_id,
        username: user.username,
        email: user.email,
        full_name: user.full_name,
        profile_picture_url: user.profile_picture_url,
        created_at: user.created_at,
    })
}

#[tauri::command]
pub async fn change_password(
    request: ChangePasswordRequest,
    repository: State<'_, UserRepository>,
) -> Result<bool> {
    let user_id = 1; // TODO: Get from auth
    
    // In a real app, verify current password first
    // let is_valid = repository.verify_password(user_id, &request.current_password).await?;
    // if !is_valid {
    //     return Err(AppError::InvalidInput("Current password is incorrect".to_string()));
    // }
    
    repository.update_password(user_id, &request.new_password).await?;
    Ok(true)
}