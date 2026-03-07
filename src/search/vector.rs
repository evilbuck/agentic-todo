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
        .arg("vsearch")
        .arg(query)
        .arg("--collection")
        .arg("backlogs")
        .arg("--min-score")
        .arg(min_score.to_string())
        .arg("-n")
        .arg(limit.to_string())
        .arg("--json")
        .output()
        .map_err(|e| Error::Qmd(format!("Failed to execute qmd: {}", e)))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Qmd(format!("qmd search failed: {}", stderr)));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let results: Vec<QmdSearchResult> = serde_json::from_str(&stdout)?;
    
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
        .ok_or_else(|| Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Data dir not found"
        )))?
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
        .ok_or_else(|| Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Data dir not found"
        )))?
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
        .ok_or_else(|| Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Data dir not found"
        )))?
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
