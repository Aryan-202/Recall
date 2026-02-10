pub mod builder;

#[allow(unused_imports)]
pub use builder::*;

use tauri::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Emitter, Runtime};

pub fn build_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    let file_menu = Submenu::with_items(
        app,
        "File",
        true,
        &[
            &MenuItem::with_id(app, "new_note", "New Note", true, Some("CmdOrCtrl+N"))?,
            &MenuItem::with_id(
                app,
                "new_folder",
                "New Folder",
                true,
                Some("CmdOrCtrl+Shift+N"),
            )?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "save", "Save", false, Some("CmdOrCtrl+S"))?,
            &MenuItem::with_id(
                app,
                "save_as",
                "Save As...",
                true,
                Some("CmdOrCtrl+Shift+S"),
            )?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "import", "Import...", true, None::<&str>)?,
            &MenuItem::with_id(app, "export", "Export...", true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "print", "Print...", true, Some("CmdOrCtrl+P"))?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "quit", "Quit", true, Some("CmdOrCtrl+Q"))?,
        ],
    )?;

    let edit_menu = Submenu::with_items(
        app,
        "Edit",
        true,
        &[
            &MenuItem::with_id(app, "undo", "Undo", true, Some("CmdOrCtrl+Z"))?,
            &MenuItem::with_id(app, "redo", "Redo", true, Some("CmdOrCtrl+Shift+Z"))?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "cut", "Cut", true, Some("CmdOrCtrl+X"))?,
            &MenuItem::with_id(app, "copy", "Copy", true, Some("CmdOrCtrl+C"))?,
            &MenuItem::with_id(app, "paste", "Paste", true, Some("CmdOrCtrl+V"))?,
            &MenuItem::with_id(app, "select_all", "Select All", true, Some("CmdOrCtrl+A"))?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "find", "Find", true, Some("CmdOrCtrl+F"))?,
            &MenuItem::with_id(app, "replace", "Replace", true, Some("CmdOrCtrl+H"))?,
        ],
    )?;

    let view_menu = Submenu::with_items(
        app,
        "View",
        true,
        &[
            &MenuItem::with_id(
                app,
                "toggle_sidebar",
                "Toggle Sidebar",
                true,
                Some("CmdOrCtrl+B"),
            )?,
            &MenuItem::with_id(
                app,
                "toggle_preview",
                "Toggle Preview",
                true,
                Some("CmdOrCtrl+E"),
            )?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "zoom_in", "Zoom In", true, Some("CmdOrCtrl+Plus"))?,
            &MenuItem::with_id(app, "zoom_out", "Zoom Out", true, Some("CmdOrCtrl+-"))?,
            &MenuItem::with_id(app, "reset_zoom", "Reset Zoom", true, Some("CmdOrCtrl+0"))?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(
                app,
                "toggle_fullscreen",
                "Toggle Fullscreen",
                true,
                Some("F11"),
            )?,
        ],
    )?;

    let notes_menu = Submenu::with_items(
        app,
        "Notes",
        true,
        &[
            &MenuItem::with_id(app, "pin_note", "Pin/Unpin Note", true, Some("CmdOrCtrl+P"))?,
            &MenuItem::with_id(
                app,
                "archive_note",
                "Archive/Unarchive Note",
                true,
                Some("CmdOrCtrl+Shift+A"),
            )?,
            &MenuItem::with_id(app, "delete_note", "Delete Note", true, Some("Delete"))?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "add_tag", "Add Tag...", true, Some("CmdOrCtrl+T"))?,
            &MenuItem::with_id(
                app,
                "add_attachment",
                "Add Attachment...",
                true,
                Some("CmdOrCtrl+Shift+A"),
            )?,
        ],
    )?;

    let help_menu = Submenu::with_items(
        app,
        "Help",
        true,
        &[
            &MenuItem::with_id(app, "about", "About Recall Notes", true, None::<&str>)?,
            &MenuItem::with_id(app, "documentation", "Documentation", true, None::<&str>)?,
            &MenuItem::with_id(
                app,
                "check_updates",
                "Check for Updates",
                true,
                None::<&str>,
            )?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "report_issue", "Report Issue", true, None::<&str>)?,
        ],
    )?;

    Menu::with_items(
        app,
        &[&file_menu, &edit_menu, &view_menu, &notes_menu, &help_menu],
    )
}

pub fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    match event.id.as_ref() {
        "new_note" => {
            println!("New Note command triggered");
            let _ = app.emit("menu:new-note", "");
        }
        "new_folder" => {
            println!("New Folder command triggered");
            let _ = app.emit("menu:new-folder", "");
        }
        "quit" => {
            println!("Quit command triggered");
            app.exit(0);
        }
        "about" => {
            println!("About command triggered");
        }
        _ => {}
    }
}
