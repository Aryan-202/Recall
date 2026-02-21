import { useState, useEffect, useCallback } from 'react';
import { Note } from '../types';
import { noteService } from '../services/noteService';

export const useNotes = () => {
    const [notes, setNotes] = useState<Note[]>([]);
    const [loading, setLoading] = useState(true);
    const [activeNoteId, setActiveNoteId] = useState<string | null>(null);
    const [searchQuery, setSearchQuery] = useState('');

    // Local storage to persist the "bytes" for now as a mock of the DB
    // In a real app, this would be handled better by the backend.
    // But based on the Rust code, load_notes expects a string representation of bytes.

    const loadFromBackend = useCallback(async () => {
        try {
            const savedBytes = localStorage.getItem('recall_notes_bytes') || '[]';
            const fetchedNotes = await noteService.loadNotes(savedBytes);
            setNotes(fetchedNotes);
        } catch (error) {
            console.error('Failed to load notes', error);
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => {
        loadFromBackend();
    }, [loadFromBackend]);

    const saveToBackend = async (updatedNotes: Note[]) => {
        const bytes = await noteService.editNote(updatedNotes);
        localStorage.setItem('recall_notes_bytes', JSON.stringify(bytes));
        setNotes(updatedNotes);
    };

    const handleNewNote = async () => {
        await noteService.saveNote("New Note", "");
        // This is a bit tricky because save_note in Rust returns bytes of ONE note
        // We need to decode it back or just manually add it to our list for now
        // Since load_notes decodes everything, let's just refresh after a save if we had a better backend
        // But for now, let's manually update:
        const newNote: Note = {
            bson_uuid: crypto.randomUUID(),
            title: "New Note",
            body: "",
            date_time: { $date: { $numberLong: Date.now().toString() } }
        };

        const newNotes = [newNote, ...notes];
        await saveToBackend(newNotes);
        setActiveNoteId(newNote.bson_uuid);
    };

    const handleUpdateNote = async (id: string, title: string, body: string) => {
        const updatedNotes = notes.map(n =>
            n.bson_uuid === id ? { ...n, title, body } : n
        );
        await saveToBackend(updatedNotes);
    };

    const handleDeleteNote = async (id: string) => {
        const updatedNotes = notes.filter(n => n.bson_uuid !== id);
        await saveToBackend(updatedNotes);
        if (activeNoteId === id) setActiveNoteId(null);
    };

    const filteredNotes = notes.filter(n =>
        n.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        n.body.toLowerCase().includes(searchQuery.toLowerCase())
    );

    const activeNote = notes.find(n => n.bson_uuid === activeNoteId) || null;

    return {
        notes: filteredNotes,
        activeNote,
        activeNoteId,
        setActiveNoteId,
        loading,
        searchQuery,
        setSearchQuery,
        handleNewNote,
        handleUpdateNote,
        handleDeleteNote
    };
};
