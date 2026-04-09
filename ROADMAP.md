# Roadmap

## Purpose
This document outlines the completed trajectory for the `bridle-ctl` repository as a hybrid AI Codebase Orchestrator and Embedded Git Forge.

## Core Directives
All roadmap items are strictly aligned with our core directives:
- **Determinism & Consistency**: Code modifications handled by robust, predictable FFI-backed tools.
- **Context Size**: Regex-based targeting ensures minimal context overhead for AI agents.
- **Workflow Autonomy**: Providing agents with a full local Git Forge to manage complex lifecycles (PRs, Issues) without network dependencies.

## Phase 1: Core Refactoring (Complete)
- Define `CodeTool` interface with structured enums and 100% test coverage.
- Change tool matching to regex strings (`match_regex()`).
- Support `ToolRunRequest` configuration (pattern, tools, tool_args, dry_run).

## Phase 2: Git Forge Backend & APIs (Complete)
- Build robust Diesel/SQLite database schema for Git entities (`Users`, `Repos`, `Issues`, `PRs`, `Commits`, `Trees`, `Blobs`).
- Implement REST API (`actix-web`) providing full Git Forge CRUD and CodeTool execution.
- Implement JSON-RPC (`jsonrpsee`) server for seamless agent communication.
- Implement `db` CLI subcommands for database operations.

## Phase 3: Agent Protocol & Interface Polish (Complete)
- Construct Terminal User Interface for interactive tool running.
- Deepen MCP (Model Context Protocol) integration within `bridle-agent` so agents can use both CodeTool fixes and DB actions seamlessly.

## Phase 4: Mass Deployment & AI Workflows (Complete)
- Connect FFI codeblocks to actually map and mutate real codebase elements deterministically.
- Agent loop interactions (Bridle-Agent), strictly enforcing context size preservation.
- Full offline-first "AI Engineering Team" simulations, letting agents spawn issues, write code, run audits, and merge PRs within the `bridle-sdk` database.

## Current Milestone: Upstream Integration & Scale
- [x] Dynamic Dual DB (SQLite & PostgreSQL).
- [x] 100% Documentation and Test Coverage.
- [x] Upstream Fork detection and reuse.
- [x] Remote Pull Request dispatching.
- [x] Global rate-limiting constraints for outbound PRs (e.g., max PRs per hour).
- [x] Angular UI Dashboard (`bridle-ui`) for real-time monitoring and task execution.
- [x] Strict Angular 18+ components (SignalStore, fully typed, OnPush change detection).
- [x] Playwright End-to-End test suite coverage for the UI.
