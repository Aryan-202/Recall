use crate::db::models::Note;
use crate::db::operations;
use crate::db::AppState;
use std::fs;
use std::path::Path;
use tauri::State;

#[tauri::command]
pub fn export_note(state: State<'_, AppState>, id: String, path: String) -> Result<(), String> {
    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;

    // Fetch the note
    let note = operations::get_note(&conn, &id).map_err(|e| e.to_string())?;

    // Format content (simple markdown export)
    let file_content = format!(
        "---\ntitle: {}\ncreated_at: {}\n---\n\n{}",
        note.title, note.created_at, note.content
    );

    // Write to file
    fs::write(&path, file_content).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn import_note_from_file(state: State<'_, AppState>, path: String) -> Result<Note, String> {
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;

    // Simple parsing strategy:
    // If it starts with YAML frontmatter, try to extract title.
    // Otherwise, use filename as title.

    let path_obj = Path::new(&path);
    let filename = path_obj
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string();

    let title = filename; // Simplify for now, could parse frontmatter later if needed

    let conn = state
        .db
        .lock()
        .map_err(|_| "Failed to lock database mutex".to_string())?;
    operations::create_note(&conn, &title, &content).map_err(|e| e.to_string())
}
