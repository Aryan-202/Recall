use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    
    static ref USERNAME_REGEX: Regex = Regex::new(
        r"^[a-zA-Z0-9_]{3,30}$"
    ).unwrap();
    
    static ref HEX_COLOR_REGEX: Regex = Regex::new(
        r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$"
    ).unwrap();
}

pub fn validate_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

pub fn validate_username(username: &str) -> bool {
    USERNAME_REGEX.is_match(username)
}

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8
}

pub fn validate_hex_color(color: &str) -> bool {
    HEX_COLOR_REGEX.is_match(color)
}

pub fn validate_note_title(title: &str) -> Result<(), String> {
    if title.trim().is_empty() {
        return Err("Note title cannot be empty".to_string());
    }
    
    if title.len() > 255 {
        return Err("Note title is too long (max 255 characters)".to_string());
    }
    
    Ok(())
}

pub fn validate_folder_name(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Folder name cannot be empty".to_string());
    }
    
    if name.len() > 100 {
        return Err("Folder name is too long (max 100 characters)".to_string());
    }
    
    Ok(())
}

pub fn validate_tag_name(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("Tag name cannot be empty".to_string());
    }
    
    if name.len() > 50 {
        return Err("Tag name is too long (max 50 characters)".to_string());
    }
    
    Ok(())
}