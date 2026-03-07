# Backlog

## Details

### implement-cli-prototype
**Description**: Build MVP agent-backlogger CLI (add/list/search) in Rust/Python per PRD.md.
**Context**: 
- Relevant files: PRD.md, src/main.rs, src/cli.rs, src/commands/
- Requirements: SQLite DB, JSON I/O, piping, basic search/sort.
- Technical notes: Use clap for CLI, rusqlite/sqlite3; init DB schema.
- Related work: PRD.md, .context/memory/phase1-implementation-2026-03-07.md
- Status: ✅ COMPLETE (2026-03-07) - All CRUD operations working, tested successfully

### full-crud-and-vector-search
**Description**: Add FTS5/QMD vector search, enhance search command.
**Context**:
- Relevant files: PRD.md, src/commands/search.rs, src/search/
- Requirements: FTS5 full-text search, QMD vector embeddings, hybrid search.
- Technical notes: Create FTS5 virtual table with triggers, QMD subprocess for vector indexing.
- Dependencies: Phase 1 complete
- Status: ✅ COMPLETE (2026-03-07) - FTS5, vector, and hybrid search all implemented and tested

### tests-and-dist
**Description**: Integration tests, performance testing, cargo install/release builds.
**Context**:
- Relevant files: tests/, benches/, Cargo.toml
- Requirements: Test CRUD operations, FK integrity, performance on 10k items.
- Technical notes: Use tempfile for test DB isolation, criterion for benchmarks.

## High Priority
- [ ] Add integration tests [#tests-and-dist](#tests-and-dist)
- [ ] Performance testing with 10k items [#tests-and-dist](#tests-and-dist)

## Medium Priority
- [ ] Add integration tests [#tests-and-dist](#tests-and-dist)
- [ ] Performance testing with 10k items [#tests-and-dist](#tests-and-dist)

## Low Priority / Nice to Have
- [ ] Build release binary and test cargo install [#tests-and-dist](#tests-and-dist)
- [ ] Add hybrid search (combine FTS + vector) [#full-crud-and-vector-search](#full-crud-and-vector-search)

## Completed
- [x] Implement CLI prototype with core CRUD [#implement-cli-prototype](#implement-cli-prototype) (2026-03-07)
- [x] Implement FTS5 full-text search (See: specs/active/phase2-search-vectors.md) (2026-03-07)
- [x] Integrate QMD vector search (See: specs/active/phase2-search-vectors.md) (2026-03-07)
- [x] Add hybrid search combining FTS + vector (See: specs/active/phase2-search-vectors.md) (2026-03-07)

