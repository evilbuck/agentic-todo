---
name: agentic-todo
category: productivity
---

# Agentic Todo (agent-backlogger)

Durable, cross-project backlog manager built in Rust by evilbuck. Uses SQLite with full-text search and vector/semantic search.

## Binary

`agent-backlogger` — installed at `~/.cargo/bin/agent-backlogger`

**Important:** Always use full path or prefix with `PATH="$HOME/.cargo/bin:$PATH"` since the PATH may not be set in all shell contexts.

## Commands

### Add an item
```bash
~/.cargo/bin/agent-backlogger add -p <project> -t <title> [--description "..."] [--context "..."] [--tags tag1,tag2] [--priority high|medium|low] [--status pending|in_progress|completed|cancelled]
```

Auto-creates the project if it doesn't exist.

### List items
```bash
~/.cargo/bin/agent-backlogger list [-p <project>] [--status <status>] [--priority <priority>] [--sort created|modified|priority|status] [-l <limit>]
```

### Search items
```bash
~/.cargo/bin/agent-backlogger search "<query>" [-p <project>] [--search-type fts|vector|hybrid] [--min-score 0.3] [-l 10]
```

### Get single item
```bash
~/.cargo/bin/agent-backlogger get <id>
```

### Update an item
```bash
~/.cargo/bin/agent-backlogger update <id> [--title "..."] [--description "..."] [--status <status>] [--priority <priority>]
```

### Delete an item
```bash
~/.cargo/bin/agent-backlogger delete <id>
```

### Manage projects
```bash
~/.cargo/bin/agent-backlogger projects
```

## When to use

- Any time a task needs to persist beyond the current session
- Cross-project tracking (different `-p` slugs per repo/client)
- When the built-in session-scoped `todo` tool would lose data on session reset
- For backlog items with context, tags, and priorities

## Search types

- `fts` — full-text search, fast keyword matching (default)
- `vector` — semantic/embedding-based search, good for conceptual queries
- `hybrid` — combines both for best results
