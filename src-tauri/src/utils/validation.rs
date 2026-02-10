use crate::utils::error::{AppError, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref EMAIL_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref HEX_COLOR_REGEX: Regex = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();
}

#[allow(dead_code)]
pub fn validate_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

#[allow(dead_code)]
pub fn validate_username(username: &str) -> bool {
    username.len() >= 3 && username.len() <= 50
}

#[allow(dead_code)]
pub fn validate_password(password: &str) -> bool {
    password.len() >= 8
}

#[allow(dead_code)]
pub fn validate_hex_color(color: &str) -> bool {
    HEX_COLOR_REGEX.is_match(color)
}

#[allow(dead_code)]
pub fn validate_note_title(title: &str) -> Result<()> {
    if title.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Title cannot be empty".to_string(),
        ));
    }
    if title.len() > 255 {
        return Err(AppError::ValidationError(
            "Title cannot exceed 255 characters".to_string(),
        ));
    }
    Ok(())
}

#[allow(dead_code)]
pub fn validate_folder_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Folder name cannot be empty".to_string(),
        ));
    }
    if name.len() > 50 {
        return Err(AppError::ValidationError(
            "Folder name cannot exceed 50 characters".to_string(),
        ));
    }
    Ok(())
}

#[allow(dead_code)]
pub fn validate_tag_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(AppError::ValidationError(
            "Tag name cannot be empty".to_string(),
        ));
    }
    if name.len() > 30 {
        return Err(AppError::ValidationError(
            "Tag name cannot exceed 30 characters".to_string(),
        ));
    }
    Ok(())
}
