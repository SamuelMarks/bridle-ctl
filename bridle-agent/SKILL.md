# Agent SKILL

## Purpose
`SKILL.md` is the system prompt mapping for the inner `bridle-agent` MCP daemon.

## Execution Directives
- **Forking/Syncing**: Call `bridle sync-prs` when tasks are done. It natively reuses forks and handles the upstream flow via global hourly limits. Do not attempt to use `curl` to GitHub directly; use the internal CLI.
- **DB Operations**: Use the `DbConnection` wrapper logic dynamically matching the DB URI prefix (`postgres://` or `sqlite://`).
