<div align="center">
  [![CI](https://github.com/SamuelMarks/bridle-ctl/actions/workflows/ci.yml/badge.svg)](https://github.com/SamuelMarks/bridle-ctl/actions/workflows/ci.yml)
  ![Test Coverage](https://img.shields.io/badge/Test%20Coverage-100.00%25-brightgreen)
  ![Doc Coverage](https://img.shields.io/badge/Doc%20Coverage-100%25-brightgreen)
  <img src="./banner.svg" alt="bridle-ctl banner" />
</div>

> **Because your LLM needs reins, not a blank check.**

`bridle-ctl` is an AI-native codebase orchestrator and embedded Git Forge backend. It combines unified tooling interfaces with the reliability of compiled FFI (Foreign Function Interface) plugins for codebase mutation, while providing a fully-featured, local and remote Git database accessible via CLI, REST, and JSON-RPC.

## 🚀 Why `bridle-ctl`?

When giving AI agents access to codebase refactoring, simple prompt-based string replacements often fail at scale. `bridle-ctl` solves this by forcing agents to run _compiled_ tools to mutate code deterministically.

It further acts as a complete "offline Git Forge," providing agents with local Database access (SQLite and PostgreSQL) to create PRs and track Issues, simulating a full AI Engineering team without needing external network access. When ready, it syncs PRs to upstream providers (e.g. GitHub) while automatically reusing forks and globally rate-limiting outbound PRs.

## 📦 Architecture

- **`bridle-sdk`**: The core library. Contains SQLite and PostgreSQL Diesel migrations (dynamically chosen via URI), FFI bindings (`libgoautoerr`, `type-correct`, `cdd-c`), encoding normalization, batch PR pipelines, and universal ToolRun schemas. 100% Test and Doc coverage.
- **`bridle-cli`**: The command-line interface. Features an interactive TUI, batch operations, upstream PR syncing (with fork detection and global rate limits), and the core `Runner` logic for executing regex-targeted tools.
- **`bridle-agent`**: The Model Context Protocol (MCP) server. Simulates AI Engineering Teams and daemon loops that autonomously monitor the database, pick up issues, run CLI tools, and merge PRs.
- **`bridle-rest`**: Actix-Web REST API for executing Git Forge CRUD operations.
- **`bridle-rpc`**: High-performance JSON-RPC server for internal agent tooling.
- **`bridle-ui`**: The Angular-based web frontend. Provides a visual dashboard for monitoring system health, managing batch operations, and synchronizing PRs.

## ✨ Core Features

- **Deterministic AI Mutations**: Compiled FFI tools handle edits.
- **Dual Database Support**: SQLite and PostgreSQL backends via `DbConnection` wrapper.
- **Upstream Sync**: Synchronize batch operations and pending PRs to upstream GitHub repositories.
- **Smart Fork Management**: Automatically checks for forks in orgs/personal accounts and reuses them.
- **Rate-Limited PR Sending**: Global limits on the number of PRs sent upstream (e.g. `max_prs_per_hour`).
- **Batch Processing**: Config-driven batch pipelines and state tracking.

---

## 📖 Further Documentation

- [ARCHITECTURE.md](./ARCHITECTURE.md) - Deep dive into the FFI architecture and Git Forge DB design.
- [USAGE.md](./USAGE.md) - Detailed command line usage, flags, and workflow examples.
- [ROADMAP.md](./ROADMAP.md) - History of completed features and roadmap.
- [PLAN.md](./PLAN.md) - Project plan and action item tracking.
