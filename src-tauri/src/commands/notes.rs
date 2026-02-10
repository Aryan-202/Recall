use crate::database::models::note::NoteWithRelations;
use crate::database::repository::notes_repository::{CreateNoteDto, NoteRepository, UpdateNoteDto};
use crate::utils::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    pub folder_id: Option<i32>,
    pub tags: Vec<String>,
    pub is_pinned: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoteRequest {
    pub note_id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub folder_id: Option<Option<i32>>,
    pub tags: Option<Vec<String>>,
    pub is_pinned: Option<bool>,
    pub is_archived: Option<bool>,
}

#[tauri::command]
pub async fn create_note(
    request: CreateNoteRequest,
    repository: State<'_, NoteRepository>,
) -> Result<NoteWithRelations> {
    let dto = CreateNoteDto {
        user_id: 1, // TODO: Get from auth
        title: request.title,
        content: request.content,
        folder_id: request.folder_id,
        is_pinned: request.is_pinned,
    };

    let note = repository.create_note(dto, &request.tags).await?;
    Ok(note)
}

#[tauri::command]
pub async fn get_note(
    note_id: i32,
    repository: State<'_, NoteRepository>,
) -> Result<NoteWithRelations> {
    let note = repository
        .get_note_by_id(note_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Note not found".to_string()))?;
    Ok(note)
}

#[tauri::command]
pub async fn get_all_notes(
    repository: State<'_, NoteRepository>,
) -> Result<Vec<NoteWithRelations>> {
    let user_id = 1; // TODO: Get from auth
    let notes = repository.get_user_notes(user_id).await?;
    Ok(notes)
}

#[tauri::command]
pub async fn update_note(
    request: UpdateNoteRequest,
    repository: State<'_, NoteRepository>,
) -> Result<NoteWithRelations> {
    let dto = UpdateNoteDto {
        note_id: request.note_id,
        title: request.title,
        content: request.content,
        folder_id: request.folder_id,
        is_pinned: request.is_pinned,
        is_archived: request.is_archived,
    };

    let note = repository.update_note(dto, request.tags).await?;
    Ok(note)
}

#[tauri::command]
pub async fn delete_note(note_id: i32, repository: State<'_, NoteRepository>) -> Result<bool> {
    let user_id = 1; // TODO: Get from auth
    repository.soft_delete_note(note_id, user_id).await?;
    Ok(true)
}

#[tauri::command]
pub async fn search_notes(
    query: String,
    repository: State<'_, NoteRepository>,
) -> Result<Vec<NoteWithRelations>> {
    let user_id = 1; // TODO: Get from auth
    let notes = repository.search_notes(user_id, &query).await?;
    Ok(notes)
}

#[tauri::command]
pub async fn get_notes_by_folder(
    folder_id: i32,
    repository: State<'_, NoteRepository>,
) -> Result<Vec<NoteWithRelations>> {
    let user_id = 1; // TODO: Get from auth
    let notes = repository.get_notes_by_folder(user_id, folder_id).await?;
    Ok(notes)
}

#[tauri::command]
pub async fn get_pinned_notes(
    repository: State<'_, NoteRepository>,
) -> Result<Vec<NoteWithRelations>> {
    let user_id = 1; // TODO: Get from auth
    let notes = repository.get_pinned_notes(user_id).await?;
    Ok(notes)
}

#[tauri::command]
pub async fn get_archived_notes(
    repository: State<'_, NoteRepository>,
) -> Result<Vec<NoteWithRelations>> {
    let user_id = 1; // TODO: Get from auth
    let notes = repository.get_archived_notes(user_id).await?;
    Ok(notes)
}

#[tauri::command]
pub async fn toggle_note_pin(note_id: i32, repository: State<'_, NoteRepository>) -> Result<bool> {
    let _user_id = 1; // TODO: Get from auth
    let current = repository.get_note_by_id(note_id).await?;

    if let Some(note) = current {
        let dto = UpdateNoteDto {
            note_id,
            title: None,
            content: None,
            folder_id: None,
            is_pinned: Some(!note.note.is_pinned),
            is_archived: None,
        };
        repository.update_note(dto, None).await?;
        Ok(true)
    } else {
        Err(AppError::NotFound("Note not found".to_string()))
    }
}

#[tauri::command]
pub async fn toggle_note_archive(
    note_id: i32,
    repository: State<'_, NoteRepository>,
) -> Result<bool> {
    let _user_id = 1; // TODO: Get from auth
    let current = repository.get_note_by_id(note_id).await?;

    if let Some(note) = current {
        let dto = UpdateNoteDto {
            note_id,
            title: None,
            content: None,
            folder_id: None,
            is_pinned: None,
            is_archived: Some(!note.note.is_archived),
        };
        repository.update_note(dto, None).await?;
        Ok(true)
    } else {
        Err(AppError::NotFound("Note not found".to_string()))
    }
}
