use crate::db::models::Note;
use crate::db::operations;
use crate::db::AppState;
use tauri::State;

#[tauri::command]
pub fn search_notes(state: State<'_, AppState>, query: String) -> Result<Vec<Note>, String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::search_notes(&conn, &query).map_err(|e| e.to_string())
}
