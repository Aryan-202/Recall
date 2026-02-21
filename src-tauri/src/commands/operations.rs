use bson::Document;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    bson_uuid: String,
    date_time: bson::DateTime,
    title: String,
    body: String,
}

#[tauri::command]
pub fn save_note(title: &str, body: &str) -> Vec<u8> {
    let note = Note {
        bson_uuid: bson::Uuid::new().to_string(),
        date_time: bson::DateTime::now(),
        title: title.to_string(),
        body: body.to_string(),
    };

    // Safe round-trip via JSON to avoid version-specific bson serialization issues
    let json = serde_json::to_string(&note).expect("Failed to serialize note to JSON");
    let doc: Document = serde_json::from_str(&json).expect("Failed to parse document from JSON");

    let mut bytes = Vec::new();
    doc.to_writer(&mut bytes).expect("Failed to write BSON");
    bytes
}

#[tauri::command]
pub fn edit_note(data: &str) -> Vec<u8> {
    let notes: Vec<Note> = serde_json::from_str(data).expect("Failed to parse JSON");
    let mut bytes = Vec::new();

    for note in notes {
        let json = serde_json::to_string(&note).expect("Failed to serialize note to JSON");
        let doc: Document =
            serde_json::from_str(&json).expect("Failed to parse document from JSON");
        doc.to_writer(&mut bytes).expect("Failed to write BSON doc");
    }

    bytes
}

#[tauri::command]
pub fn load_notes(data: &str) -> String {
    // Basic empty check
    if data.trim().is_empty() || data == "[]" {
        return String::from("no data");
    }

    // Parse the string array of bytes: e.g. "[1, 2, 3]" -> Vec<u8>
    let bytes_res: Result<Vec<u8>, _> = data
        .trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().parse::<u8>())
        .collect();

    let my_bytes = match bytes_res {
        Ok(b) => b,
        Err(_) => return String::from("error: invalid data format"),
    };

    if my_bytes.is_empty() {
        return String::from("no data");
    }

    let mut curs = Cursor::new(my_bytes);
    let mut docs = Vec::new();

    // Read BSON documents until the end of the byte array
    loop {
        match Document::from_reader(&mut curs) {
            Ok(doc) => docs.push(doc),
            Err(e) => {
                // If we hit EOF, it's fine, we're done
                if e.to_string().contains("unexpected end of file")
                    || e.to_string().contains("io error")
                {
                    // Check if we're actually at the end
                    if curs.position() >= curs.get_ref().len() as u64 {
                        break;
                    }
                }
                println!("Error reading BSON document: {:?}", e);
                break;
            }
        }
    }

    serde_json::to_string(&docs).unwrap_or_else(|_| String::from("[]"))
}
