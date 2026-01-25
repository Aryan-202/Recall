use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id : String,
    pub name : String,
    pub email: String,
    pub password: String,
    pub avtar: String,
    pub notes: Vec<Note>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct Note {
    pub id : String,
    pub title : String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}