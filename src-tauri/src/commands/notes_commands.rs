use crate::db::models::Note;
use crate::db::operations;
use crate::db::AppState;
use tauri::State;

#[tauri::command]
pub fn create_note(
    state: State<'_, AppState>,
    title: String,
    content: String,
) -> Result<Note, String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::create_note(&conn, &title, &content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_notes(state: State<'_, AppState>) -> Result<Vec<Note>, String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::get_notes(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_note(
    state: State<'_, AppState>,
    id: String,
    title: String,
    content: String,
) -> Result<Note, String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::update_note(&conn, &id, &title, &content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_note(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::delete_note(&conn, &id).map_err(|e| e.to_string())
}
