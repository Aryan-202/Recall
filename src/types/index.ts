export interface Note {
    id: string;
    user_id: string;
    title: string;
    content: string;
    created_at: string;
    updated_at: string;
}

export interface User {
    id: string;
    email: string;
    created_at: string;
}

export interface NoteItemProps {
    note: Note;
    isActive: boolean;
    onClick: () => void;
}

