use crate::db::models::Tag;
use crate::db::operations;
use crate::db::AppState;
use tauri::State;

#[tauri::command]
pub fn create_tag(state: State<'_, AppState>, name: String) -> Result<Tag, String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::create_tag(&conn, &name).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_tags(state: State<'_, AppState>) -> Result<Vec<Tag>, String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::get_tags(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_tag(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::delete_tag(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_tag_to_note(
    state: State<'_, AppState>,
    note_id: String,
    tag_id: String,
) -> Result<(), String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::add_tag_to_note(&conn, &note_id, &tag_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_tag_from_note(
    state: State<'_, AppState>,
    note_id: String,
    tag_id: String,
) -> Result<(), String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::remove_tag_from_note(&conn, &note_id, &tag_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_tags_for_note(state: State<'_, AppState>, note_id: String) -> Result<Vec<Tag>, String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::get_tags_for_note(&conn, &note_id).map_err(|e| e.to_string())
}
