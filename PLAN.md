# Project Plan

## Purpose

This document tracks the strategic steps, completed milestones, and actionable items for the `bridle-ctl` project. As of the current state, all initial phases have been successfully completed, establishing the foundation of the AI Codebase Orchestrator and Embedded Git Forge.

---

## 🎯 Strategic Alignment

Every step in this plan was designed to reinforce our core pillars:

1. **Determinism over Hallucination**: AI code changes must execute via compiled, syntax-aware FFI tools.
2. **Context Size Reduction**: Targeted tool execution prevents LLM context bloat and reduces inference costs.
3. **Workflow Autonomy**: Providing agents with a full local Git Forge (DB) to manage PRs and Issues natively without immediate upstream synchronization.

---

## ✅ Completed Action Items

### Phase 1: Engine Foundation
- [x] **Refactor Core**: Implement the `CodeTool` Rust trait/interface.
- [x] **Regex Matching**: Allow agents to target files efficiently using regex bounds rather than reading full directories.
- [x] **FFI Codeblock Wiring**: Connect FFI endpoints to native handlers for `.so`/`.dylib` execution.

### Phase 2: Embedded Git Forge
- [x] **Database Schema**: Full SQLite and PostgreSQL (Diesel) schemas created for Users, Repos, Issues, PRs, Commits, Trees, and Blobs.
- [x] **Dual DB Support**: Dynamic `DbConnection` routing based on URI connection string (`postgres://` vs `sqlite://`).
- [x] **CRUD Implementation**: Complete data-access methods built in `bridle-sdk`.

### Phase 3: APIs and CLI
- [x] **Unified APIs**: Integrated `ToolRunRequest` and DB operations across `bridle-rest` (Actix Web) and `bridle-rpc` (jsonrpsee).
- [x] **TUI Experience**: Interactive runner selection via Terminal UI (`ratatui`) for human-in-the-loop oversight.
- [x] **Batch Pipelines**: Config-driven YAML/TOML pipelines for executing mass migrations.

### Phase 4: Agent AI Simulations
- [x] **Agent MCP Protocol**: Built the `bridle-agent` MCP integration, enabling agents to autonomously trigger tools.
- [x] **Continuous Loop**: Engineered the daemon polling loop mapping FFI tools to pending issue states.
- [x] **"AI Engineering Team" Simulations**: Scripted interactions orchestrating independent PM, Engineer, and Reviewer autonomous roles locally.

### Phase 5: Production and Sync
- [x] **Upstream PR Sync (Workflow 2)**: Developed the engine to pull from the queue of candidates, fork (or reuse fork), interpolate PR templates, and push to remote repositories.
- [x] **Global Rate Limiting**: Strict constraints (e.g., max 10 PRs an hour) enforced during Workflow 2 to prevent upstream API bans.
- [x] **Codebase Mutation Pipeline (Workflow 1)**: Integrated cloning, `mkconf` dockerfile generation, build validation, tool execution (like `go-auto-err-handling`), and marking as successful candidates.
- [x] **Smart Fork Management**: Automatic remote fork detection, creation, and reuse on specified `fork_org`.
- [x] **Frontend Dashboard**: Deployed `bridle-ui` (Angular 18+, NgRx SignalStore, Playwright E2E).
- [x] **100% Coverage**: Achieved and enforced 100% Rust unit test coverage and 100% documentation coverage metrics.

**Status: All Planned Phases Complete.** 

*(Note: New planning items will be added as community feedback and enterprise requirements are gathered for the v2.0 cycle.)*