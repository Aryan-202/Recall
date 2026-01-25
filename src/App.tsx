import { useState, useEffect } from "react";
import "./App.css";
import Sidebar from "./components/sidebar/Sidebar";
import NotesEditor from "./components/notes-editor/NotesEditor";

export interface Note {
  id: string;
  title: string;
  content: string;
  updatedAt: number;
}

export default function App() {
  const [notes, setNotes] = useState<Note[]>(() => {
    const saved = localStorage.getItem("tauri-notes");
    return saved ? JSON.parse(saved) : [];
  });
  const [activeNoteId, setActiveNoteId] = useState<string | null>(null);

  useEffect(() => {
    localStorage.setItem("tauri-notes", JSON.stringify(notes));
  }, [notes]);

  const activeNote = notes.find((note) => note.id === activeNoteId);

  const createNote = () => {
    const newNote: Note = {
      id: crypto.randomUUID(),
      title: "Untitled Note",
      content: "",
      updatedAt: Date.now(),
    };
    setNotes([newNote, ...notes]);
    setActiveNoteId(newNote.id);
  };

  const updateNote = (updatedNote: Note) => {
    setNotes(
      notes.map((note) => (note.id === updatedNote.id ? updatedNote : note))
    );
  };

  const deleteNote = (id: string) => {
    setNotes(notes.filter((note) => note.id !== id));
    if (activeNoteId === id) {
      setActiveNoteId(null);
    }
  };

  return (
    <main className="container">
      <Sidebar
        notes={notes}
        activeNoteId={activeNoteId}
        setActiveNoteId={setActiveNoteId}
        createNote={createNote}
        deleteNote={deleteNote}
      />
      <div className="editor-container">
        {activeNote ? (
          <NotesEditor note={activeNote} onUpdate={updateNote} />
        ) : (
          <div className="empty-state">
            <h1>Select a note to view</h1>
            <p>Choose a note from the sidebar or create a new one to get started.</p>
          </div>
        )}
      </div>
    </main>
  );
}