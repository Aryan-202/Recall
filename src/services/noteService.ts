import { invoke } from "@tauri-apps/api/core";
import { Note } from "../types";

export const noteService = {
    async saveNote(title: string, body: string): Promise<number[]> {
        try {
            const bytes = await invoke<number[]>("save_note", { title, body });
            return bytes;
        } catch (error) {
            console.error("Failed to save note:", error);
            throw error;
        }
    },

    async loadNotes(bytesArrayJson: string): Promise<Note[]> {
        try {
            const result = await invoke<string>("load_notes", { data: bytesArrayJson });
            if (result === "no data") return [];
            if (result.startsWith("error:")) throw new Error(result);
            return JSON.parse(result);
        } catch (error) {
            console.error("Failed to load notes:", error);
            return [];
        }
    },

    async editNote(notes: Note[]): Promise<number[]> {
        try {
            const data = JSON.stringify(notes);
            const bytes = await invoke<number[]>("edit_note", { data });
            return bytes;
        } catch (error) {
            console.error("Failed to edit note:", error);
            throw error;
        }
    }
};
