use crate::db::models::{Note, Tag};
use chrono::Utc;
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

pub fn create_note(conn: &Connection, title: &str, content: &str) -> Result<Note> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now();

    conn.execute(
        "INSERT INTO notes (id, title, content, created_at, updated_at, is_archived, is_pinned) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, title, content, now, now, false, false],
    )?;

    Ok(Note {
        id,
        title: title.to_string(),
        content: content.to_string(),
        is_archived: false,
        is_pinned: false,
        created_at: now,
        updated_at: now,
    })
}

pub fn get_notes(conn: &Connection) -> Result<Vec<Note>> {
    let mut stmt = conn.prepare("SELECT id, title, content, is_archived, is_pinned, created_at, updated_at FROM notes ORDER BY updated_at DESC")?;
    let note_iter = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            is_archived: row.get(3)?,
            is_pinned: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;

    let mut notes = Vec::new();
    for note in note_iter {
        notes.push(note?);
    }
    Ok(notes)
}

pub fn get_note(conn: &Connection, id: &str) -> Result<Note> {
    conn.query_row(
        "SELECT id, title, content, is_archived, is_pinned, created_at, updated_at FROM notes WHERE id = ?1",
        params![id],
        |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                is_archived: row.get(3)?,
                is_pinned: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        },
    )
}

pub fn update_note(conn: &Connection, id: &str, title: &str, content: &str) -> Result<Note> {
    let now = Utc::now();
    conn.execute(
        "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
        params![title, content, now, id],
    )?;

    // Return the updated note
    get_note(conn, id)
}

pub fn delete_note(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
    Ok(())
}

// Tag Operations

pub fn create_tag(conn: &Connection, name: &str) -> Result<Tag> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO tags (id, name) VALUES (?1, ?2)",
        params![id, name],
    )?;
    Ok(Tag {
        id,
        name: name.to_string(),
    })
}

pub fn get_tags(conn: &Connection) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare("SELECT id, name FROM tags ORDER BY name ASC")?;
    let tag_iter = stmt.query_map([], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let mut tags = Vec::new();
    for tag in tag_iter {
        tags.push(tag?);
    }
    Ok(tags)
}

pub fn delete_tag(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn search_notes(conn: &Connection, query: &str) -> Result<Vec<Note>> {
    let like_query = format!("%{}%", query);
    let mut stmt = conn.prepare("SELECT id, title, content, is_archived, is_pinned, created_at, updated_at FROM notes WHERE title LIKE ?1 OR content LIKE ?1 ORDER BY updated_at DESC")?;
    let note_iter = stmt.query_map(params![like_query], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            is_archived: row.get(3)?,
            is_pinned: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;

    let mut notes = Vec::new();
    for note in note_iter {
        notes.push(note?);
    }

    Ok(notes)
}

// Note-Tag Association Operations

pub fn add_tag_to_note(conn: &Connection, note_id: &str, tag_id: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
        params![note_id, tag_id],
    )?;
    Ok(())
}

pub fn remove_tag_from_note(conn: &Connection, note_id: &str, tag_id: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM note_tags WHERE note_id = ?1 AND tag_id = ?2",
        params![note_id, tag_id],
    )?;
    Ok(())
}

pub fn get_tags_for_note(conn: &Connection, note_id: &str) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name FROM tags t 
         INNER JOIN note_tags nt ON t.id = nt.tag_id 
         WHERE nt.note_id = ?1",
    )?;

    let tag_iter = stmt.query_map(params![note_id], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let mut tags = Vec::new();
    for tag in tag_iter {
        tags.push(tag?);
    }
    Ok(tags)
}
