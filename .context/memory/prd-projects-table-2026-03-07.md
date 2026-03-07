---
date: 2026-03-07
domains: [planning, schema]
topics: [projects-table, normalization]
related: [prd-project-update-2026-03-07.md, prd-refine-2026-03-07.md]
priority: high
status: active
---

# Session: 2026-03-07 - PRD Projects Table

## Context
- Suggestion: Separate projects table for metadata/context.

## Decisions Made
- Normalized: projects(id,slug,path,name,desc); backlogs FK project_id.
- CLI: projects subcmd; auto-create/resolve slug on add.

## Implementation Notes
- PRD.md → v1.2.

## Next Steps
- [ ] Prototype DB schema/migrations.
