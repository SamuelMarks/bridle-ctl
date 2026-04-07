# Agent Guidelines

## Purpose
This document serves as the definitive set of operational guidelines for autonomous agents operating within the `bridle-ctl` workspace. It reflects the current repository state, a unified system for both deterministic codebase modifications (`CodeTool`) and offline Git project management (Embedded Git Forge).

As an autonomous agent, you must adhere strictly to determinism, context minimization, and leveraging the local databases to track workflows.

## Core Concepts

### 1. Determinism and Consistency (Fixes)
Instead of manually modifying wide sections of code using raw LLM output, you must leverage the specialized `CodeTool` plugins (via `ToolRunRequest`). These FFI integrations execute regex-targeted fixes identically on every run. 

### 2. Context Size Preservation (Regex)
Use precise regex patterns (e.g., `\.github/workflows/.*\.ya?ml$`) in your `ToolRunRequest`. This ensures tools execute autonomously in Rust and only return minimal JSON reports, sparing your context size from massive dumps.

### 3. Workflow Autonomy (The Git Forge DB)
`bridle-ctl` provides an embedded SQLite Git Forge (`Users`, `Repositories`, `PullRequests`, `Issues`, `Commits`). When working on complex tasks:
- **Track State:** Do not rely purely on memory. Open Issues in the local DB.
- **Branch and Merge:** Simulate Git flows by creating `Branch` and `PullRequest` records via the `bridle db` CLI, REST, or RPC endpoints.
- **Reviews:** Use the local Git Forge to log PR Reviews and Status Checks.

### 4. API & Usage Interactions
Interact with the system using `ToolRunRequest` payloads for codebase tasks, and CRUD payloads for DB tasks.
- For Code: `bridle fix --pattern '...' --tools '...'`
- For DB: `bridle db --action create_issue --payload '...'`
- Alternatively, use the REST API (`:8080`) or JSON-RPC endpoints. Use `--dry-run` to preview large scale refactorings before applying changes.