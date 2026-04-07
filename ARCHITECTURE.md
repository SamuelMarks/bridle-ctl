# Architecture

## Purpose
This document outlines the structural design of the `bridle-ctl` system, reflecting the current repository state consisting of an overarching `CodeTool` interface for codebase orchestration and a comprehensive SQLite-based Git Forge backend.

The system is designed around a modular `CodeTool` interface for safe, FFI-backed file mutations, alongside a robust relational database schema to mock and manage complex Git workflows locally.

## The Git Forge Backend (`bridle-sdk/db`)

`bridle-ctl` includes a full embedded Git Forge powered by Diesel and SQLite.
- **Entities Mapped:** Users, Organisations, Repositories, Branches, Commits, Trees, Blobs, Pull Requests, Issues, Milestones, and Webhooks.
- **Purpose:** By embedding this state, autonomous agents can perform complete software lifecycle workflows (opening PRs, requesting reviews, creating issues) entirely offline or within an isolated environment, bridging the gap between raw file editing and high-level project management.
- **Migrations:** Automated via `diesel_migrations` on startup.

## `CodeTool` Interface

Tools for modifying the actual source code files must implement:
- `name()`: Identifier.
- `description()`: Brief summary.
- `match_regex()`: The regex pattern of files the tool applies to.
- `audit(args)`: Logic for identifying issues.
- `fix(args, dry_run)`: Logic for correcting issues, supporting dry-runs.

### Regex-Based Context Management

By defining file targets through **regex matching**, `bridle-ctl` limits the amount of context an AI agent needs to parse. A tool targeting `.*\.go$` only processes Go files, preventing uncontrolled dumps of entire codebases into the LLM context window.

### Determinism via FFI

Codebase mutations are handled by compiled C-ABI plugins (`unsafe extern "C"`). This ensures identical execution across runs, preventing the subtle hallucinations common with LLM-generated refactors.

## API Layers

`bridle-ctl` exposes both the `CodeTool` operations (`ToolRunRequest`) and the Git Forge CRUD operations through multiple interfaces:
1. **`bridle-cli`**: Subcommands for `audit`, `fix`, `db` (CRUD operations), and server bootstrapping (`rest`, `rpc`, `agent`).
2. **`bridle-rest`**: Actix-Web server providing standard HTTP interfaces for running tools and managing Git entities.
3. **`bridle-rpc`**: High-performance JSON-RPC server (via `jsonrpsee`) for seamless, typed agent-to-backend communication.
