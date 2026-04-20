# Architecture

## Purpose

`ARCHITECTURE.md` details the systemic layout, modular design, and data paradigms driving the `bridle-ctl` orchestrator. The architecture is designed to enforce **deterministic codebase mutations** via FFI boundaries while providing a **local, embedded Git Forge** for AI agents to operate within autonomously.

---

## 🏗️ System Components

The workspace is organized into discrete, highly cohesive crates and applications.

### 1. SDK (`bridle-sdk`)

The foundational data and execution layer.

- **Database Abstraction**: Utilizes Diesel ORM to support dynamic database connections. Based on the connection URI (`postgres://` vs `sqlite://`), it automatically applies the correct migrations and executes dialect-specific queries.
- **Git Forge Models**: Defines the schema for local `Users`, `Organizations`, `Repositories`, `Issues`, `PullRequests`, `Commits`, and `Blobs`.
- **FFI Tool Registry**: The `ffi.rs` module loads compiled shared libraries (`.so`, `.dylib`, `.dll`) and defines the `extern "C"` blocks required to invoke deterministic tools.
- **Concurrency Control**: Implements `file_lock.rs` for granular, thread-safe, and process-safe file locking to prevent race conditions when multiple agents attempt to modify the same repository tree.

### 2. CLI (`bridle-cli`)

The operator's command-line interface, orchestrating two core, automated workflows.

#### Workflow 1: Codebase Mutation Pipeline (`batch_pipeline.rs`, `batch_fix.rs`)

0. **Specify Target**: Set the target GitHub org (e.g., `example-org`).
1. **Clone**: Clone all non-readonly, non-fork repositories updated in the past year.
2. **Build Preparation**: Build the code with custom Dockerfiles dynamically generated via [mkconf](https://github.com/SamuelMarks/mkconf).
3. **Execution**: If the build succeeds, execute the tool (e.g., `go-auto-err-handling` on a Go project).
4. **Validation**: If both build and tool succeed (resulting in changed code, e.g., `*.go` files).
5. **Candidate Promotion**: Mark it as a successful patch and queue it as a candidate for sending an upstream PR.

#### Workflow 2: Upstream Synchronization (`sync_prs.rs`)

0. **Queue Processing**: Pull from the queue of successful candidates meant for the target org.
1. **PR Templating**: Interpolate details into the repo's PR template or create one from scratch if absent.
2. **Fork Management**: Fork the repo (if needed) or reuse an existing fork (on a specified `fork_org`).
3. **Send PR**: Push branches and send the Pull Request while enforcing strict global rate limits (e.g., max 10 PRs an hour) to prevent API bans.

- **Interactive TUI**: Uses libraries like `ratatui` or `dialoguer` to allow human operators to step through tool executions interactively.

### 3. AI Agent (`bridle-agent`)

The Model Context Protocol (MCP) server and autonomous daemon.

- **Simulation Loop**: Continuously polls the local database for open `Issues` assigned to the AI.
- **Execution Pipeline**: When an issue is detected, the agent clones the local repo, analyzes the request, decides which FFI tools to invoke, runs them, commits the deterministic changes, and opens a local PR.
- **Multi-Agent Simulation**: Capable of simulating different roles (Engineer, Reviewer, PM) natively without network access.

### 4. API Services (`bridle-rest` & `bridle-rpc`)

Gateways for external integration.

- **REST API (`actix-web`)**: Exposes standard HTTP endpoints for creating issues, viewing PRs, and triggering syncs. Serves as the backend for `bridle-ui`.
- **JSON-RPC (`jsonrpsee`)**: A high-performance RPC layer specifically tailored for internal agent-to-agent or script-to-agent communication.

### 5. Frontend Dashboard (`bridle-ui`)

An Angular 18+ web application.

- **State Management**: Built with NgRx SignalStore for reactive, signal-based state.
- **Visualizations**: Displays pipeline progress, upstream sync queues, rate limiting status, and diff visualizations for local PRs.

---

## 🧬 The FFI Integration Paradigm

The core philosophy of `bridle-ctl` is that **LLMs should not rewrite code using raw strings or simple regex**.

Instead, tools are written in languages suited for AST manipulation (C++, Rust, Go), compiled, and exposed via a strict C Application Binary Interface (ABI).

### The C ABI Contract

Tools expose two primary functions:

1. `audit(const char* file_path)`: Returns a JSON string or struct indicating _if_ the tool should be run and _where_.
2. `fix(const char* file_path, const char* args)`: Executes the deterministic change and returns a success boolean or error string.

By forcing agents to use this FFI boundary:

- Context windows are kept incredibly small (the agent only needs to output: `run_tool("type-correct", "src/main.c")`).
- Syntax errors introduced by LLM hallucinations are reduced to zero.
- The same tool can be used interactively by a human via `bridle-cli`, or autonomously by the agent.

---

## 💾 Database Schema Overview

The embedded Git Forge schema is fully defined in Diesel and includes:

- **`org_and_repo_memberships`**: RBAC for local operations.
- **`issues_and_milestones`**: The task queue for AI agents.
- **`prs_releases_webhooks`**: Tracks code changes proposed by agents.
- **`git_internals`**: (Optional) For deep blob/tree tracking if standard filesystem git is insufficient.
- **`batch_jobs_and_tasks`**: State tracking for large-scale migrations, enabling pause, resume, and retry capabilities.
