# Agent Backlogger

Agent-optimized CLI for managing project-specific backlogs with JSON I/O, full-text search, and semantic vector search.

## Features

- **Project-based organization**: Each project has its own backlog with metadata
- **JSON I/O**: All commands output JSON for easy agent consumption
- **Full CRUD**: Create, read, update, delete backlog items
- **Search**: FTS5 full-text search, QMD vector search, and hybrid search
- **Auto-project creation**: Projects auto-created when adding first item
- **Detailed context**: Store research, decisions, and gotchas with each item

## Installation

### Prerequisites

For vector search functionality, install [QMD](https://github.com/tobil4/qmd):
```bash
npm install -g @tobilu/qmd
```

### From Source

```bash
git clone <repository>
cd agent_backlogger
cargo install --path .
```

### Binary Release

Download from releases (coming soon).

## Quick Start

```bash
# Create a project
agent-backlogger projects add my-project --name "My Project" --description "Project description"

# Add a backlog item (auto-creates project if needed)
agent-backlogger add --project my-project --title "Fix API bug" --priority high

# List items
agent-backlogger list --project my-project

# Search items
agent-backlogger search "bug" --search-type fts
agent-backlogger search "security authentication" --search-type vector --min-score 0.5
agent-backlogger search "api issue" --search-type hybrid

# Update an item
agent-backlogger update 1 --status in_progress

# Get item by ID
agent-backlogger get 1

# Delete an item
agent-backlogger delete 1
```

## Commands

### Projects

```bash
# List all projects
agent-backlogger projects list

# Add a project
agent-backlogger projects add <SLUG> --name <NAME> [--path <PATH>] [--description <DESC>]

# Update a project
agent-backlogger projects update <SLUG> [--name <NAME>] [--path <PATH>] [--description <DESC>]

# Delete a project (cascades to all items)
agent-backlogger projects delete <SLUG>
```

### Backlog Items

```bash
# Add item (auto-creates project if missing)
agent-backlogger add --project <SLUG> --title <TITLE> \
  [--description <DESC>] [--context <CONTEXT>] \
  [--tags <TAG1> <TAG2>] [--priority high|medium|low] \
  [--status pending|in_progress|completed|cancelled]

# List items
agent-backlogger list [--project <SLUG>] [--status <STATUS>] \
  [--priority <PRIORITY>] [--sort created|modified|priority|status] \
  [--limit <N>]

# Search items
agent-backlogger search <QUERY> [--project <SLUG>] \
  [--search-type fts|vector|hybrid] [--min-score <SCORE>] \
  [--limit <N>]

# Get item by ID
agent-backlogger get <ID>

# Update item
agent-backlogger update <ID> [--title <TITLE>] [--description <DESC>] \
  [--context <CONTEXT>] [--tags <TAGS>] [--priority <PRIORITY>] \
  [--status <STATUS>]

# Delete item
agent-backlogger delete <ID>
```

## Search Types

The `search` command supports three search types via `--search-type`:

### FTS5 (`--search-type fts`)
Full-text search using SQLite FTS5 with porter stemmer.
- Fast keyword matching
- Supports boolean operators: `AND`, `OR`, `NOT`
- Relevance ranking
- Example: `agent-backlogger search "api OR database" --search-type fts`

### Vector (`--search-type vector`)
Semantic similarity search using QMD embeddings.
- Understands meaning, not just keywords
- Finds conceptually similar items
- Use `--min-score` to filter low-relevance results (default: 0.3)
- Example: `agent-backlogger search "security login authentication" --search-type vector --min-score 0.5`

### Hybrid (`--search-type hybrid`)
Combines FTS5 and vector search for best results.
- Items appearing in both searches ranked highest
- FTS-only results ranked second
- Vector-only results ranked third
- Example: `agent-backlogger search "api security" --search-type hybrid`

## JSON Schema

Run with no arguments to get the JSON schema:

```bash
agent-backlogger
```

All commands output JSON for easy parsing by agents and scripts.

## Data Model

### Projects Table
- `id`: Auto-increment integer (primary key)
- `slug`: Unique string identifier
- `path`: Optional repository path
- `name`: Full project name
- `description`: Project overview
- `created`: ISO datetime

### Backlogs Table
- `id`: Auto-increment integer (primary key)
- `project_id`: Foreign key to projects
- `title`: Item title
- `description`: Detailed specification
- `context`: Research, decisions, gotchas
- `created`: ISO datetime
- `modified`: ISO datetime (auto-updated)
- `tags`: JSON array of strings
- `priority`: `high`, `medium`, or `low`
- `status`: `pending`, `in_progress`, `completed`, or `cancelled`

## Storage

- **Database**: `~/.local/share/agent-backlogger/backlogs.db`
- **Search index**: `~/.local/share/agent-backlogger/items/` (for QMD vector search)

The database is automatically created and migrated on first run.

## Development

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Run

```bash
cargo run -- [commands]
```

## Roadmap

### Phase 1 ✅ (Complete)
- [x] Core CRUD operations
- [x] JSON I/O
- [x] Basic search
- [x] Project management

### Phase 2 ✅ (Complete)
- [x] FTS5 full-text search with porter stemmer
- [x] QMD vector search integration
- [x] Hybrid search (FTS + vector combined)
- [x] Auto-sync to QMD index on add/update/delete

### Phase 3 (Planned)
- [ ] Integration tests
- [ ] Performance optimization
- [ ] Release builds and distribution

## License

MIT
