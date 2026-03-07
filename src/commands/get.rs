use rusqlite::Connection;
use crate::models::BacklogItem;
use crate::db::schema::row_to_backlog_item;
use crate::error::{Error, Result};

pub fn handle(id: i64, db: &Connection) -> Result<()> {
    let item = get_by_id(db, id)?;
    println!("{}", serde_json::to_string_pretty(&item)?);
    Ok(())
}

fn get_by_id(db: &Connection, id: i64) -> Result<BacklogItem> {
    db.query_row(
        "SELECT b.id, b.project_id, b.title, b.description, b.context, 
                b.created, b.modified, b.tags, b.priority, b.status,
                p.slug as project_slug, p.name as project_name
         FROM backlogs b
         JOIN projects p ON b.project_id = p.id
         WHERE b.id = ?1",
        [id],
        |row| row_to_backlog_item(row)
    ).map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Error::ItemNotFound(id),
        other => Error::Database(other),
    })
}
