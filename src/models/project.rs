use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub slug: String,
    pub path: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub created: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProject {
    pub slug: String,
    pub name: String,
    pub path: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub path: Option<String>,
    pub description: Option<String>,
}
