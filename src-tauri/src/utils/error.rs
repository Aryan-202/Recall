use serde::Serialize;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Internal server error")]
    InternalError,
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InvalidInput(err.to_string())
    }
}

impl From<dotenvy::Error> for AppError {
    fn from(err: dotenvy::Error) -> Self {
        AppError::ConfigError(err.to_string())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::InternalError => write!(f, "Internal server error"),
            AppError::IoError(msg) => write!(f, "IO error: {}", msg),
            AppError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;