use schemars::{schema_for, JsonSchema};
use crate::error::Result;

#[derive(JsonSchema)]
struct CliSchema {
    version: String,
    commands: CommandsSchema,
}

#[derive(JsonSchema)]
struct CommandsSchema {
    projects: String,
    add: String,
    list: String,
    search: String,
    get: String,
    update: String,
    delete: String,
}

pub fn print_schema() -> Result<()> {
    let schema = serde_json::json!({
        "name": "agent-backlogger",
        "version": "0.1.0",
        "description": "Agent-optimized project backlog CLI",
        "commands": {
            "projects": {
                "description": "Manage projects",
                "subcommands": ["list", "add", "update", "delete"]
            },
            "add": {
                "description": "Add backlog item (auto-creates project if missing)",
                "args": ["--project", "--title", "--description", "--context", "--tags", "--priority", "--status"]
            },
            "list": {
                "description": "List items",
                "args": ["--project", "--status", "--priority", "--sort", "--limit"]
            },
            "search": {
                "description": "Search items",
                "args": ["QUERY", "--project", "--search-type", "--min-score", "--limit"]
            },
            "get": {
                "description": "Get item by ID",
                "args": ["ID"]
            },
            "update": {
                "description": "Update item",
                "args": ["ID", "--title", "--description", "--context", "--tags", "--priority", "--status"]
            },
            "delete": {
                "description": "Delete item",
                "args": ["ID"]
            }
        }
    });
    println!("{}", serde_json::to_string_pretty(&schema)?);
    Ok(())
}
