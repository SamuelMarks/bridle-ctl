# Agent Guidelines

## Purpose
`AGENT_GUIDELINES.md` outlines the operational rules for the AI context interacting with `bridle-ctl`.

## Mandates
- **Always use FFI/Tools**: Do not attempt to string-replace manually. Use `bridle fix`.
- **Database Context**: Connect to `bridle.db` (SQLite) or `postgres://` URLs to track tasks.
- **PR Rate Limiting**: You can generate as many PRs as desired in the local SQLite/PostgreSQL DB. When it is time to push, invoke `bridle sync-prs` with `--max-prs-per-hour` so the underlying framework manages fork reuse and deployment safely.
