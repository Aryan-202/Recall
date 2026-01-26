pub mod models;
pub mod operations;
pub mod schema;

use rusqlite::Connection;
use std::sync::Mutex;

pub struct AppState {
    pub db: Mutex<Connection>,
}

pub fn init_db() -> Connection {
    let conn = Connection::open("recall.db").expect("Failed to open database");

    conn.execute(schema::CREATE_NOTES_TABLE, ())
        .expect("Failed to create notes table");
    conn.execute(schema::CREATE_TAGS_TABLE, ())
        .expect("Failed to create tags table");
    conn.execute(schema::CREATE_NOTE_TAGS_TABLE, ())
        .expect("Failed to create note_tags table");

    conn
}
