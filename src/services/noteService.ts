import { invoke } from "@tauri-apps/api/core";
import { Note } from "../types";

export const noteService = {
    async getNotes(user_id: string): Promise<Note[]> {
        try {
            const notes = await invoke<Note[]>("get_notes", { userId: user_id });
            return notes;
        } catch (error) {
            console.error("Failed to fetch notes:", error);
            return [];
        }
    },

    async createNote(user_id: string, title: string, content: string): Promise<Note> {
        try {
            const note = await invoke<Note>("create_note", {
                userId: user_id,
                req: { title, content }
            });
            return note;
        } catch (error) {
            console.error("Failed to create note:", error);
            throw error;
        }
    },

    async updateNote(note_id: string, title?: string, content?: string): Promise<void> {
        try {
            await invoke("update_note", {
                noteId: note_id,
                req: { title, content }
            });
        } catch (error) {
            console.error("Failed to update note:", error);
            throw error;
        }
    },

    async deleteNote(note_id: string): Promise<void> {
        try {
            await invoke("delete_note", { noteId: note_id });
        } catch (error) {
            console.error("Failed to delete note:", error);
            throw error;
        }
    }
};
