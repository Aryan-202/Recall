use crate::database::operations::NoteRepository;
use crate::database::models::{Note, NoteWithTags};
use crate::utils::error_handling::AppError;
use tauri::State;
use std::sync::Arc;

#[tauri::command]
pub async fn create_note(
    title: String,
    content: String,
    repository: State<'_, Arc<NoteRepository>>,
) -> Result<Note, AppError> {
    repository.create_note(&title, &content)
}

#[tauri::command]
pub async fn get_all_notes(
    repository: State<'_, Arc<NoteRepository>>,
) -> Result<Vec<NoteWithTags>, AppError> {
    repository.get_all_notes()
}

#[tauri::command]
pub async fn update_note(
    id: String,
    title: String,
    content: String,
    repository: State<'_, Arc<NoteRepository>>,
) -> Result<Note, AppError> {
    repository.update_note(&id, &title, &content)
}

#[tauri::command]
pub async fn delete_note(
    id: String,
    repository: State<'_, Arc<NoteRepository>>,
) -> Result<(), AppError> {
    repository.delete_note(&id)
}