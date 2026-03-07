---
id: phase2-search-vectors
type: sprint
status: active
priority: high
owner: buckleyrobinson
created: 2026-03-07
completed:
dependencies: []
---

# Phase 2: Search & Vector Integration

## Goal
Implement advanced search capabilities: FTS5 full-text search, QMD vector embeddings for semantic search, and hybrid search combining both methods.

## Context / Background

Phase 1 completed core CRUD with basic LIKE search. Now we need:
- **FTS5**: Fast keyword search across title/description/context
- **Vector Search**: Semantic similarity using QMD tool
- **Hybrid**: Combine both for best results

Current search (`src/commands/search.rs`) uses simple LIKE queries. Need to upgrade to production-ready search.

## Requirements

### Must Have
- [ ] FTS5 virtual table with auto-sync triggers
- [ ] FTS5 search command (`--type=fts`)
- [ ] QMD subprocess integration for vector search
- [ ] Vector search command (`--type=vector`)
- [ ] Text file sync for QMD indexing
- [ ] Error handling for missing QMD

### Should Have
- [ ] Hybrid search combining FTS + vector scores
- [ ] Search result ranking/relevance scoring
- [ ] Configurable min_score threshold for vectors

### Nice to Have
- [ ] FTS5 snippet highlighting
- [ ] Search result pagination
- [ ] Search history/logging

## Implementation Plan

### 2.1 FTS5 Full-Text Search (2 hours)

#### Database Migration
Create migration `v2_add_fts.sql`:

```sql
-- FTS5 virtual table
CREATE VIRTUAL TABLE IF NOT EXISTS backlogs_fts USING fts5(
    title,
    description,
    context,
    content='backlogs',
    content_rowid='id',
    tokenize='porter unicode61'
);

-- Sync triggers
CREATE TRIGGER IF NOT EXISTS backlogs_ai AFTER INSERT ON backlogs BEGIN
    INSERT INTO backlogs_fts(rowid, title, description, context)
    VALUES (new.id, new.title, new.description, new.context);
END;

CREATE TRIGGER IF NOT EXISTS backlogs_ad AFTER DELETE ON backlogs BEGIN
    INSERT INTO backlogs_fts(backlogs_fts, rowid, title, description, context)
    VALUES ('delete', old.id, old.title, old.description, old.context);
END;

CREATE TRIGGER IF NOT EXISTS backlogs_au AFTER UPDATE ON backlogs BEGIN
    INSERT INTO backlogs_fts(backlogs_fts, rowid, title, description, context)
    VALUES ('delete', old.id, old.title, old.description, old.context);
    INSERT INTO backlogs_fts(rowid, title, description, context)
    VALUES (new.id, new.title, new.description, new.context);
END;

-- Rebuild FTS index for existing data
INSERT INTO backlogs_fts(backlogs_fts) VALUES('rebuild');
```

#### Implementation Files
- `src/db/migrations.rs` - Add `run_migration_v2()` function
- `src/search/fts.rs` - Implement `search_fts(query, project_slug, limit)`
  ```rust
  pub fn search_fts(
      db: &Connection,
      query: &str,
      project_slug: Option<&str>,
      limit: usize,
  ) -> Result<Vec<BacklogItem>> {
      let sql = format!(
          "SELECT b.id, b.project_id, b.title, b.description, b.context,
                  b.created, b.modified, b.tags, b.priority, b.status,
                  p.slug as project_slug, p.name as project_name
           FROM backlogs b
           JOIN backlogs_fts fts ON b.id = fts.rowid
           JOIN projects p ON b.project_id = p.id
           WHERE backlogs_fts MATCH ?1
           ORDER BY rank
           LIMIT {}",
          limit
      );
      
      let items = db.query_map(&sql, [query], |row| {
          row_to_backlog_item(row)
      })?.collect::<std::result::Result<Vec<_>, _>>()?;
      
      Ok(items)
  }
  ```

- `src/commands/search.rs` - Update to use FTS when `--type=fts`

#### Testing
```bash
# Add test data
agent-backlogger add --project test --title "API authentication" --description "OAuth2 implementation"
agent-backlogger add --project test --title "Database optimization" --description "Add indexes for performance"

# Test FTS
agent-backlogger search "auth" --type fts
agent-backlogger search "database performance" --type fts
```

---

### 2.2 QMD Vector Integration (3 hours)

