#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod config;
mod database;
mod menu;
mod utils;

use commands::*;
use database::init_db;
use tauri::Manager;
use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting Recall Notes App...");

    tauri::Builder::default()
        .setup(|app| {
            let _handle = app.handle();

            // Initialize database
            tokio::spawn(async move {
                if let Err(e) = init_db().await {
                    eprintln!("Failed to initialize database: {}", e);
                }
            });

            // Set window title
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Recall - Notes App").unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Notes commands
            create_note,
            get_note,
            get_all_notes,
            update_note,
            delete_note,
            search_notes,
            get_notes_by_folder,
            get_pinned_notes,
            get_archived_notes,
            toggle_note_pin,
            toggle_note_archive,
            // Folders commands
            create_folder,
            get_folder,
            get_all_folders,
            update_folder,
            delete_folder,
            get_folder_tree,
            // Tags commands
            create_tag,
            get_tag,
            get_all_tags,
            update_tag,
            delete_tag,
            assign_tag_to_note,
            remove_tag_from_note,
            get_notes_by_tag,
            // User commands
            get_current_user,
            update_user_profile,
            change_password,
            // File operations
            upload_attachment,
            delete_attachment,
            get_note_attachments,
        ])
        .menu(menu::build_menu)
        .on_menu_event(menu::handle_menu_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
