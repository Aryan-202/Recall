pub const CREATE_NOTES_TABLE: &str = "
CREATE TABLE IF NOT EXISTS notes (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    is_pinned BOOLEAN DEFAULT 0,
    is_archived BOOLEAN DEFAULT 0,
    color TEXT
);
";

pub const CREATE_TAGS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS tags (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    color TEXT
);
";

pub const CREATE_NOTE_TAGS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS note_tags (
    note_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    PRIMARY KEY (note_id, tag_id),
    FOREIGN KEY (note_id) REFERENCES notes(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
";

pub const CREATE_INDEXES: [&str; 4] = [
    "CREATE INDEX IF NOT EXISTS idx_notes_created_at ON notes(created_at);",
    "CREATE INDEX IF NOT EXISTS idx_notes_updated_at ON notes(updated_at);",
    "CREATE INDEX IF NOT EXISTS idx_notes_is_pinned ON notes(is_pinned);",
    "CREATE INDEX IF NOT EXISTS idx_note_tags_note_id ON note_tags(note_id);",
];