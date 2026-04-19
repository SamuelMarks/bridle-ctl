# Roadmap

## Purpose

This document outlines the completed trajectory and future direction for the `bridle-ctl` repository, evolving from a simple batch-runner into a hybrid **AI Codebase Orchestrator and Embedded Git Forge**.

---

## 🧭 Core Directives

All roadmap items, past and future, are strictly aligned with these principles:

- **Determinism & Consistency**: Code modifications must be handled by robust, predictable FFI-backed tools.
- **Context Efficiency**: Ensure minimal context overhead for AI agents (don't read what you don't need to change).
- **Offline Workflow Autonomy**: Provide agents with a full local Git Forge to manage complex lifecycles without network dependencies.

---

## 🏆 Completed Milestones

### Phase 1: Core Refactoring
- Defined `CodeTool` interface with structured enums and 100% test coverage.
- Changed tool matching to regex strings (`match_regex()`).
- Supported `ToolRunRequest` configuration (pattern, tools, tool_args, dry_run).

### Phase 2: Git Forge Backend & APIs
- Built robust Diesel database schema for Git entities (`Users`, `Repos`, `Issues`, `PRs`, `Commits`, `Trees`, `Blobs`).
- Implemented dual-database support for both SQLite (local/testing) and PostgreSQL (enterprise).
- Implemented REST API (`actix-web`) and JSON-RPC (`jsonrpsee`).

### Phase 3: Agent Protocol & Interface Polish
- Constructed Terminal User Interface (`bridle-cli` TUI) for interactive tool running.
- Deepened MCP (Model Context Protocol) integration within `bridle-agent`.

### Phase 4: Mass Deployment & AI Workflows
- **Workflow 1 Complete**: Orchestrated the codebase mutation pipeline (Target Org -> Clone -> Build via mkconf Dockerfiles -> Execute Tools -> Mark successful patch candidates).
- Connected FFI codeblocks to map and mutate real codebase elements deterministically.
- Built offline-first "AI Engineering Team" simulations, letting agents spawn issues, write code, run audits, and merge PRs entirely within the `bridle-sdk` database.

### Phase 5: Upstream Integration & Scale
- **Workflow 2 Complete**: Upstream PR Sync pipeline (Pull Candidates -> Interpolate PR Template -> Fork/Reuse Fork -> Send PR).
- Implemented Upstream Fork detection and reuse logic on specified `fork_org`.
- Built Remote Pull Request dispatching (`sync-prs`).
- Enforced Global rate-limiting constraints for outbound PRs (e.g., max 10 PRs per hour).
- Developed the Angular UI Dashboard (`bridle-ui`) with strict types, SignalStore, and E2E coverage.

---

## 🔮 Future Horizons (v2.0 & Beyond)

While all initial phases are complete, we are exploring the following concepts for future updates:

- **Distributed Agent Swarms**: Allowing multiple `bridle-agent` daemons across different network nodes to connect to the central `bridle-rest` Postgres database, parallelizing the workload of massive organization-wide migrations.
- **Community FFI Tool Registry**: A centralized package manager for downloading pre-compiled `.so`/`.dylib` tools (e.g., `bridle install tool/react-to-vue-converter`).
- **WASM Tool Support**: In addition to standard C ABI shared libraries, supporting WebAssembly (WASM) plugins for safer, sandboxed, cross-platform deterministic code tools.
- **Enhanced UI Analytics**: Diff visualization heatmaps and agent performance metrics in the Angular dashboard.