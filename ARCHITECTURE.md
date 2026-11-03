# Architecture

## Purpose

`ARCHITECTURE.md` details the systemic layout, modular design, and data paradigms driving the `bridle-ctl` orchestrator. The architecture is designed to enforce **deterministic codebase mutations** via FFI boundaries while providing a **local, embedded Git Forge** for AI agents to operate within autonomously.

---

## 🏗️ System Components

The workspace is organized into discrete, highly cohesive crates and applications.

### 1. Models (`bridle-models`)

The shared domain model and schema layer.

- **Git Forge Models**: Defines the structs and Diesel schemas for local `Users`, `Organisations`, `Repositories`, `Teams`, `Branches`, `BranchProtectionRules`, `Keys`, `Follows`, `Stars`, `OrgMemberships`, `RepoCollaborators`, `Milestones`, `Labels`, `Issues`, `IssueLabels`, `PullRequests`, `PullRequestReviews`, `Releases`, `Webhooks`, `Commits`, `Trees`, `Blobs`, `BatchJobs`, and `BatchTasks`.
- **Request Payloads**: Houses shared operational request structs (`ToolRunRequest`, `BatchRunRequest`, `BatchFixRequest`, `SyncPrsRequest`).

### 2. SDK (`bridle-sdk`)

The foundational data and execution layer.

- **Database Abstraction**: Utilizes Diesel ORM (`db.rs`, `batch_db.rs`) to support dynamic database connections (`DbConnection::Sqlite` and `DbConnection::Pg`). Based on the connection URI (`postgres://` vs SQLite path), it automatically applies embedded migrations and executes dialect-specific queries.
- **Unified Error Handling**: Defines a strongly-typed `BridleError` enum (`error.rs`) powered by `derive_more`, implementing both Actix-Web `ResponseError` and JSON-RPC error conversions.
- **FFI Tool Bindings**: The `ffi.rs` module wraps statically linked C/C++/Go libraries (`type-correct`, `go-auto-err-handling`, `cdd-c` CST transformers) and `lib2notebook2lib` behind safe Rust functions with RAII argument cleanup (`RawCArgv`) and `PathScope` enforcement (`path_scope.rs`).
- **Concurrency & Encoding**: Implements `file_lock.rs` (`ExclusiveFileLock` RAII guard) for granular, process-safe exclusive file mutations and `encoding.rs` for encoding-preserving text reads/writes (UTF-8, UTF-8 BOM, UTF-16LE/BE, Windows-1252) and line-ending normalization.

### 3. CLI (`bridle-cli`)

The operator's command-line interface, orchestrating local tool execution (`runner.rs`, `tools/`) and two core automated workflows.

#### Workflow 1: Codebase Mutation Pipeline (`batch_pipeline.rs`, `batch_fix.rs`, `batch_executor.rs`)

