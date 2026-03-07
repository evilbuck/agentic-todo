use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
    
    #[error("Item not found: {0}")]
    ItemNotFound(i64),
    
    #[error("Invalid enum value: {0}")]
    InvalidEnum(String),
    
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("QMD error: {0}")]
    Qmd(String),
    
    #[error("CLI error: {0}")]
    Cli(String),
}

impl Error {
    pub fn error_code(&self) -> &'static str {
        match self {
            Error::Database(_) => "DATABASE_ERROR",
            Error::ProjectNotFound(_) => "PROJECT_NOT_FOUND",
            Error::ItemNotFound(_) => "ITEM_NOT_FOUND",
            Error::InvalidEnum(_) => "INVALID_ENUM",
            Error::Json(_) => "JSON_ERROR",
            Error::Io(_) => "IO_ERROR",
            Error::Qmd(_) => "QMD_ERROR",
            Error::Cli(_) => "CLI_ERROR",
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
