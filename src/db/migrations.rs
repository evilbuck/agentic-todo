use rusqlite::Connection;
use crate::error::Result;

const V1_INIT: &str = "
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT UNIQUE NOT NULL,
    path TEXT,
    name TEXT NOT NULL,
    description TEXT,
    created TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_projects_slug ON projects(slug);

CREATE TABLE IF NOT EXISTS backlogs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    context TEXT,
    created TEXT NOT NULL DEFAULT (datetime('now')),
    modified TEXT NOT NULL DEFAULT (datetime('now')),
    tags TEXT,
    priority TEXT NOT NULL DEFAULT 'medium' 
        CHECK(priority IN ('high', 'medium', 'low')),
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK(status IN ('pending', 'in_progress', 'completed', 'cancelled')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_backlogs_project ON backlogs(project_id);
CREATE INDEX IF NOT EXISTS idx_backlogs_status ON backlogs(status);
CREATE INDEX IF NOT EXISTS idx_backlogs_priority ON backlogs(priority);
CREATE INDEX IF NOT EXISTS idx_backlogs_created ON backlogs(created);
";

const V2_FTS: &str = "
-- FTS5 virtual table for full-text search
CREATE VIRTUAL TABLE IF NOT EXISTS backlogs_fts USING fts5(
    title,
    description,
    context,
    content='backlogs',
    content_rowid='id',
    tokenize='porter unicode61'
);

-- Sync triggers: Insert
CREATE TRIGGER IF NOT EXISTS backlogs_ai AFTER INSERT ON backlogs BEGIN
    INSERT INTO backlogs_fts(rowid, title, description, context)
    VALUES (new.id, new.title, new.description, new.context);
END;

-- Sync triggers: Delete
CREATE TRIGGER IF NOT EXISTS backlogs_ad AFTER DELETE ON backlogs BEGIN
    INSERT INTO backlogs_fts(backlogs_fts, rowid, title, description, context)
    VALUES ('delete', old.id, old.title, old.description, old.context);
END;

-- Sync triggers: Update
CREATE TRIGGER IF NOT EXISTS backlogs_au AFTER UPDATE ON backlogs BEGIN
    INSERT INTO backlogs_fts(backlogs_fts, rowid, title, description, context)
    VALUES ('delete', old.id, old.title, old.description, old.context);
    INSERT INTO backlogs_fts(rowid, title, description, context)
    VALUES (new.id, new.title, new.description, new.context);
END;

-- Rebuild FTS index for existing data
INSERT INTO backlogs_fts(backlogs_fts) VALUES('rebuild');
";

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(V1_INIT)?;
    conn.execute_batch(V2_FTS)?;
    Ok(())
}