0. **Specify Target**: Set the target GitHub org (e.g., `example-org`) and ingest repositories (`ingest.rs`).
1. **Clone**: Clone all non-readonly, non-fork repositories updated in the past year (`workspace.rs`).
2. **Build Preparation**: Build the code with custom Dockerfiles dynamically generated via [mkconf](https://github.com/SamuelMarks/mkconf).
3. **Execution**: If the build succeeds, execute the tool (e.g., `go-auto-err-handling` on a Go project).
4. **Validation**: If both build and tool succeed (resulting in changed code, e.g., `*.go` files).
5. **Candidate Promotion**: Mark it as a successful patch and queue it as a candidate for sending an upstream PR.

#### Workflow 2: Upstream Synchronization (`sync_prs.rs`, `forge_mutator.rs`)

0. **Queue Processing**: Pull from the queue of successful candidates meant for the target org.
1. **PR Templating**: Interpolate details into the repo's PR template or create one from scratch if absent.
2. **Fork Management**: Fork the repo (if needed) or reuse an existing fork (on a specified `fork_org`).
3. **Send PR**: Push branches and send the Pull Request while enforcing strict global rate limits (e.g., max 10 PRs an hour) to prevent API bans.

- **Interactive TUI**: Uses `ratatui` and `crossterm` (`tui.rs`) to display real-time batch job progress and task statuses.

### 4. AI Agent (`bridle-agent`)

The Model Context Protocol (MCP) server and autonomous daemon.

- **Simulation Loop**: Continuously polls the local database (`agent_loop.rs`) for open `Issues` assigned to the AI, selects tools/patterns from the issue metadata, applies fixes, opens local PRs, and marks resolved issues as `"closed"`.
- **Execution Pipeline**: Clones or inspects the target repository, invokes deterministic tools via `bridle-cli`, commits changes, and records local PRs.
- **Multi-Agent Simulation**: Capable of simulating different roles (Engineer, Reviewer, PM) natively without network access (`simulation.rs`, `team_simulation.rs`).

### 5. API Services (`bridle-rest` & `bridle-rpc`)

Gateways for external integration.

- **REST API (`actix-web`)**: Exposes HTTP endpoints for CRUD operations on all Git Forge entities as well as `/api/*` dashboard routes (`/api/orgs`, `/api/batch/*`, `/api/prs/*`, `/api/local/*`, `/api/dev/*`). Serves as the backend for `bridle-ui`.
- **JSON-RPC (`jsonrpsee`)**: A high-performance RPC layer specifically tailored for internal agent-to-agent or script-to-agent communication.

### 6. Frontend Dashboard (`bridle-ui`)

An Angular 22 web application.

- **State Management**: Built with native Angular Signals (`signal()`, `computed()`) inside injectable stores (`JobsStore`, `PrsStore`) backed by stateless HTTP data-access services.
- **Visualizations**: Displays system health, organization/repository ingestion, local audit/fix operations, batch pipeline progress, and upstream PR sync controls.

---

## 🧬 The FFI & Dynamic Plugin Paradigm

The core philosophy of `bridle-ctl` is that **LLMs should not rewrite code using raw strings or simple regex**.

Instead, tools are written in languages suited for AST/CST manipulation (C, C++, Rust, Go, Python), compiled or packaged, and exposed via `CodeTool` implementations configured in `bridle-tools.toml` and `.bridle-plugins/*.toml`.

### Supported C ABI Contracts

1. **Statically-Linked SDK FFI (`bridle-sdk/src/ffi.rs`)**:
   - Single-file audit/fix functions (`type_correct_audit(target_path)`, `type_correct_fix(target_path, dry_run)`, `GoAutoErrAudit(target_path)`, `GoAutoErrFix(target_path, dry_run)`).
   - Multi-argument CST transformers (`cli_cst_transformer_main(argc, argv)`) invoked via RAII `RawCArgv`.
2. **Dynamically Loaded Shared Libraries (`dlopen` in `bridle-cli/src/tools/dynamic.rs`)**:
   - `int tool_audit(const char** args_ptr, size_t args_len, char** out_ptr)`
   - `int tool_fix(const char** args_ptr, size_t args_len, bool dry_run, char** out_ptr)`

By forcing agents to use this deterministic tool boundary:

- Context windows are kept small (the agent only needs to specify the tool and target pattern/file).
- Syntax errors introduced by LLM hallucinations are eliminated.
- The same tool can be used interactively by a human via `bridle-cli`, via REST/RPC, or autonomously by `bridle-agent`.

---

## 💾 Database Schema Overview

The embedded Git Forge schema is fully defined in Diesel and includes:

- **`org_and_repo_memberships`**: RBAC for local operations.
- **`issues_and_milestones`**: The task queue for AI agents.
- **`prs_releases_webhooks`**: Tracks code changes proposed by agents.
- **`git_internals`**: (Optional) For deep blob/tree tracking if standard filesystem git is insufficient.
- **`batch_jobs_and_tasks`**: State tracking for large-scale migrations, enabling pause, resume, and retry capabilities.
