-- Add full-text search support for notes
ALTER TABLE notes ADD COLUMN search_vector tsvector GENERATED ALWAYS AS (
    setweight(to_tsvector('english', coalesce(title, '')), 'A') ||
    setweight(to_tsvector('english', coalesce(content, '')), 'B')
) STORED;

CREATE INDEX idx_notes_search_vector ON notes USING GIN(search_vector);

-- Create function for searching notes
CREATE OR REPLACE FUNCTION search_notes(
    user_id_param INTEGER,
    search_query TEXT
) RETURNS TABLE (
    note_id INTEGER,
    title VARCHAR,
    content TEXT,
    relevance REAL
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        n.note_id,
        n.title,
        n.content,
        ts_rank(n.search_vector, websearch_to_tsquery('english', search_query)) as relevance
    FROM notes n
    WHERE n.user_id = user_id_param
        AND n.is_deleted = FALSE
        AND n.search_vector @@ websearch_to_tsquery('english', search_query)
    ORDER BY relevance DESC;
END;
$$ LANGUAGE plpgsql;