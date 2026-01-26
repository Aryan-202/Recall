pub mod commands;
pub mod db;

use crate::db::AppState;
use std::sync::Mutex;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let conn = db::init_db();
            app.manage(AppState {
                db: Mutex::new(conn),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::notes_commands::create_note,
            commands::notes_commands::get_all_notes,
            commands::notes_commands::update_note,
            commands::notes_commands::delete_note,
            commands::tag_commands::create_tag,
            commands::tag_commands::get_all_tags,
            commands::tag_commands::delete_tag,
            commands::tag_commands::add_tag_to_note,
            commands::tag_commands::remove_tag_from_note,
            commands::tag_commands::get_tags_for_note,
            commands::search_commands::search_notes,
            commands::file_commands::export_note,
            commands::file_commands::import_note_from_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
