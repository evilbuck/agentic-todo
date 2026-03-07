# Backlog

## Details

### implement-cli-prototype
**Description**: Build MVP agent-backlogger CLI (add/list/search) in Rust/Python per PRD.md.
**Context**: 
- Relevant files: PRD.md
- Requirements: SQLite DB, JSON I/O, piping, basic search/sort.
- Technical notes: Use clap for CLI, rusqlite/sqlite3; init DB schema.
- Related work: PRD.md

### full-crud-and-vector-search
**Description**: Add update/delete/get, FTS5/QMD vector search.
**Context**:
- Relevant files: PRD.md
- Requirements: Full CRUD, vector embeddings on desc.
- Technical notes: rusqlite vector ext or QMD subprocess.

### tests-and-dist
**Description**: Unit/integration tests, install script/pipx/cargo.
**Context**:
- Relevant files: PRD.md
- Requirements: Test perf on 10k items.

## High Priority
- [ ] Implement CLI prototype [#implement-cli-prototype](#implement-cli-prototype)

## Medium Priority
- [ ] Full CRUD & vector search [#full-crud-and-vector-search](#full-crud-and-vector-search)

## Low Priority / Nice to Have
- [ ] Tests & distribution [#tests-and-dist](#tests-and-dist)

## Completed
