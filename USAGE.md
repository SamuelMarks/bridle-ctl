# Usage

`bridle-ctl` provides a unified command-line interface for orchestrating AI agents, executing deterministic codebase mutations, and synchronizing local data with remote Git providers.

## 📥 Installation

Ensure you have Rust and Cargo installed, then build the workspace:

```bash
cargo build --release
```

The compiled binaries will be available in `target/release/`. You can also run the tools directly using `cargo run --bin <binary_name>`.

---

## 🤖 Running the AI Agent & APIs

The CLI provides subcommands to launch the autonomous agent daemon, the REST API server (`bridle-rest`), the JSON-RPC server (`bridle-rpc`), and a health check. Database connections for these services are configured via `DATABASE_URL` (or `POSTGRES_URL`), defaulting to `bridle.db`.

```bash
# Perform a CLI health check
bridle health

# Start the AI Agent daemon
DATABASE_URL="sqlite://bridle.db" bridle agent

# Start the REST API (Actix-Web) on 127.0.0.1:8080
DATABASE_URL="sqlite://bridle.db" bridle rest

# Start the JSON-RPC server (jsonrpsee)
DATABASE_URL="sqlite://bridle.db" bridle rpc
```

---

## 🔍 Local Auditing, Fixing & Database Operations

You can run registered tools directly against your local workspace or query/mutate the embedded Git Forge database from the CLI:

```bash
# Audit files matching a language alias or regex pattern
bridle audit --pattern "rust" --tools "rust-unwrap-to-question-mark"

# Apply fixes (optionally with --dry-run and per-tool arguments)
bridle fix \
  --pattern "go" \
  --tools "go-auto-err-handling" \
  --tool-args "go-auto-err-handling:--strict" \
  --dry-run

# Ingest all repositories for an upstream organization into the local database
bridle ingest-org --org "my-organization" --provider "github" --db-url "bridle.db"

# Execute a direct JSON database operation (e.g., create_user, get_user, create_issue, get_issue)
bridle db --db-url "bridle.db" --action "get_user" --id 1
```

---

## 🛠️ Batch Operations & Refactoring

`bridle-ctl` excels at large-scale, automated refactoring. You can configure pipelines to run specific FFI tools across hundreds of repositories.

### Running a Batch Pipeline from Config

Define your pipeline in a YAML or TOML file (e.g., `pipeline.yml`), then execute it (using `DATABASE_URL` or the default `bridle.db`):

```bash
# Start a new batch job from a pipeline config
bridle batch-run --config pipeline.yml --safety-mode --max-repos 50 --max-prs-per-hour 10

# Display the status of a specific batch job via the TUI
bridle batch-status --job-id 12

# Resume a paused or interrupted batch job
bridle batch-resume --job-id 12
```

### Direct Batch Fix via CLI

You can also bypass the config file for quicker, targeted operations across an organization:

```bash
bridle batch-fix \
  --org "my-organization" \
  --issue "Refactor legacy error handling" \
  --pattern "go" \
  --tools "go-auto-err-handling" \
  --db-url "bridle.db" \
  --safety-mode \
  --max-repos 25 \
  --max-prs-per-hour 10
```

### Workflow 1: Codebase Mutation Pipeline

When running a batch operation (e.g., via `bridle batch-fix` or `bridle batch-run`), the system executes the following workflow:

0. **Target Identification**: Specify a target GitHub org (e.g., `example-org`).
1. **Clone**: Clone down all non-readonly, non-fork repositories that were updated in the past year.
2. **Build Validation**: Build the repository using custom Dockerfiles generated dynamically via [mkconf](https://github.com/SamuelMarks/mkconf).
3. **Tool Execution**: If the build succeeds, proceed to tool execution (e.g., running [go-auto-err-handling](https://github.com/SamuelMarks/go-auto-err-handling) on a Go project).
4. **Verification**: Verify that both the build and tool run succeed, resulting in modified code files (e.g., `*.go` files).
5. **Candidate Promotion**: Mark the result as a successful patch and queue it as a candidate for an upstream Pull Request.

---

## ☁️ Upstream Synchronization

As your AI agents operate, they generate Pull Requests _locally_ within the `bridle-ctl` database. When you are ready to publish these changes to an upstream provider (like GitHub), use the `sync-prs` command:

```bash
bridle sync-prs \
  --org "example-org" \
  --fork-org "my-fork-org" \
  --db-url "bridle.db" \
  --max-prs-per-hour 10
```

### Workflow 2: Synchronize Pull Requests (with Limits)

The `sync-prs` process follows this strict pipeline to protect your API standing:

0. **Queue Pull**: Pull from the queue of successful patch candidates ready to be sent back to the org.
1. **PR Templating**: Interpolate the patch details into the target repository's PR template, or create a new one from scratch if no template exists.
2. **Fork Management**: Fork the repository (if not done already), otherwise reuse your existing fork (specifically on the target `fork_org`).
3. **Send PR**: Push the branch and send the Pull Request upstream, strictly enforcing rate limits (e.g., max 10 PRs an hour).

---

## 💻 Web UI Integration

To run the Angular Web UI:

```bash
cd bridle-ui
npm install
npm run start
```

Navigate to `http://localhost:4200` to view the graphical dashboard for your embedded Git Forge and batch operations.

---

## ⚙️ Environment Variables

You can configure `bridle-ctl` behavior using the following environment variables:

- `DATABASE_URL` (or fallback `POSTGRES_URL`): The database connection string (e.g., `bridle.db` or `postgres://user:pass@localhost/bridle`).
- `FORGE_TOKEN`: Your GitHub Personal Access Token for creating forks and upstream PRs during batch pipelines.
- `GITHUB_API_URL`: Override the base GitHub API URL (defaults to `https://api.github.com`).
- `GIT_COMMAND`: Override the `git` executable binary used by workspace and forge operations (defaults to `git`).
- `RUST_LOG`: Structured JSON log filter level via `tracing_subscriber` (defaults to `info`).
- `LIB2NOTEBOOK2LIB_BIN`: Optional path override for the `lib2notebook2lib` executable.
