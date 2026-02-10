use chrono::{DateTime, Local, Utc};

#[allow(dead_code)]
pub fn format_date(date: DateTime<Utc>) -> String {
    date.with_timezone(&Local)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

#[allow(dead_code)]
pub fn format_date_relative(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(date);

    if duration.num_seconds() < 60 {
        return "just now".to_string();
    } else if duration.num_minutes() < 60 {
        return format!("{}m ago", duration.num_minutes());
    } else if duration.num_hours() < 24 {
        return format!("{}h ago", duration.num_hours());
    } else if duration.num_days() < 7 {
        return format!("{}d ago", duration.num_days());
    }

    date.format("%b %d, %Y").to_string()
}

#[allow(dead_code)]
pub fn generate_slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

#[allow(dead_code)]
pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-'))
        .collect()
}

#[allow(dead_code)]
pub fn get_file_extension(filename: &str) -> Option<String> {
    std::path::Path::new(filename)
        .extension()
        .map(|ext| ext.to_string_lossy().to_string().to_lowercase())
}

#[allow(dead_code)]
pub fn is_image_file(filename: &str) -> bool {
    let ext = get_file_extension(filename).unwrap_or_default();
    matches!(
        ext.as_str(),
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg"
    )
}

#[allow(dead_code)]
pub fn is_document_file(filename: &str) -> bool {
    let ext = get_file_extension(filename).unwrap_or_default();
    matches!(ext.as_str(), "pdf" | "doc" | "docx" | "txt" | "md" | "rtf")
}

#[allow(dead_code)]
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        return text.to_string();
    }
    format!("{}...", &text[..max_length])
}

#[allow(dead_code)]
pub fn extract_first_paragraph(text: &str) -> String {
    text.split("\n\n").next().unwrap_or("").to_string()
}
