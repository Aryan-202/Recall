-- Users table
CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(100),
    profile_picture_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Folders table (supports nesting)
CREATE TABLE folders (
    folder_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    parent_folder_id INTEGER REFERENCES folders(folder_id) ON DELETE CASCADE,
    color VARCHAR(7),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, name, parent_folder_id)
);

-- Notes table
CREATE TABLE notes (
    note_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    folder_id INTEGER REFERENCES folders(folder_id) ON DELETE SET NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    is_pinned BOOLEAN DEFAULT FALSE,
    is_archived BOOLEAN DEFAULT FALSE,
    is_deleted BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Tags table
CREATE TABLE tags (
    tag_id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    name VARCHAR(50) NOT NULL,
    color VARCHAR(7),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, name)
);

-- Note-tags junction table (many-to-many)
CREATE TABLE note_tags (
    note_id INTEGER NOT NULL REFERENCES notes(note_id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(tag_id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    PRIMARY KEY (note_id, tag_id)
);

-- Attachments table
CREATE TABLE attachments (
    attachment_id SERIAL PRIMARY KEY,
    note_id INTEGER NOT NULL REFERENCES notes(note_id) ON DELETE CASCADE,
    file_name VARCHAR(255) NOT NULL,
    file_path TEXT NOT NULL,
    file_size BIGINT NOT NULL,
    mime_type VARCHAR(100) NOT NULL,
    uploaded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Shared notes table (for collaboration)
CREATE TABLE shared_notes (
    share_id SERIAL PRIMARY KEY,
    note_id INTEGER NOT NULL REFERENCES notes(note_id) ON DELETE CASCADE,
    owner_user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    shared_with_user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    permission_level VARCHAR(10) NOT NULL CHECK (permission_level IN ('view', 'edit')),
    shared_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(note_id, shared_with_user_id)
);

-- Create indexes for better performance
CREATE INDEX idx_notes_user_id ON notes(user_id);
CREATE INDEX idx_notes_folder_id ON notes(folder_id);
CREATE INDEX idx_notes_created_at ON notes(created_at);
CREATE INDEX idx_notes_is_pinned ON notes(is_pinned) WHERE is_pinned = TRUE;
CREATE INDEX idx_notes_is_archived ON notes(is_archived) WHERE is_archived = TRUE;
CREATE INDEX idx_folders_user_id ON folders(user_id);
CREATE INDEX idx_folders_parent_id ON folders(parent_folder_id);
CREATE INDEX idx_tags_user_id ON tags(user_id);
CREATE INDEX idx_note_tags_note_id ON note_tags(note_id);
CREATE INDEX idx_note_tags_tag_id ON note_tags(tag_id);
CREATE INDEX idx_attachments_note_id ON attachments(note_id);
CREATE INDEX idx_shared_notes_note_id ON shared_notes(note_id);
CREATE INDEX idx_shared_notes_shared_with ON shared_notes(shared_with_user_id);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply triggers to tables
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_folders_updated_at BEFORE UPDATE ON folders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_notes_updated_at BEFORE UPDATE ON notes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert a default user for development
INSERT INTO users (username, email, password_hash, full_name) VALUES
('demo', 'demo@example.com', '$2b$12$YourHashedPasswordHere', 'Demo User');