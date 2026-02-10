use crate::database::repository::tags_repository::{
    TagRepository, TagWithNotes, CreateTagDto, UpdateTagDto
};
use crate::utils::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    pub tag_id: i32,
    pub name: Option<String>,
    pub color: Option<String>,
}

#[tauri::command]
pub async fn create_tag(
    request: CreateTagRequest,
    repository: State<'_, TagRepository>,
) -> Result<TagWithNotes> {
    let dto = CreateTagDto {
        user_id: 1, // TODO: Get from auth
        name: request.name,
        color: request.color,
    };
    
    let tag = repository.create_tag(dto).await?;
    Ok(tag)
}

#[tauri::command]
pub async fn get_tag(
    tag_id: i32,
    repository: State<'_, TagRepository>,
) -> Result<TagWithNotes> {
    let tag = repository.get_tag_by_id(tag_id).await?
        .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;
    Ok(tag)
}

#[tauri::command]
pub async fn get_all_tags(
    repository: State<'_, TagRepository>,
) -> Result<Vec<TagWithNotes>> {
    let user_id = 1; // TODO: Get from auth
    let tags = repository.get_user_tags(user_id).await?;
    Ok(tags)
}

#[tauri::command]
pub async fn update_tag(
    request: UpdateTagRequest,
    repository: State<'_, TagRepository>,
) -> Result<TagWithNotes> {
    let dto = UpdateTagDto {
        tag_id: request.tag_id,
        name: request.name,
        color: request.color,
    };
    
    let tag = repository.update_tag(dto).await?;
    Ok(tag)
}

#[tauri::command]
pub async fn delete_tag(
    tag_id: i32,
    repository: State<'_, TagRepository>,
) -> Result<bool> {
    let user_id = 1; // TODO: Get from auth
    repository.delete_tag(tag_id, user_id).await?;
    Ok(true)
}

#[tauri::command]
pub async fn assign_tag_to_note(
    note_id: i32,
    tag_id: i32,
    repository: State<'_, TagRepository>,
) -> Result<bool> {
    repository.assign_tag_to_note(note_id, tag_id).await?;
    Ok(true)
}

#[tauri::command]
pub async fn remove_tag_from_note(
    note_id: i32,
    tag_id: i32,
    repository: State<'_, TagRepository>,
) -> Result<bool> {
    repository.remove_tag_from_note(note_id, tag_id).await?;
    Ok(true)
}

#[tauri::command]
pub async fn get_notes_by_tag(
    tag_id: i32,
    repository: State<'_, TagRepository>,
) -> Result<Vec<crate::database::models::note::Note>> {
    let notes = repository.get_notes_by_tag(tag_id).await?;
    Ok(notes)
}