export interface Note {
    id: string;
    title: string;
    content: string;
    is_archived: boolean;
    is_pinned: boolean;
    created_at: string;
    updated_at: string;
}

export interface Tag {
    id: string;
    name: string;
}
