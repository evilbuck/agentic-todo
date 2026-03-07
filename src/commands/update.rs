use rusqlite::Connection;
use crate::cli::UpdateArgs;
use crate::models::{BacklogItem, Priority, Status, UpdateBacklogItem};
use crate::db::schema::row_to_backlog_item;
use crate::error::{Error, Result};
use crate::search::vector;

pub fn handle(args: UpdateArgs, db: &Connection) -> Result<()> {
    let item = UpdateBacklogItem {
        title: args.title,
        description: args.description,
        context: args.context,
        tags: args.tags,
        priority: args.priority,
        status: args.status,
    };
    
    update(db, args.id, item)
}

fn update(db: &Connection, id: i64, updates: UpdateBacklogItem) -> Result<()> {
    let existing = get_by_id(db, id)?;
    
    let title = updates.title.unwrap_or(existing.title);
    let description = updates.description.or(existing.description);
    let context = updates.context.or(existing.context);
    let tags_json = updates.tags
        .map(|t| serde_json::to_string(&t))
        .transpose()?
        .or_else(|| existing.tags.map(|t| serde_json::to_string(&t).unwrap()));
    
    let priority = updates.priority
        .map(|s| Priority::from_str(&s))
        .transpose()
        .map_err(|e| Error::InvalidEnum(e))?
        .unwrap_or(existing.priority);
    
    let status = updates.status
        .map(|s| Status::from_str(&s))
        .transpose()
        .map_err(|e| Error::InvalidEnum(e))?
        .unwrap_or(existing.status);
    
    let now = chrono::Utc::now().to_rfc3339();
    
    db.execute(
        "UPDATE backlogs 
         SET title = ?1, description = ?2, context = ?3, 
             modified = ?4, tags = ?5, priority = ?6, status = ?7
         WHERE id = ?8",
        rusqlite::params![
            title,
            description,
            context,
            now,
            tags_json,
            priority.as_str(),
            status.as_str(),
            id
        ]
    )?;
    
    vector::sync_item_to_file(
        id,
        &title,
        description.as_deref(),
        context.as_deref()
    ).unwrap_or_else(|e| eprintln!("Warning: Failed to sync to QMD: {}", e));
    
    let updated = get_by_id(db, id)?;
    println!("{}", serde_json::to_string_pretty(&updated)?);
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
