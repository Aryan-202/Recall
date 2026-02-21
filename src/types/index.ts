export interface Note {
    bson_uuid: string;
    date_time: { $date: { $numberLong: string } } | string;
    title: string;
    body: string;
}

export interface NoteItemProps {
    note: Note;
    isActive: boolean;
    onClick: () => void;
}
