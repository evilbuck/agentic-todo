use rusqlite::Row;
use crate::models::{BacklogItem, Priority, Status};

pub fn row_to_backlog_item(row: &Row) -> Result<BacklogItem, rusqlite::Error> {
    let tags_json: Option<String> = row.get(7)?;
    let tags = tags_json
        .map(|s| serde_json::from_str(&s))
        .transpose()
        .map_err(|_| rusqlite::Error::InvalidQuery)?;
    
    let priority_str: String = row.get(8)?;
    let priority = Priority::from_str(&priority_str)
        .map_err(|_| rusqlite::Error::InvalidQuery)?;
    
    let status_str: String = row.get(9)?;
    let status = Status::from_str(&status_str)
        .map_err(|_| rusqlite::Error::InvalidQuery)?;
    
    Ok(BacklogItem {
        id: row.get(0)?,
        project_id: row.get(1)?,
        project_slug: row.get(10)?,
        project_name: row.get(11)?,
        title: row.get(2)?,
        description: row.get(3)?,
        context: row.get(4)?,
        created: row.get(5)?,
        modified: row.get(6)?,
        tags,
        priority,
        status,
    })
}
