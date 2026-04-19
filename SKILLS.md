# Capabilities & Skills

## Purpose

`SKILLS.md` defines the comprehensive list of AI-enabled capabilities and workflows supported out-of-the-box by the `bridle-ctl` orchestrator. These skills are what the `bridle-agent` utilizes to act as an autonomous software engineer.

---

## 🛠️ 1. Deterministic FFI Tool Execution
The core capability. Agents do not write string replacements; they execute compiled tools.
- **`audit(tool, path)`**: Safely scan a repository to see if a specific refactoring tool is applicable.
- **`fix(tool, path, args)`**: Execute a compiled C/C++/Go/Rust binary against a file to update its Abstract Syntax Tree deterministically.
- Includes built-in tools like `type-correct`, `cdd-extern-c`, and `go-auto-err-handling`.

## 🗄️ 2. Embedded Git Forge Operations
Agents simulate GitHub/GitLab natively without touching the network.
- **Issue Management**: Agents can read `Issues`, assign themselves, and update statuses to `In Progress` or `Done`.
- **Local PR Creation**: Agents can propose code changes by writing rows into the local `prs_releases_webhooks` table, attaching diffs and commit hashes.
- **Review Simulation**: Secondary agents can be assigned to review local PRs, providing feedback or approving them.

## 🚀 3. Smart Upstream PR Synchronization
Once a local PR is approved, the system can autonomously push it to the real world.
- **Fork Resolution**: Automatically queries GitHub/GitLab to find an existing personal fork. If none exists, it creates one.
- **Branch Management**: Pushes the local changes to the remote fork.
- **Cross-Repo PRs**: Submits the Pull Request to the upstream organization's repository.
- **Global Rate Limiting**: The agent cluster is constrained by strict configuration (e.g., 10 PRs per hour) to ensure automated mass-refactoring does not trigger API spam filters.

## 🔀 4. Dynamic Database Routing
- **SQLite Support**: Agents can operate on lightweight, file-based databases (`sqlite://bridle.db`) for simple local tests.
- **PostgreSQL Support**: Agents can connect to enterprise data stores (`postgres://...`) for massive concurrent operations.

## 📦 5. Batch Pipeline Execution
For mass migrations, agents or human operators can trigger YAML/TOML pipelines.
- Applies a specific FFI tool (e.g., standardizing copyright headers) across thousands of repositories concurrently.
- State tracking allows pausing, resuming, and retrying failed nodes.