import { Note } from "../../App";
import styles from "./sidebar.module.css";

interface SidebarProps {
  notes: Note[];
  activeNoteId: string | null;
  setActiveNoteId: (id: string) => void;
  createNote: () => void;
  deleteNote: (id: string) => void;
}

const Sidebar = ({
  notes,
  activeNoteId,
  setActiveNoteId,
  createNote,
  deleteNote,
}: SidebarProps) => {
  return (
    <aside className={styles.sidebar}>
      <div className={styles.header}>
        <h2>Notes</h2>
        <button className={styles.newButton} onClick={createNote}>
          + New
        </button>
      </div>
      <div className={styles.notesList}>
        {notes.map((note) => (
          <div
            key={note.id}
            className={`${styles.noteItem} ${activeNoteId === note.id ? styles.activeNote : ""
              }`}
            onClick={() => setActiveNoteId(note.id)}
          >
            <span className={styles.noteTitle}>
              {note.title || "Untitled Note"}
            </span>
            <span className={styles.noteSnippet}>
              {note.content.substring(0, 30) || "No content..."}
            </span>
            <button
              className={styles.deleteButton}
              onClick={(e) => {
                e.stopPropagation();
                deleteNote(note.id);
              }}
              title="Delete Note"
            >
              ×
            </button>
          </div>
        ))}
      </div>
    </aside>
  );
};

export default Sidebar;