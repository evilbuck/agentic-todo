use rusqlite::Connection;
use crate::models::BacklogItem;
use crate::db::schema::row_to_backlog_item;
use crate::error::{Error, Result};

pub fn search_fts(
    db: &Connection,
    query: &str,
    project_slug: Option<&str>,
    limit: usize,
) -> Result<Vec<BacklogItem>> {
    let mut sql = String::from(
        "SELECT b.id, b.project_id, b.title, b.description, b.context,
                b.created, b.modified, b.tags, b.priority, b.status,
                p.slug as project_slug, p.name as project_name
         FROM backlogs b
         JOIN backlogs_fts fts ON b.id = fts.rowid
         JOIN projects p ON b.project_id = p.id
         WHERE backlogs_fts MATCH ?1"
    );
    
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(query.to_string())];
    
    if let Some(slug) = project_slug {
        sql.push_str(" AND p.slug = ?");
        params.push(Box::new(slug.to_string()));
    }
    
    sql.push_str(" ORDER BY rank");
    sql.push_str(&format!(" LIMIT {}", limit));
    
    let mut stmt = db.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    
    let items = stmt
        .query_map(params_refs.as_slice(), |row| row_to_backlog_item(row))?
        .collect::<std::result::Result<Vec<_>, rusqlite::Error>>()
        .map_err(Error::Database)?;
    
    Ok(items)
}

pub fn search_ids_only(
    db: &Connection,
    query: &str,
    project_slug: Option<&str>,
    limit: usize,
) -> Result<Vec<i64>> {
    let mut sql = String::from(
        "SELECT b.id
         FROM backlogs b
         JOIN backlogs_fts fts ON b.id = fts.rowid
         JOIN projects p ON b.project_id = p.id
         WHERE backlogs_fts MATCH ?1"
    );
    
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(query.to_string())];
    
    if let Some(slug) = project_slug {
        sql.push_str(" AND p.slug = ?");
        params.push(Box::new(slug.to_string()));
    }
    
    sql.push_str(" ORDER BY rank");
    sql.push_str(&format!(" LIMIT {}", limit));
    
    let mut stmt = db.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    
    let ids: Vec<i64> = stmt
        .query_map(params_refs.as_slice(), |row| row.get(0))?
        .collect::<std::result::Result<Vec<_>, rusqlite::Error>>()
        .map_err(Error::Database)?;
    
    Ok(ids)
}
