use clap::{Parser, Subcommand, Args};
use schemars::JsonSchema;

#[derive(Parser)]
#[command(name = "agent-backlogger")]
#[command(about = "Agent-optimized project backlog CLI", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, JsonSchema)]
pub enum Commands {
    /// Manage projects
    Projects {
        #[command(subcommand)]
        cmd: ProjectCommands,
    },
    /// Add backlog item (auto-creates project if missing)
    Add(AddArgs),
    /// List items
    List(ListArgs),
    /// Search items
    Search(SearchArgs),
    /// Get item by ID
    Get {
        /// Item ID
        id: i64,
    },
    /// Update item
    Update(UpdateArgs),
    /// Delete item
    Delete {
        /// Item ID
        id: i64,
    },
}

#[derive(Subcommand, JsonSchema)]
pub enum ProjectCommands {
    /// List all projects
    List,
    /// Add a new project
    Add {
        /// Project slug (unique identifier)
        slug: String,
        /// Project name
        #[arg(short, long)]
        name: String,
        /// Project path (optional)
        #[arg(short, long)]
        path: Option<String>,
        /// Project description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Update a project
    Update {
        /// Project slug
        slug: String,
        /// Project name
        #[arg(short, long)]
        name: Option<String>,
        /// Project path
        #[arg(short, long)]
        path: Option<String>,
        /// Project description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Delete a project (cascade deletes all items)
    Delete {
        /// Project slug
        slug: String,
    },
}

#[derive(Args, JsonSchema)]
pub struct AddArgs {
    /// Project slug
    #[arg(short, long)]
    pub project: String,
    /// Item title
    #[arg(short, long)]
    pub title: String,
    /// Item description
    #[arg(long)]
    pub description: Option<String>,
    /// Item context/research
    #[arg(long)]
    pub context: Option<String>,
    /// Tags (comma-separated or multiple flags)
    #[arg(long)]
    pub tags: Option<Vec<String>>,
    /// Priority (high/medium/low)
    #[arg(long, default_value = "medium")]
    pub priority: String,
    /// Status (pending/in_progress/completed/cancelled)
    #[arg(long, default_value = "pending")]
    pub status: String,
}

#[derive(Args, JsonSchema)]
pub struct ListArgs {
    /// Filter by project slug
    #[arg(short, long)]
    pub project: Option<String>,
    /// Filter by status
    #[arg(long)]
    pub status: Option<String>,
    /// Filter by priority
    #[arg(long)]
    pub priority: Option<String>,
    /// Sort by field (created/modified/priority/status)
    #[arg(long, default_value = "created")]
    pub sort: String,
    /// Limit number of results
    #[arg(short, long)]
    pub limit: Option<usize>,
}

#[derive(Args, JsonSchema)]
pub struct SearchArgs {
    /// Search query
    pub query: String,
    /// Filter by project slug
    #[arg(short, long)]
    pub project: Option<String>,
    /// Search type (fts/vector/hybrid)
    #[arg(long, default_value = "fts")]
    pub search_type: String,
    /// Minimum score (for vector search)
    #[arg(long, default_value = "0.3")]
    pub min_score: f32,
    /// Limit number of results
    #[arg(short, long, default_value = "10")]
    pub limit: usize,
}

#[derive(Args, JsonSchema)]
pub struct UpdateArgs {
    /// Item ID
    pub id: i64,
    /// Item title
    #[arg(short, long)]
    pub title: Option<String>,
    /// Item description
    #[arg(long)]
    pub description: Option<String>,
    /// Item context/research
    #[arg(long)]
    pub context: Option<String>,
    /// Tags
    #[arg(long)]
    pub tags: Option<Vec<String>>,
    /// Priority
    #[arg(long)]
    pub priority: Option<String>,
    /// Status
    #[arg(long)]
    pub status: Option<String>,
}
