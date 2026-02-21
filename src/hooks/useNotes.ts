import { useState, useEffect, useCallback } from 'react';
import { Note, User } from '../types';
import { noteService } from '../services/noteService';

export const useNotes = (user: User | null) => {
    const [notes, setNotes] = useState<Note[]>([]);
    const [loading, setLoading] = useState(true);
    const [activeNoteId, setActiveNoteId] = useState<string | null>(null);
    const [searchQuery, setSearchQuery] = useState('');

    const fetchNotes = useCallback(async () => {
        if (!user) {
            setNotes([]);
            setLoading(false);
            return;
        }

        setLoading(true);
        try {
            const fetchedNotes = await noteService.getNotes(user.id);
            setNotes(fetchedNotes);
        } catch (error) {
            console.error('Failed to fetch notes', error);
        } finally {
            setLoading(false);
        }
    }, [user]);

    useEffect(() => {
        fetchNotes();
    }, [fetchNotes]);

    const handleNewNote = async () => {
        if (!user) return;

        try {
            const newNote = await noteService.createNote(user.id, "New Note", "");
            setNotes(prev => [newNote, ...prev]);
            setActiveNoteId(newNote.id);
        } catch (error) {
            console.error("Error creating note", error);
        }
    };

    const handleUpdateNote = async (id: string, title: string, content: string) => {
        try {
            await noteService.updateNote(id, title, content);
            setNotes(prev => prev.map(n =>
                n.id === id ? { ...n, title, content, updated_at: new Date().toISOString() } : n
            ));
        } catch (error) {
            console.error("Error updating note", error);
        }
    };

    const handleDeleteNote = async (id: string) => {
        try {
            await noteService.deleteNote(id);
            setNotes(prev => prev.filter(n => n.id !== id));
            if (activeNoteId === id) setActiveNoteId(null);
        } catch (error) {
            console.error("Error deleting note", error);
        }
    };

    const filteredNotes = notes.filter(n =>
        n.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        n.content.toLowerCase().includes(searchQuery.toLowerCase())
    );

    const activeNote = notes.find(n => n.id === activeNoteId) || null;

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
