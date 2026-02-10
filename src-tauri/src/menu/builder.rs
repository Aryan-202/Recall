use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Runtime};

#[allow(dead_code)]
pub struct MenuBuilder<R: Runtime> {
    app: AppHandle<R>,
    items: Vec<tauri::menu::MenuItemKind<R>>,
}

impl<R: Runtime> MenuBuilder<R> {
    #[allow(dead_code)]
    pub fn new(app: AppHandle<R>) -> Self {
        Self {
            app,
            items: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_file_menu(mut self) -> tauri::Result<Self> {
        let file_menu = Submenu::with_items(
            &self.app,
            "File",
            true,
            &[
                &MenuItem::with_id(&self.app, "new_note", "New Note", true, Some("CmdOrCtrl+N"))?,
                &MenuItem::with_id(
                    &self.app,
                    "new_folder",
                    "New Folder",
                    true,
                    Some("CmdOrCtrl+Shift+N"),
                )?,
                &PredefinedMenuItem::separator(&self.app)?,
                &MenuItem::with_id(&self.app, "save", "Save", true, Some("CmdOrCtrl+S"))?,
                &PredefinedMenuItem::separator(&self.app)?,
                &MenuItem::with_id(&self.app, "quit", "Quit", true, Some("CmdOrCtrl+Q"))?,
            ],
        )?;

        self.items
            .push(tauri::menu::MenuItemKind::Submenu(file_menu));
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn add_edit_menu(mut self) -> tauri::Result<Self> {
        let edit_menu = Submenu::with_items(
            &self.app,
            "Edit",
            true,
            &[
                &MenuItem::with_id(&self.app, "undo", "Undo", true, Some("CmdOrCtrl+Z"))?,
                &MenuItem::with_id(&self.app, "redo", "Redo", true, Some("CmdOrCtrl+Shift+Z"))?,
                &PredefinedMenuItem::separator(&self.app)?,
                &MenuItem::with_id(&self.app, "cut", "Cut", true, Some("CmdOrCtrl+X"))?,
                &MenuItem::with_id(&self.app, "copy", "Copy", true, Some("CmdOrCtrl+C"))?,
                &MenuItem::with_id(&self.app, "paste", "Paste", true, Some("CmdOrCtrl+V"))?,
                &MenuItem::with_id(
                    &self.app,
                    "select_all",
                    "Select All",
                    true,
                    Some("CmdOrCtrl+A"),
                )?,
            ],
        )?;

        self.items
            .push(tauri::menu::MenuItemKind::Submenu(edit_menu));
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn add_notes_menu(mut self) -> tauri::Result<Self> {
        let notes_menu = Submenu::with_items(
            &self.app,
            "Notes",
            true,
            &[
                &MenuItem::with_id(&self.app, "pin_note", "Pin Note", true, Some("CmdOrCtrl+P"))?,
                &MenuItem::with_id(
                    &self.app,
                    "archive_note",
                    "Archive Note",
                    true,
                    Some("CmdOrCtrl+Shift+A"),
                )?,
                &MenuItem::with_id(
                    &self.app,
                    "delete_note",
                    "Delete Note",
                    true,
                    Some("Delete"),
                )?,
                &PredefinedMenuItem::separator(&self.app)?,
                &MenuItem::with_id(&self.app, "add_tag", "Add Tag", true, Some("CmdOrCtrl+T"))?,
            ],
        )?;

        self.items
            .push(tauri::menu::MenuItemKind::Submenu(notes_menu));
        Ok(self)
    }

    #[allow(dead_code)]
    pub fn build(self) -> tauri::Result<Menu<R>> {
        Menu::with_items(
            &self.app,
            &self
                .items
                .iter()
                .map(|i| i as &dyn tauri::menu::IsMenuItem<R>)
                .collect::<Vec<_>>(),
        )
    }
}
