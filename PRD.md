# Agent Backlogger CLI - PRD (v1.2)

## Overview
Agent-optimized CLI for **project-specific** backlogs. Normalized schema: projects table for metadata; items FK to project. Detailed context per item/project for zero-context resumption. JSON/JSONL I/O. Help JSON schema default.

## Data Model

### Projects Table
- `id`: auto-inc integer (PK)
- `slug`: string (unique, CLI key e.g. 'agent_backlogger')
- `path`: string (optional, repo dir)
- `name`: string (full name)
- `description`: text (project overview/research)
- `created`: ISO datetime (auto)

### Backlogs Table
- `id`: auto-inc integer (PK)
- `project_id`: integer (FK REFERENCES projects.id, req)
- `title`: string (req)
- `description`: text (detailed spec)
- `context`: text (research/decisions/gotchas)
- `created`: ISO datetime (auto)
- `modified`: ISO datetime (auto)
- `tags`: JSON array (opt)
- `priority`: enum ['high','medium','low'] (default 'medium')
- `status`: enum ['pending','in_progress','completed','cancelled'] (default 'pending')

## CLI Interface
```
agent-backlogger [command] [flags]
```
- **No args**: JSON schema.
- **projects list/add &lt;SLUG&gt; --name=... --path=... --description=...** / **update &lt;SLUG&gt;** / **delete &lt;SLUG&gt;**: Manage projects (auto-resolve slug→id).
- **add** [--project=SLUG --tags=... --priority=...]: Auto-create project if missing.
- **list** [--project=SLUG --sort=... --limit=... --status=...]: By project_id.
- **search &lt;QUERY&gt;** [--project=SLUG --type=...]: Scoped.
- **get/update/delete &lt;ID&gt;**: Item ops.

Flags: --output=..., --format=..., --dry-run.

## Input/Output
- Stdin JSON: Use 'project_slug' for add (resolves to id).
- Output: Items incl. project_slug/name.

## Storage
- DB: `~/.local/share/agent-backlogger/backlogs.db`.
- Indexes: projects(slug); backlogs(project_id, status, priority, created).
- FTS5: title+desc+context per project.
- Vector: Embed on desc+context.
- Auto-migrate/create.

## Tech Stack
Rust/Python; rusqlite; clap; serde. FK enforcement.

## Non-Functional
- 10k items/project; project isolation.
- Large TEXT fields.
- JSON errors.
- Tests: FK integrity, resolve slug.

Version: 1.2 (2026-03-07) - Normalized projects table, slug resolve.