#### Architecture
1. **Item Sync**: Write each backlog item to `~/.local/share/agent-backlogger/items/{id}.txt`
2. **QMD Indexing**: Call `qmd index ~/.local/share/agent-backlogger/items/ --collection backlogs`
3. **Vector Search**: Call `qmd_vector_search(query, collection="backlogs", limit, min_score)`
4. **Result Mapping**: QMD returns docids, map back to backlog item IDs

#### Implementation Files

**`src/search/vector.rs`**:
```rust
use std::process::Command;
use serde::Deserialize;
use crate::error::{Error, Result};

#[derive(Deserialize)]
struct QmdSearchResult {
    file: String,
    score: f32,
}

pub fn search_vector(
    query: &str,
    min_score: f32,
    limit: usize,
) -> Result<Vec<i64>> {
    let output = Command::new("qmd")
        .arg("vector-search")
        .arg(query)
        .arg("--collection")
        .arg("backlogs")
        .arg("--min-score")
        .arg(min_score.to_string())
        .arg("--limit")
        .arg(limit.to_string())
        .arg("--format")
        .arg("json")
        .output()
        .map_err(|e| Error::Qmd(format!("Failed to execute qmd: {}", e)))?;
    
    if !output.status.success() {
        return Err(Error::Qmd(String::from_utf8_lossy(&output.stderr).to_string()));
    }
    
    let results: Vec<QmdSearchResult> = serde_json::from_slice(&output.stdout)?;
    
    // Extract item IDs from file paths: items/{id}.txt
    let ids: Vec<i64> = results
        .iter()
        .filter_map(|r| {
            r.file
                .trim_end_matches(".txt")
                .split('/')
                .last()
                .and_then(|s| s.parse().ok())
        })
        .collect();
    
    Ok(ids)
}

pub fn sync_item_to_file(id: i64, title: &str, description: Option<&str>, context: Option<&str>) -> Result<()> {
    let items_dir = dirs::data_local_dir()
        .ok_or_else(|| Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Data dir not found")))?
        .join("agent-backlogger")
        .join("items");
    
    std::fs::create_dir_all(&items_dir)?;
    
    let file_path = items_dir.join(format!("{}.txt", id));
    let mut content = title.to_string();
    
    if let Some(desc) = description {
        content.push_str("\n\n");
        content.push_str(desc);
    }
    
    if let Some(ctx) = context {
        content.push_str("\n\n");
        content.push_str(ctx);
    }
    
    std::fs::write(&file_path, content)?;
    Ok(())
}

pub fn delete_item_file(id: i64) -> Result<()> {
    let items_dir = dirs::data_local_dir()
        .ok_or_else(|| Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Data dir not found")))?
        .join("agent-backlogger")
        .join("items");
    
    let file_path = items_dir.join(format!("{}.txt", id));
    
    if file_path.exists() {
        std::fs::remove_file(&file_path)?;
    }
    
    Ok(())
}

pub fn rebuild_qmd_index() -> Result<()> {
    let items_dir = dirs::data_local_dir()
        .ok_or_else(|| Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Data dir not found")))?
        .join("agent-backlogger")
        .join("items");
    
    let status = Command::new("qmd")
        .arg("index")
        .arg(&items_dir)
        .arg("--collection")
        .arg("backlogs")
        .status()
        .map_err(|e| Error::Qmd(format!("Failed to execute qmd index: {}", e)))?;
    
    if !status.success() {
        return Err(Error::Qmd("qmd index failed".to_string()));
    }
    
    Ok(())
}
```

**Update `src/commands/add.rs`**:
```rust
// After successful INSERT, sync to file
search::vector::sync_item_to_file(id, &item.title, item.description.as_deref(), item.context.as_deref())?;
```

**Update `src/commands/update.rs`**:
```rust
// After successful UPDATE, sync to file
search::vector::sync_item_to_file(id, &title, description.as_deref(), context.as_deref())?;
```

**Update `src/commands/delete.rs`**:
```rust
// After successful DELETE, remove file
search::vector::delete_item_file(id)?;
```

**Update `src/commands/search.rs`**:
```rust
fn search_vector(db: &Connection, args: &SearchArgs) -> Result<()> {
    let ids = search::vector::search_vector(&args.query, args.min_score, args.limit)?;
    
    if ids.is_empty() {
        println!("[]");
        return Ok(());
    }
    
    // Fetch items by IDs
    let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
    let sql = format!(
        "SELECT b.id, b.project_id, b.title, b.description, b.context,
                b.created, b.modified, b.tags, b.priority, b.status,
                p.slug as project_slug, p.name as project_name
         FROM backlogs b
         JOIN projects p ON b.project_id = p.id
         WHERE b.id IN ({})
         ORDER BY CASE b.id {} END",
        placeholders.join(","),
        ids.iter().enumerate().map(|(i, id)| format!("WHEN {} THEN {}", id, i)).collect::<Vec<_>>().join(" ")
    );
    
    let params: Vec<&dyn rusqlite::ToSql> = ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();
    
    let mut stmt = db.prepare(&sql)?;
    let items = stmt.query_map(params.as_slice(), |row| row_to_backlog_item(row))?
        .collect::<std::result::Result<Vec<_>, _>>()?;
    
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}
```

