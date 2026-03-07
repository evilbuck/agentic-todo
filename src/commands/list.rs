use rusqlite::Connection;
use crate::cli::ListArgs;
use crate::models::{BacklogItem, Priority, Status};
use crate::db::schema::row_to_backlog_item;
use crate::error::{Error, Result};
use super::project;

pub fn handle(args: ListArgs, db: &Connection) -> Result<()> {
    let mut query = String::from(
        "SELECT b.id, b.project_id, b.title, b.description, b.context, 
                b.created, b.modified, b.tags, b.priority, b.status,
                p.slug as project_slug, p.name as project_name
         FROM backlogs b
         JOIN projects p ON b.project_id = p.id
         WHERE 1=1"
    );
    
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
    
    if let Some(slug) = &args.project {
        query.push_str(" AND p.slug = ?");
        params.push(Box::new(slug.clone()));
    }
    
    if let Some(status) = &args.status {
        let status = Status::from_str(status)
            .map_err(|e| Error::InvalidEnum(e))?;
        query.push_str(" AND b.status = ?");
        params.push(Box::new(status.as_str().to_string()));
    }
    
    if let Some(priority) = &args.priority {
        let priority = Priority::from_str(priority)
            .map_err(|e| Error::InvalidEnum(e))?;
        query.push_str(" AND b.priority = ?");
        params.push(Box::new(priority.as_str().to_string()));
    }
    
    let order = match args.sort.as_str() {
        "created" => "b.created DESC",
        "modified" => "b.modified DESC",
        "priority" => "CASE b.priority WHEN 'high' THEN 1 WHEN 'medium' THEN 2 ELSE 3 END",
        "status" => "b.status",
        _ => "b.created DESC"
    };
    query.push_str(&format!(" ORDER BY {}", order));
    
    if let Some(limit) = args.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }
    
    let mut stmt = db.prepare(&query)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    
    let items = stmt.query_map(params_refs.as_slice(), |row| {
        row_to_backlog_item(row)
    })?.collect::<std::result::Result<Vec<_>, rusqlite::Error>>().map_err(Error::Database)?;
    
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}
