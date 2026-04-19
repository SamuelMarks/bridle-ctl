# Usage

`bridle-ctl` provides a unified command-line interface for orchestrating AI agents, executing deterministic codebase mutations, and synchronizing local data with remote Git providers.

## 📥 Installation

Ensure you have Rust and Cargo installed, then build the workspace:

```bash
cargo build --release
```

The compiled binaries will be available in `target/release/`. You can also run the tools directly using `cargo run --bin <binary_name>`.

---

## 🤖 Running the AI Agent

The agent runs as a daemon, continuously polling the local database (the embedded Git Forge) for open issues, executing necessary FFI tools, and submitting local Pull Requests.

```bash
# Start the agent using a local SQLite database
bridle agent --db-url "sqlite://bridle.db"

# Start the agent using a PostgreSQL database
bridle agent --db-url "postgres://user:pass@localhost/bridle"
```

## 🌐 Starting the APIs

If you are using the Angular frontend (`bridle-ui`) or need to connect external scripts, start the API servers:

```bash
# Start the REST API (Actix-Web) on port 8080
bridle rest --port 8080 --db-url "sqlite://bridle.db"

# Start the JSON-RPC server on port 8081
bridle rpc --port 8081 --db-url "sqlite://bridle.db"
```

---

## 🛠️ Batch Operations & Refactoring

`bridle-ctl` excels at large-scale, automated refactoring. You can configure pipelines to run specific FFI tools across hundreds of repositories.

### Running a Batch Pipeline from Config

Define your pipeline in a YAML or TOML file (e.g., `pipeline.yml`), then execute it:

```bash
# Start a new batch job
bridle batch-run --config pipeline.yml --db-url "postgres://user:pass@localhost/bridle"

# Check the status of a specific job
bridle batch-status --job-id 12 --db-url "postgres://user:pass@localhost/bridle"

# Resume a paused or failed job
bridle batch-resume --job-id 12 --db-url "postgres://user:pass@localhost/bridle"
```

### Direct Batch Fix via CLI

You can also bypass the config file for quicker, targeted operations:

```bash
bridle batch-fix \
  --org "my-organization" \
  --issue "Refactor legacy error handling" \
  --tools "go-auto-err-handling" \
  --db-url "sqlite://bridle.db"
```

---

## ☁️ Upstream Synchronization

As your AI agents operate, they generate Pull Requests *locally* within the `bridle-ctl` database. When you are ready to publish these changes to an upstream provider (like GitHub or GitLab), use the `sync-prs` command.

```bash
bridle sync-prs \
  --org "my-organization" \
  --db-url "sqlite://bridle.db" \
  --max-prs-per-hour 10
```

### Workflow 1: Codebase Mutation Pipeline
When running a batch operation (e.g., via `bridle batch-fix`), the system executes the following workflow:
0. **Target Identification**: Specify a target GitHub org (e.g., `google`).
1. **Clone**: Clone down all non-readonly, non-fork repositories that were updated in the past year.
2. **Build Validation**: Build the repository using custom Dockerfiles generated dynamically via [mkconf](https://github.com/SamuelMarks/mkconf).
3. **Tool Execution**: If the build succeeds, proceed to tool execution (e.g., running [go-auto-err-handling](https://github.com/SamuelMarks/go-auto-err-handling) on a Go project).
4. **Verification**: If both the build and tool run succeed, resulting in modified code files (e.g., `*.go` files).
5. **Candidate Promotion**: Mark the result as a successful patch and queue it as a candidate for an upstream Pull Request.

---

## ☁️ Upstream Synchronization

As your AI agents operate, they generate Pull Requests *locally* within the `bridle-ctl` database. When you are ready to publish these changes to an upstream provider (like GitHub or GitLab), use the `sync-prs` command.

```bash
bridle sync-prs \
  --org "google" \
  --fork-org "my-fork-org" \
  --db-url "sqlite://bridle.db" \
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

For convenience, you can set the following environment variables instead of passing CLI flags:

- `BRIDLE_DB_URL`: The database connection string (e.g., `sqlite://bridle.db` or `postgres://...`).
- `BRIDLE_GITHUB_TOKEN`: Your GitHub Personal Access Token (required for `sync-prs` to create forks and upstream PRs).
- `BRIDLE_LOG`: Log level (`debug`, `info`, `warn`, `error`).