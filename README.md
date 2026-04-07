<div align="center">
  <img src="./banner.svg" alt="bridle-ctl banner" />
</div>

> **Because your LLM needs reins, not a blank check.**

`bridle-ctl` is an AI-native codebase orchestrator and embedded Git Forge backend. It combines unified tooling interfaces with the reliability of compiled FFI (Foreign Function Interface) plugins for codebase mutation, while providing a fully-featured, local SQLite Git database (managing repositories, PRs, issues, users, and git internals) accessible via CLI, REST, and JSON-RPC.

**In plain English:** Instead of an AI hallucinating code across your entire repo and breaking things, `bridle-ctl` forces the AI to use deterministic, compiled plugins to modify files safely. Furthermore, it provides an entire mock Git Forge environment locally, enabling agents to create branches, open pull requests, and manage issues natively without needing an external network connection to GitHub.

---

## 🛑 The Problem: AI Needs Structure

1. **Non-Deterministic Edits:** LLMs hallucinate syntax and context when tasked with broad refactoring. `bridle-ctl` fixes this by providing deterministic `CodeTool` FFI plugins that execute regex-targeted mutations predictably.
2. **Lack of Local Workflow:** Agents typically struggle to manage complex branch, PR, and review workflows locally. `bridle-ctl` embeds a full SQLite-based Git forge, allowing agents to persist repository state, commits, issues, and PRs natively.

---

## ✨ How it Works

1. **Code Orchestration (`CodeTool`):** Agents dispatch `ToolRunRequest` payloads (specifying a regex pattern and a target tool). `bridle-ctl` scans matching files and delegates the safe mutation to a compiled FFI plugin.
2. **Git Forge Backend (`bridle-sdk`):** A robust Diesel/SQLite database schema supports CRUD operations for `Users`, `Repositories`, `PullRequests`, `Issues`, `Commits`, and more.
3. **Multi-Interface Access:** Both the code orchestration tools and the Git forge backend are exposed via the `bridle-cli`, a `bridle-rest` API server, and a `bridle-rpc` JSON-RPC server.

---

## 🚀 Key Features

- **Embedded Git Forge:** Full relational schema for Users, Orgs, Repos, Teams, PRs, Issues, and Git objects (Commits, Trees, Blobs).
- **Smart File Targeting:** FFI tools automatically apply to specific file patterns via internal regex matching, preserving agent context limits.
- **Unified APIs:** Access everything via CLI subcommands (`bridle fix`, `bridle db`), REST (`actix-web`), or JSON-RPC (`jsonrpsee`).
- **Dry-Run Capability:** Simulate changes (`--dry-run`) to verify codebase operations before committing to disk.

---

## ⚡ Getting Started

### Basic Commands
```bash
bridle audit --pattern '.*\.go$' --tools go-err-check
bridle fix --pattern '\.github/workflows/.*\.ya?ml$' --tools gha-improver --dry-run
```

### Git Forge / DB Commands
```bash
# Insert a new user into the local Git forge
bridle db --action create_user --payload '{"id":1,"username":"agent","email":"bot@ai.com","password_hash":"","created_at":"2026-04-07T00:00:00","updated_at":"2026-04-07T00:00:00"}'
# Retrieve user
bridle db --action get_user --id 1
```

### Running the Servers
```bash
bridle rest  # Starts Actix-Web REST API on :8080
bridle rpc   # Starts JSON-RPC server
bridle agent # Starts the Agent daemon
```

---

## 🛠 Project Architecture

- **`bridle-cli`**: The CLI runner combining codebase operations and DB interactions.
- **`bridle-sdk`**: Core library containing `ToolRunRequest`, FFI bindings, Diesel SQLite migrations, and the full Git Forge relational schema.
- **`bridle-agent`**: Wrappers for AI Agent integrations (MCP).
- **`bridle-rest`**: Actix-based REST API for codebase tools and Git Forge CRUD operations.
- **`bridle-rpc`**: High-performance JSON-RPC server for internal agent tooling.

---

## 📖 Further Documentation

- [ARCHITECTURE.md](./ARCHITECTURE.md) - Deep dive into the FFI architecture and Git Forge DB design.
- [USAGE.md](./USAGE.md) - Detailed command line usage, flags, and workflow examples.
- [ROADMAP.md](./ROADMAP.md) - Future plans and upcoming features.
- [PLAN.md](./PLAN.md) - Short-term development tracking.
