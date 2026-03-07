use rusqlite::Connection;
use crate::cli::AddArgs;
use crate::models::{BacklogItem, Priority, Status, CreateBacklogItem};
use crate::db::schema::row_to_backlog_item;
use crate::error::{Error, Result};
use crate::search::vector;
use super::project;

pub fn handle(args: AddArgs, db: &Connection) -> Result<()> {
    let item = CreateBacklogItem {
        project_slug: args.project,
        title: args.title,
        description: args.description,
        context: args.context,
        tags: args.tags,
        priority: Some(args.priority),
        status: Some(args.status),
    };
    
    add(db, item)
}

fn add(db: &Connection, item: CreateBacklogItem) -> Result<()> {
    let project_id = project::resolve_or_create(db, &item.project_slug)?;
    
    let priority = item.priority
        .map(|s| Priority::from_str(&s))
        .transpose()
        .map_err(|e| Error::InvalidEnum(e))?
        .unwrap_or_default();
    
    let status = item.status
        .map(|s| Status::from_str(&s))
        .transpose()
        .map_err(|e| Error::InvalidEnum(e))?
        .unwrap_or_default();
    
    let tags_json = item.tags
        .map(|t| serde_json::to_string(&t))
        .transpose()?;
    
    let now = chrono::Utc::now().to_rfc3339();
    
    db.execute(
        "INSERT INTO backlogs 
         (project_id, title, description, context, created, modified, tags, priority, status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            project_id,
            item.title,
            item.description,
            item.context,
            now,
            now,
            tags_json,
            priority.as_str(),
            status.as_str()
        ]
    )?;
    
    let id = db.last_insert_rowid();
    
    vector::sync_item_to_file(
        id,
        &item.title,
        item.description.as_deref(),
        item.context.as_deref()
    ).unwrap_or_else(|e| eprintln!("Warning: Failed to sync to QMD: {}", e));
    
    let created = get_by_id(db, id)?;
    
    println!("{}", serde_json::to_string_pretty(&created)?);
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
