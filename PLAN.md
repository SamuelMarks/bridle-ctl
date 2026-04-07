# Project Plan

## Purpose
This document tracks the actionable next steps for the `bridle-ctl` project based on its current state as an AI Codebase Orchestrator and Embedded Git Forge.

## Strategic Alignment
Every step in this plan is designed to reinforce our core pillars:
- **Determinism**: Code changes execute via compiled FFI tools.
- **Context Size Reduction**: Regex-based file targeting prevents context bloat.
- **Workflow Autonomy**: Providing agents with a full local Git Forge (DB) to manage PRs and Issues natively.

## Action Items

1.  **Refactor Core (Done)**: `CodeTool` interface with regex matching implemented.
2.  **Git Forge Backend (Done)**: Full SQLite/Diesel schema created for Users, Repos, Issues, PRs, Commits, Trees, and Blobs. CRUD methods fully implemented in `bridle-sdk`.
3.  **Unified APIs (Done)**: Integrated `ToolRunRequest` and Git Forge CRUD operations across `bridle-cli` (`db` subcommand), `bridle-rest` (Actix Web), and `bridle-rpc` (jsonrpsee).
4.  **TUI Experience (Next)**: Interactive runner selection via Terminal UI, replacing the current simple prompts for richer human-in-the-loop oversight.
5.  **Agent MCP Protocol**: Finalize the `bridle-agent` MCP integration, enabling agents to natively trigger `CodeTool` runs and interact with the local Git Forge DB.
6.  **Complex Workflow Simulation**: Develop scripts that combine codebase mutations (via `fix`) with PR creation (via the SQLite DB) to simulate a complete autonomous AI engineering workflow.
