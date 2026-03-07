---
date: 2026-03-07
domains: [planning, docs]
topics: [prd-refine, backlog-tool]
related: []
priority: high
status: active
---

# Session: 2026-03-07 - PRD Refinement & Backlog Init

## Context
- Goal: Refine raw PRD.md into structured spec; init project context.
- No prior backlog/memory.

## Decisions Made
- Extended data model (id/tags/priority/status).
- Commands: add/list/search/get/update/delete.
- Tech: Rust preferred (perf); SQLite + FTS5/QMD.
- Storage: XDG ~/.local/share/agent-backlogger/.

## Implementation Notes
- Key files: PRD.md (rewritten), .context/backlog.md (created).
- Gotchas: Ensure agent-JSON (no fmt), piping.

## Next Steps
- [ ] Pick lang (Rust?); prototype CLI.
