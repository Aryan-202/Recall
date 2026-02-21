// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
mod commands;
mod db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize Supabase client
    tauri::async_runtime::spawn(async {
        if let Err(e) = db::supabase_clients::init_supabase().await {
            eprintln!("Failed to initialize Supabase: {}", e);
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            crate::commands::operations::save_note,
            crate::commands::operations::edit_note,
            crate::commands::operations::load_notes,
            // Supabase notes commands
            crate::commands::supabase::get_notes,
            crate::commands::supabase::create_note,
            crate::commands::supabase::update_note,
            crate::commands::supabase::delete_note,
            // Auth commands
            crate::commands::auth::login,
            crate::commands::auth::signup,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
