import { Note } from "../../App";
import styles from "./notes-editor.module.css";

interface NotesEditorProps {
    note: Note;
    onUpdate: (updatedNote: Note) => void;
}

const NotesEditor = ({ note, onUpdate }: NotesEditorProps) => {
    const handleTitleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        onUpdate({
            ...note,
            title: e.target.value,
            updatedAt: Date.now(),
        });
    };

    const handleContentChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        onUpdate({
            ...note,
            content: e.target.value,
            updatedAt: Date.now(),
        });
    };

    return (
        <div className={styles.editor}>
            <input
                type="text"
                className={styles.titleInput}
                value={note.title}
                onChange={handleTitleChange}
                placeholder="Note Title"
            />
            <textarea
                className={styles.textArea}
                value={note.content}
                onChange={handleContentChange}
                placeholder="Start typing..."
            />
            <div className={styles.meta}>
                Last updated: {new Date(note.updatedAt).toLocaleString()}
            </div>
        </div>
    );
};

export default NotesEditor;
