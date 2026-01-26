use crate::utils::error_handling::AppError;
use dirs::data_dir;
use rusqlite::{Connection, OptionalExtension};
use std::path::PathBuf;
use log::{info, error};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self, AppError> {
        let db_path = Self::get_db_path()?;
        info!("Connecting to database at: {:?}", db_path);
        
        let connection = Connection::open(&db_path)?;
        
        Ok(Self { connection })
    }
    
    fn get_db_path() -> Result<PathBuf, AppError> {
        let mut path = data_dir().ok_or_else(|| AppError::Database("Could not find data directory".into()))?;
        path.push("notes-app");
        std::fs::create_dir_all(&path)?;
        path.push("notes.db");
        Ok(path)
    }
    
    pub fn initialize(&self) -> Result<(), AppError> {
        self.connection.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;"
        )?;
        
        self.create_tables()?;
        self.create_indexes()?;
        
        info!("Database initialized successfully");
        Ok(())
    }
    
    fn create_tables(&self) -> Result<(), AppError> {
        use super::schema::*;
        
        self.connection.execute(CREATE_NOTES_TABLE, [])?;
        self.connection.execute(CREATE_TAGS_TABLE, [])?;
        self.connection.execute(CREATE_NOTE_TAGS_TABLE, [])?;
        
        Ok(())
    }
    
    fn create_indexes(&self) -> Result<(), AppError> {
        use super::schema::*;
        
        for index_sql in CREATE_INDEXES.iter() {
            self.connection.execute(index_sql, [])?;
        }
        
        Ok(())
    }
    
    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
    
    pub fn transaction<T, F>(&self, f: F) -> Result<T, AppError>
    where
        F: FnOnce(&Connection) -> Result<T, AppError>,
    {
        let tx = self.connection.transaction()?;
        let result = f(&tx)?;
        tx.commit()?;
        Ok(result)
    }
}