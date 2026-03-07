pub mod schema;
mod migrations;

use rusqlite::Connection;
use crate::error::Result;

pub fn init_db() -> Result<Connection> {
    let db_path = get_db_path()?;
    
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let conn = Connection::open(&db_path)?;
    
    conn.execute_batch("
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
    ")?;
    
    migrations::run_migrations(&conn)?;
    
    Ok(conn)
}

fn get_db_path() -> Result<std::path::PathBuf> {
    let data_dir = dirs::data_local_dir()
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot determine data directory"
        ))?;
    
    Ok(data_dir.join("agent-backlogger").join("backlogs.db"))
}
