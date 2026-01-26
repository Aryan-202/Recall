import { invoke } from "@tauri-apps/api/core";
import type { Note, Tag } from "./types";

export async function createNote(title: string, content: string): Promise<Note> {
    return await invoke("create_note", { title, content });
}

export async function getNotes(): Promise<Note[]> {
    return await invoke("get_all_notes");
}

export async function updateNote(id: string, title: string, content: string): Promise<Note> {
    return await invoke("update_note", { id, title, content });
}

export async function deleteNote(id: string): Promise<void> {
    return await invoke("delete_note", { id });
}

export async function searchNotes(query: string): Promise<Note[]> {
    return await invoke("search_notes", { query });
}

// Tags
export async function createTag(name: string): Promise<Tag> {
    return await invoke("create_tag", { name });
}

export async function getTags(): Promise<Tag[]> {
    return await invoke("get_all_tags");
}

export async function deleteTag(id: string): Promise<void> {
    return await invoke("delete_tag", { id });
}

export async function addTagToNote(noteId: string, tagId: string): Promise<void> {
    return await invoke("add_tag_to_note", { noteId, tagId });
}

export async function removeTagFromNote(noteId: string, tagId: string): Promise<void> {
    return await invoke("remove_tag_from_note", { noteId, tagId });
}

export async function getTagsForNote(noteId: string): Promise<Tag[]> {
    return await invoke("get_tags_for_note", { noteId });
}
