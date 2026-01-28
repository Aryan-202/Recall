export interface Note {
    id: string;
    title: string;
    content: string;
    is_archived: boolean;
    is_pinned: boolean;
    created_at: string;
    updated_at: string;
    folder_id?: string;
    tag_ids?: string[];
}

export interface Tag {
    id: string;
    name: string;
    created_at: string;
    updated_at: string; 
}

export interface NoteFolder {
    id: string;
    name: string;
    is_archived: boolean;
    is_pinned: boolean;
    note_count: number;
    note_ids: string[];
    created_at: string;
    updated_at: string;
}