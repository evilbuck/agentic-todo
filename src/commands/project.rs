use rusqlite::Connection;
use crate::cli::ProjectCommands;
use crate::models::{Project, CreateProject, UpdateProject};
use crate::error::{Error, Result};

pub fn handle(cmd: ProjectCommands, db: &Connection) -> Result<()> {
    match cmd {
        ProjectCommands::List => list(db),
        ProjectCommands::Add { slug, name, path, description } => {
            add(db, CreateProject { slug, name, path, description })
        }
        ProjectCommands::Update { slug, name, path, description } => {
            update(db, &slug, UpdateProject { name, path, description })
        }
        ProjectCommands::Delete { slug } => delete(db, &slug),
    }
}

fn list(db: &Connection) -> Result<()> {
    let mut stmt = db.prepare(
        "SELECT id, slug, path, name, description, created 
         FROM projects 
         ORDER BY created DESC"
    )?;
    
    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            slug: row.get(1)?,
            path: row.get(2)?,
            name: row.get(3)?,
            description: row.get(4)?,
            created: row.get(5)?,
        })
    })?.map(|r| r.map_err(Error::from)).collect::<Result<Vec<_>>>()?;
    
    println!("{}", serde_json::to_string_pretty(&projects)?);
    Ok(())
}

fn add(db: &Connection, project: CreateProject) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    
    let result = db.execute(
        "INSERT INTO projects (slug, path, name, description, created)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![
            project.slug,
            project.path,
            project.name,
            project.description,
            now
        ]
    )?;
    
    let id = db.last_insert_rowid();
    
    let created = get_by_id(db, id)?;
    println!("{}", serde_json::to_string_pretty(&created)?);
    Ok(())
}

fn update(db: &Connection, slug: &str, updates: UpdateProject) -> Result<()> {
    let project = get_by_slug(db, slug)?;
    
    let name = updates.name.unwrap_or(project.name);
    let path = updates.path.or(project.path);
    let description = updates.description.or(project.description);
    
    db.execute(
        "UPDATE projects SET name = ?1, path = ?2, description = ?3
         WHERE slug = ?4",
        rusqlite::params![name, path, description, slug]
    )?;
    
    let updated = get_by_slug(db, slug)?;
    println!("{}", serde_json::to_string_pretty(&updated)?);
    Ok(())
}

fn delete(db: &Connection, slug: &str) -> Result<()> {
    let project = get_by_slug(db, slug)?;
    
    db.execute("DELETE FROM projects WHERE slug = ?1", [slug])?;
    
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
        "success": true,
        "deleted": project
    }))?);
    Ok(())
}

pub fn get_by_slug(db: &Connection, slug: &str) -> Result<Project> {
    db.query_row(
        "SELECT id, slug, path, name, description, created 
         FROM projects WHERE slug = ?1",
        [slug],
        |row| Ok(Project {
            id: row.get(0)?,
            slug: row.get(1)?,
            path: row.get(2)?,
            name: row.get(3)?,
            description: row.get(4)?,
            created: row.get(5)?,
        })
    ).map_err(|_| Error::ProjectNotFound(slug.to_string()))
}

fn get_by_id(db: &Connection, id: i64) -> Result<Project> {
    db.query_row(
        "SELECT id, slug, path, name, description, created 
         FROM projects WHERE id = ?1",
        [id],
        |row| Ok(Project {
            id: row.get(0)?,
            slug: row.get(1)?,
            path: row.get(2)?,
            name: row.get(3)?,
            description: row.get(4)?,
            created: row.get(5)?,
        })
    ).map_err(|_| Error::ProjectNotFound(format!("id:{}", id)))
}

pub fn resolve_or_create(db: &Connection, slug: &str) -> Result<i64> {
    if let Ok(project) = get_by_slug(db, slug) {
        return Ok(project.id);
    }
    
    let now = chrono::Utc::now().to_rfc3339();
    db.execute(
        "INSERT INTO projects (slug, name, created)
         VALUES (?1, ?2, ?3)",
        rusqlite::params![slug, slug, now]
    )?;
    
    Ok(db.last_insert_rowid())
}
