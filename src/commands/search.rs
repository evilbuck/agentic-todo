use rusqlite::Connection;
use crate::cli::SearchArgs;
use crate::models::BacklogItem;
use crate::db::schema::row_to_backlog_item;
use crate::error::{Error, Result};
use crate::search;

pub fn handle(args: SearchArgs, db: &Connection) -> Result<()> {
    match args.search_type.as_str() {
        "fts" => search_fts(db, &args),
        "vector" => search_vector(db, &args),
        "hybrid" => search_hybrid(db, &args),
        _ => Err(Error::Cli(format!("Invalid search type: {}. Use fts, vector, or hybrid", args.search_type)))
    }
}

fn search_fts(db: &Connection, args: &SearchArgs) -> Result<()> {
    let items = search::fts::search_fts(db, &args.query, args.project.as_deref(), args.limit)?;
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

fn search_vector(db: &Connection, args: &SearchArgs) -> Result<()> {
    let ids = search::vector::search_vector(&args.query, args.min_score, args.limit)?;
    
    if ids.is_empty() {
        println!("[]");
        return Ok(());
    }
    
    let items = fetch_items_by_ids(db, &ids)?;
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

fn search_hybrid(db: &Connection, args: &SearchArgs) -> Result<()> {
    let fts_ids = search::fts::search_ids_only(db, &args.query, args.project.as_deref(), args.limit)?;
    let vector_ids = search::vector::search_vector(&args.query, args.min_score, args.limit)?;
    
    let mut combined_ids = Vec::new();
    let mut seen = std::collections::HashSet::new();
    
    for id in &fts_ids {
        if vector_ids.contains(id) {
            combined_ids.push(*id);
            seen.insert(*id);
        }
    }
    
    for id in &fts_ids {
        if !seen.contains(id) {
            combined_ids.push(*id);
            seen.insert(*id);
        }
    }
    
    for id in &vector_ids {
        if !seen.contains(id) {
            combined_ids.push(*id);
        }
    }
    
    combined_ids.truncate(args.limit);
    
    let items = fetch_items_by_ids(db, &combined_ids)?;
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

fn fetch_items_by_ids(db: &Connection, ids: &[i64]) -> Result<Vec<BacklogItem>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    
    let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
    let order_case: String = ids
        .iter()
        .enumerate()
        .map(|(i, id)| format!("WHEN {} THEN {}", id, i))
        .collect::<Vec<_>>()
        .join(" ");
    
    let sql = format!(
        "SELECT b.id, b.project_id, b.title, b.description, b.context,
                b.created, b.modified, b.tags, b.priority, b.status,
                p.slug as project_slug, p.name as project_name
         FROM backlogs b
         JOIN projects p ON b.project_id = p.id
         WHERE b.id IN ({})
         ORDER BY CASE b.id {} END",
        placeholders.join(","),
        order_case
    );
    
    let params: Vec<&dyn rusqlite::ToSql> = ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();
    
    let mut stmt = db.prepare(&sql)?;
    let items = stmt
        .query_map(params.as_slice(), |row| row_to_backlog_item(row))?
        .collect::<std::result::Result<Vec<_>, rusqlite::Error>>()
        .map_err(Error::Database)?;
    
    Ok(items)
}
