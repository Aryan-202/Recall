use crate::db::{
    models::{CreateNoteRequest, Note, UpdateNoteRequest},
    supabase_clients::SUPABASE_CLIENT,
};
use tauri::command;

#[command]
pub async fn get_notes(user_id: String) -> Result<Vec<Note>, String> {
    let client = SUPABASE_CLIENT
        .get()
        .ok_or("Supabase client not initialized")?;

    let res = client.get_notes(&user_id).await;
    match res {
        Ok(notes) => Ok(notes),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn create_note(user_id: String, req: CreateNoteRequest) -> Result<Note, String> {
    let client = SUPABASE_CLIENT
        .get()
        .ok_or("Supabase client not initialized")?;

    let res = client.create_note(&user_id, &req.title, &req.content).await;
    match res {
        Ok(note) => Ok(note),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn update_note(note_id: String, req: UpdateNoteRequest) -> Result<(), String> {
    let client = SUPABASE_CLIENT
        .get()
        .ok_or("Supabase client not initialized")?;

    let res = client.update_note(&note_id, req.title, req.content).await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn delete_note(note_id: String) -> Result<(), String> {
    let client = SUPABASE_CLIENT
        .get()
        .ok_or("Supabase client not initialized")?;

    let res = client.delete_note(&note_id).await;
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
