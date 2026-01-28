import { writable, derived } from 'svelte/store';
import type { Note, Tag } from './types';
import * as db from './db';

// Primary Stores
export const notes = writable<Note[]>([]);
export const tags = writable<Tag[]>([]);
export const selectedNoteId = writable<string | null>(null);
export const searchQuery = writable<string>("");

// Derived Stores
export const selectedNote = derived(
    [notes, selectedNoteId],
    ([$notes, $selectedNoteId]) => $notes.find(n => n.id === $selectedNoteId) || null
);

// Actions
export async function loadNotes() {
    const data = await db.getNotes();
    notes.set(data);
}

export async function loadTags() {
    const data = await db.getTags();
    tags.set(data);
}

export async function addNote() {
    const newNote = await db.createNote("Untitled Note", "");
    notes.update(n => [newNote, ...n]);
    selectedNoteId.set(newNote.id);
}

export async function performSearch(query: string) {
    searchQuery.set(query);
    if (!query.trim()) {
        await loadNotes();
        return;
    }
    const results = await db.searchNotes(query);
    notes.set(results);
}

export async function saveCurrentNote(id: string, title: string, content: string) {
    const updated = await db.updateNote(id, title, content);
    notes.update(all => all.map(n => n.id === id ? updated : n));
}

export async function removeNote(id: string) {
    await db.deleteNote(id);
    notes.update(all => all.filter(n => n.id !== id));
    selectedNoteId.update(current => current === id ? null : current);
}
