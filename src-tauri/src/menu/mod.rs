pub mod builder;

pub use builder::*;

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

pub fn build_menu() -> Menu {
    Menu::new()
        .add_submenu(Submenu::new(
            "File",
            Menu::new()
                .add_item(CustomMenuItem::new("new_note", "New Note").accelerator("CmdOrCtrl+N"))
                .add_item(CustomMenuItem::new("new_folder", "New Folder").accelerator("CmdOrCtrl+Shift+N"))
                .add_separator()
                .add_item(CustomMenuItem::new("save", "Save").accelerator("CmdOrCtrl+S").enabled(false))
                .add_item(CustomMenuItem::new("save_as", "Save As...").accelerator("CmdOrCtrl+Shift+S"))
                .add_separator()
                .add_item(CustomMenuItem::new("import", "Import..."))
                .add_item(CustomMenuItem::new("export", "Export..."))
                .add_separator()
                .add_item(CustomMenuItem::new("print", "Print...").accelerator("CmdOrCtrl+P"))
                .add_separator()
                .add_item(CustomMenuItem::new("quit", "Quit").accelerator("CmdOrCtrl+Q")),
        ))
        .add_submenu(Submenu::new(
            "Edit",
            Menu::new()
                .add_item(CustomMenuItem::new("undo", "Undo").accelerator("CmdOrCtrl+Z"))
                .add_item(CustomMenuItem::new("redo", "Redo").accelerator("CmdOrCtrl+Shift+Z"))
                .add_separator()
                .add_item(CustomMenuItem::new("cut", "Cut").accelerator("CmdOrCtrl+X"))
                .add_item(CustomMenuItem::new("copy", "Copy").accelerator("CmdOrCtrl+C"))
                .add_item(CustomMenuItem::new("paste", "Paste").accelerator("CmdOrCtrl+V"))
                .add_item(CustomMenuItem::new("select_all", "Select All").accelerator("CmdOrCtrl+A"))
                .add_separator()
                .add_item(CustomMenuItem::new("find", "Find").accelerator("CmdOrCtrl+F"))
                .add_item(CustomMenuItem::new("replace", "Replace").accelerator("CmdOrCtrl+H")),
        ))
        .add_submenu(Submenu::new(
            "View",
            Menu::new()
                .add_item(CustomMenuItem::new("toggle_sidebar", "Toggle Sidebar").accelerator("CmdOrCtrl+B"))
                .add_item(CustomMenuItem::new("toggle_preview", "Toggle Preview").accelerator("CmdOrCtrl+E"))
                .add_separator()
                .add_item(CustomMenuItem::new("zoom_in", "Zoom In").accelerator("CmdOrCtrl+Plus"))
                .add_item(CustomMenuItem::new("zoom_out", "Zoom Out").accelerator("CmdOrCtrl+-"))
                .add_item(CustomMenuItem::new("reset_zoom", "Reset Zoom").accelerator("CmdOrCtrl+0"))
                .add_separator()
                .add_item(CustomMenuItem::new("toggle_fullscreen", "Toggle Fullscreen").accelerator("F11")),
        ))
        .add_submenu(Submenu::new(
            "Notes",
            Menu::new()
                .add_item(CustomMenuItem::new("pin_note", "Pin/Unpin Note").accelerator("CmdOrCtrl+P"))
                .add_item(CustomMenuItem::new("archive_note", "Archive/Unarchive Note").accelerator("CmdOrCtrl+Shift+A"))
                .add_item(CustomMenuItem::new("delete_note", "Delete Note").accelerator("Delete"))
                .add_separator()
                .add_item(CustomMenuItem::new("add_tag", "Add Tag...").accelerator("CmdOrCtrl+T"))
                .add_item(CustomMenuItem::new("add_attachment", "Add Attachment...").accelerator("CmdOrCtrl+Shift+A")),
        ))
        .add_submenu(Submenu::new(
            "Help",
            Menu::new()
                .add_item(CustomMenuItem::new("about", "About Recall Notes"))
                .add_item(CustomMenuItem::new("documentation", "Documentation"))
                .add_item(CustomMenuItem::new("check_updates", "Check for Updates"))
                .add_separator()
                .add_item(CustomMenuItem::new("report_issue", "Report Issue")),
        ))
}

pub fn handle_menu_event(app: &tauri::AppHandle, event: WindowMenuEvent) {
    match event.menu_item_id() {
        "new_note" => {
            println!("New Note command triggered");
            // Emit event to frontend
            let _ = app.emit_all("menu:new-note", "");
        }
        "new_folder" => {
            println!("New Folder command triggered");
            let _ = app.emit_all("menu:new-folder", "");
        }
        "quit" => {
            println!("Quit command triggered");
            app.exit(0);
        }
        "about" => {
            println!("About command triggered");
            // You could open an about window here
        }
        _ => {}
    }
}