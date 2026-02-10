use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub struct MenuBuilder {
    menu: Menu,
}

impl MenuBuilder {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(),
        }
    }
    
    pub fn add_file_menu(mut self) -> Self {
        let file_menu = Menu::new()
            .add_item(CustomMenuItem::new("new_note", "New Note").accelerator("CmdOrCtrl+N"))
            .add_item(CustomMenuItem::new("new_folder", "New Folder").accelerator("CmdOrCtrl+Shift+N"))
            .add_separator()
            .add_item(CustomMenuItem::new("save", "Save").accelerator("CmdOrCtrl+S"))
            .add_separator()
            .add_item(CustomMenuItem::new("quit", "Quit").accelerator("CmdOrCtrl+Q"));
        
        self.menu = self.menu.add_submenu(Submenu::new("File", file_menu));
        self
    }
    
    pub fn add_edit_menu(mut self) -> Self {
        let edit_menu = Menu::new()
            .add_item(CustomMenuItem::new("undo", "Undo").accelerator("CmdOrCtrl+Z"))
            .add_item(CustomMenuItem::new("redo", "Redo").accelerator("CmdOrCtrl+Shift+Z"))
            .add_separator()
            .add_item(CustomMenuItem::new("cut", "Cut").accelerator("CmdOrCtrl+X"))
            .add_item(CustomMenuItem::new("copy", "Copy").accelerator("CmdOrCtrl+C"))
            .add_item(CustomMenuItem::new("paste", "Paste").accelerator("CmdOrCtrl+V"))
            .add_item(CustomMenuItem::new("select_all", "Select All").accelerator("CmdOrCtrl+A"));
        
        self.menu = self.menu.add_submenu(Submenu::new("Edit", edit_menu));
        self
    }
    
    pub fn add_notes_menu(mut self) -> Self {
        let notes_menu = Menu::new()
            .add_item(CustomMenuItem::new("pin_note", "Pin Note").accelerator("CmdOrCtrl+P"))
            .add_item(CustomMenuItem::new("archive_note", "Archive Note").accelerator("CmdOrCtrl+Shift+A"))
            .add_item(CustomMenuItem::new("delete_note", "Delete Note").accelerator("Delete"))
            .add_separator()
            .add_item(CustomMenuItem::new("add_tag", "Add Tag").accelerator("CmdOrCtrl+T"));
        
        self.menu = self.menu.add_submenu(Submenu::new("Notes", notes_menu));
        self
    }
    
    pub fn build(self) -> Menu {
        self.menu
    }
}