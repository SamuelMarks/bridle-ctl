# Usage Guide

## Purpose
This document outlines how to use `bridle-ctl` and its associated tools, reflecting the current repository state where regex-based code tooling AND local Git Forge database commands are fully implemented.

`bridle-ctl` acts as a unified facade for codebase operations and embedded Git data management using the `bridle` executable.

## Core Commands

```bash
bridle audit [OPTIONS]
bridle fix [OPTIONS]
bridle db [OPTIONS]
bridle rest
bridle rpc
bridle agent
```

## Options: Codebase Operations (Audit/Fix)

- `--pattern <REGEX>`: The regex pattern to target (e.g., `.*\.go$`). 
- `--tools <TOOL1,TOOL2>`: Specify exact tools to run.
- `--tool-args <TOOL:ARG,...>`: Pass arbitrary flags to specific tools.
- `--dry-run`: (Fix only) Simulate the change without actually editing files.

### Determinism & Context Size
By explicitly defining `pattern` and `tools`, users bypass interactive modes, delegating codebase modifications to pre-compiled, FFI-backed logic. The `--pattern` flag ensures that the tool only reads matching files, aggressively reducing the context size that tools and AI agents must process.

## Options: Git Forge (Db)

The `db` command allows you to interact with the local SQLite Git Forge backend.

- `--db-url <PATH>`: The path to the SQLite database (defaults to `bridle.db`).
- `--action <ACTION>`: The CRUD action (e.g., `create_user`, `get_repo`, `create_pull_request`).
- `--payload <JSON>`: The JSON representation of the entity for `create_*` actions.
- `--id <ID>`: The ID of the entity for `get_*` actions.

## Examples

### Fix GitHub Actions (Dry-Run)
```bash
bridle fix --pattern '\.github/workflows/.*\.ya?ml$' --tools gha-improver --dry-run
```

### Create a Local Pull Request
```bash
bridle db --action create_pull_request --payload '{"id":1, "repo_id":1, "number":1, "title":"Fix auth", "state":"open", "head_branch":"fix-auth", "base_branch":"main", "author_id":1, "is_draft":false, "created_at":"2026-04-07T00:00:00", "updated_at":"2026-04-07T00:00:00"}'
```

### Fetch a User via CLI
```bash
bridle db --action get_user --id 1
```

### Start Servers
```bash
bridle rest  # Starts the Actix REST API
bridle rpc   # Starts the JSON-RPC server
```