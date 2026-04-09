# Project Plan

## Purpose
This document tracked the actionable steps for the `bridle-ctl` project. As of the current state, all initial phases have been completed to establish the AI Codebase Orchestrator and Embedded Git Forge.

## Strategic Alignment
Every step in this plan was designed to reinforce our core pillars:
- **Determinism**: Code changes execute via compiled FFI tools.
- **Context Size Reduction**: Regex-based file targeting prevents context bloat.
- **Workflow Autonomy**: Providing agents with a full local Git Forge (DB) to manage PRs and Issues natively.

## Completed Action Items

1.  **Refactor Core (Done)**: `CodeTool` interface with regex matching implemented.
2.  **Git Forge Backend (Done)**: Full SQLite/Diesel schema created for Users, Repos, Issues, PRs, Commits, Trees, and Blobs. CRUD methods fully implemented in `bridle-sdk`.
3.  **Unified APIs (Done)**: Integrated `ToolRunRequest` and Git Forge CRUD operations across `bridle-cli` (`db` subcommand), `bridle-rest` (Actix Web), and `bridle-rpc` (jsonrpsee).
4.  **TUI Experience (Done)**: Interactive runner selection via Terminal UI, replacing the current simple prompts for richer human-in-the-loop oversight.
5.  **Agent MCP Protocol (Done)**: Finalize the `bridle-agent` MCP integration, enabling agents to natively trigger `CodeTool` runs and interact with the local Git Forge DB.
6.  **Complex Workflow Simulation (Done)**: Develop scripts that combine codebase mutations (via `fix`) with PR creation (via the SQLite DB) to simulate a complete autonomous AI engineering workflow.
7.  **FFI Codeblock Wiring (Done)**: Connected FFI endpoints to native handlers mapping the workspace efficiently.
8.  **Agent Loop Interactions (Done)**: Engineered continuous polling loop mapping FFI tools to issue states automatically.
9.  **Full "AI Engineering Team" Simulations (Done)**: Expanded to orchestrate independent PM, Engineer, and Code Reviewer autonomous roles natively offline.

**Status: All Planned Phases Complete.** No pending action items at this time.

## Recent Additions (Completed)
10. **Dual DB Support (Done)**: PostgreSQL and SQLite integration mapped over dynamic URIs.
11. **100% Coverage (Done)**: Enforced strict 100% test coverage and 100% documentation coverage metrics.
12. **Upstream PR Sync & Fork Management (Done)**: Added features to reuse or create remote forks, push code, and globally rate-limit PR generation (e.g., max 10/hr).
13. **Frontend Dashboard (Done)**: Angular 18+ standalone application (`bridle-ui`) for visualizing pipelines, PR syncing, and local ops, featuring strict TypeScript, NgRx SignalStore, and Playwright E2E testing.
