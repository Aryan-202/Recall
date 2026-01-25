#[tauri::command]
pub fn save_notes(title: String) -> String {
    println!("success in saving notes {}", title);
    return "success in saving notes".to_string();
}
