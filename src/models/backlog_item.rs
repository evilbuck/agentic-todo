use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Err(format!("Invalid priority: {}", s)),
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Pending => "pending",
            Status::InProgress => "in_progress",
            Status::Completed => "completed",
            Status::Cancelled => "cancelled",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pending" => Ok(Status::Pending),
            "in_progress" => Ok(Status::InProgress),
            "completed" => Ok(Status::Completed),
            "cancelled" => Ok(Status::Cancelled),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Pending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BacklogItem {
    pub id: i64,
    pub project_id: i64,
    pub project_slug: String,
    pub project_name: String,
    pub title: String,
    pub description: Option<String>,
    pub context: Option<String>,
    pub created: String,
    pub modified: String,
    pub tags: Option<Vec<String>>,
    pub priority: Priority,
    pub status: Status,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateBacklogItem {
    pub project_slug: String,
    pub title: String,
    pub description: Option<String>,
    pub context: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema, Default)]
pub struct UpdateBacklogItem {
    pub title: Option<String>,
    pub description: Option<String>,
    pub context: Option<String>,
    pub tags: Option<Vec<String>>,
    pub priority: Option<String>,
    pub status: Option<String>,
}