#### Testing
```bash
# Ensure QMD is available
qmd --version

# Add items
agent-backlogger add --project test --title "User authentication" --description "Login with email and password"
agent-backlogger add --project test --title "API rate limiting" --description "Prevent abuse with throttling"

# Rebuild index
# (should auto-index on add, but manual rebuild if needed)
qmd index ~/.local/share/agent-backlogger/items/ --collection backlogs

# Test vector search
agent-backlogger search "security login access control" --type vector --min-score 0.5
agent-backlogger search "prevent spam too many requests" --type vector
```

---

### 2.3 Enhanced Search Command (1 hour)

#### Update `src/commands/search.rs`

```rust
pub fn handle(args: SearchArgs, db: &Connection) -> Result<()> {
    match args.search_type.as_str() {
        "fts" => search_fts(db, &args),
        "vector" => search_vector(db, &args),
        "hybrid" => search_hybrid(db, &args),
        _ => Err(Error::Cli(format!("Invalid search type: {}. Use fts, vector, or hybrid", args.search_type)))
    }
}

fn search_hybrid(db: &Connection, args: &SearchArgs) -> Result<()> {
    // Get FTS results
    let fts_ids = search::fts::search_ids_only(db, &args.query, args.project.as_deref(), args.limit)?;
    
    // Get vector results
    let vector_ids = search::vector::search_vector(&args.query, args.min_score, args.limit)?;
    
    // Merge with preference to items in both sets
    let mut combined_ids = Vec::new();
    let mut seen = std::collections::HashSet::new();
    
    // Items in both get highest priority
    for id in &fts_ids {
        if vector_ids.contains(id) {
            combined_ids.push(*id);
            seen.insert(*id);
        }
    }
    
    // Then FTS-only results
    for id in &fts_ids {
        if !seen.contains(id) {
            combined_ids.push(*id);
            seen.insert(*id);
        }
    }
    
    // Then vector-only results
    for id in &vector_ids {
        if !seen.contains(id) {
            combined_ids.push(*id);
        }
    }
    
    combined_ids.truncate(args.limit);
    
    // Fetch and return items
    fetch_items_by_ids(db, &combined_ids)
}
```

---

## Acceptance Criteria

### FTS5
- [ ] `agent-backlogger search "query" --type fts` returns FTS-ranked results
- [ ] Search covers title, description, and context fields
- [ ] FTS index auto-updates on add/update/delete
- [ ] Existing items indexed after migration

### Vector Search
- [ ] `agent-backlogger search "query" --type vector` returns semantic matches
- [ ] Item files synced to `~/.local/share/agent-backlogger/items/`
- [ ] File deleted when item deleted
- [ ] Clear error message if QMD not available
- [ ] `--min-score` filters low-relevance results

### Hybrid Search
- [ ] `agent-backlogger search "query" --type hybrid` combines FTS + vector
- [ ] Results in both lists ranked highest
- [ ] Returns up to `--limit` results

### General
- [ ] All search types return consistent JSON structure
- [ ] Search respects `--project` filter
- [ ] Performance: <200ms for search on 10k items
- [ ] Documentation updated in README

## Risks & Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| QMD not installed | Medium | High | Graceful error, fallback to FTS-only |
| Vector search slow | Low | Medium | Cache QMD results, batch indexing |
| FTS index corruption | Low | High | Add rebuild command, log errors |
| File sync failures | Medium | Low | Retry logic, log warnings |

## Dependencies

- QMD tool must be installed and in PATH
- `~/.local/share/agent-backlogger/items/` directory writable
- rusqlite compiled with FTS5 support (bundled mode)

## Estimated Time

- **FTS5**: 2 hours
- **Vector Integration**: 3 hours
- **Hybrid Search**: 1 hour
- **Testing & Docs**: 1 hour
- **Total**: 7 hours

## Next Steps After Completion

1. Move this spec to `.context/specs/archive/`
2. Update backlog with Phase 3 tasks
3. Create Phase 3 spec (tests, performance, distribution)
