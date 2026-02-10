use chrono::{DateTime, Utc, TimeZone, Local};
use std::path::Path;

pub fn format_date(date: DateTime<Utc>) -> String {
    let local_date: DateTime<Local> = date.into();
    local_date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_date_relative(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now - date;
    
    if duration.num_seconds() < 60 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{} minutes ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_days() < 7 {
        format!("{} days ago", duration.num_days())
    } else {
        format_date(date)
    }
}

pub fn generate_slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
        .replace("--", "-")
}

pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect()
}

pub fn get_file_extension(filename: &str) -> Option<String> {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

pub fn is_image_file(filename: &str) -> bool {
    if let Some(ext) = get_file_extension(filename) {
        matches!(
            ext.as_str(),
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg"
        )
    } else {
        false
    }
}

pub fn is_document_file(filename: &str) -> bool {
    if let Some(ext) = get_file_extension(filename) {
        matches!(
            ext.as_str(),
            "pdf" | "doc" | "docx" | "txt" | "rtf" | "md" | "odt"
        )
    } else {
        false
    }
}

pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        let truncated = &text[..max_length - 3];
        format!("{}...", truncated)
    }
}

pub fn extract_first_paragraph(text: &str) -> String {
    text.split("\n\n")
        .next()
        .unwrap_or("")
        .chars()
        .take(200)
        .collect()
}