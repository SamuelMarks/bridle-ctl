<div align="center">
  [![CI](https://github.com/SamuelMarks/bridle-ctl/actions/workflows/ci.yml/badge.svg)](https://github.com/SamuelMarks/bridle-ctl/actions/workflows/ci.yml)
  ![Test Coverage](https://img.shields.io/badge/Test%20Coverage-100.00%25-brightgreen)
  ![Doc Coverage](https://img.shields.io/badge/Doc%20Coverage-100%25-brightgreen)
  <img src="./banner.svg" alt="bridle-ctl banner" />
</div>

# bridle-ctl: The AI-Native Codebase Orchestrator

> **Because your LLM needs reins, not a blank check.**

`bridle-ctl` is a comprehensive AI-native codebase orchestrator and embedded Git Forge backend. It was built from the ground up to solve the fundamental problem of AI code generation at scale: **hallucinated, non-deterministic string replacements**.

Instead of letting AI agents blindly manipulate raw text, `bridle-ctl` forces agents to interact with your codebase through **compiled, deterministic FFI (Foreign Function Interface) tools**.

Furthermore, `bridle-ctl` provides a completely offline, local "Git Forge" (via SQLite or PostgreSQL) complete with Issues, Pull Requests, Commits, and Repositories. This allows an entire simulated AI Engineering team to operate, debate, and generate code concurrently without ever touching the network—until you are ready to synchronize the results upstream.

## 🚀 What Does This Project Do?

1. **Deterministic Refactoring**: Wraps targeted code mutations into compiled C/C++/Go/Rust binaries that the AI invokes via an MCP (Model Context Protocol) agent.
2. **Embedded Git Forge**: Simulates GitHub/GitLab locally. Agents create Issues, assign themselves, write code, run tests, and open Pull Requests against the local database.
3. **Upstream Synchronization**: Once local PRs are reviewed (by humans or other AI agents), the system seamlessly synchronizes them to upstream providers (e.g., GitHub), automatically managing forks, branches, and strict global PR rate limits to prevent API spam.
4. **Unified API Surface**: Provides CLI, REST, JSON-RPC, and an Angular web UI to monitor, control, and execute these AI workflows.

## 🏗️ Core Workflows

`bridle-ctl` is built around two primary, automated workflows designed to safely scale AI-driven codebase mutations across organizations:

### Workflow 1: Codebase Mutation Pipeline

0. **Target**: Specify a target GitHub org (e.g., `example-org`).
1. **Clone**: Clone down all non-readonly, updated-in-past-year, non-fork repositories.
2. **Build**: Build the target code using custom Dockerfiles dynamically generated with [mkconf](https://github.com/SamuelMarks/mkconf).
3. **Execute**: If the build succeeds, proceed to tool execution (e.g., running [go-auto-err-handling](https://github.com/SamuelMarks/go-auto-err-handling) on a Go project).
4. **Validate**: If the build and tool execution succeed (resulting in changed code files, e.g., `*.go`), proceed.
5. **Mark**: Mark it as a successful patch and a candidate for sending a Pull Request back to the target org.

### Workflow 2: PR Synchronization (Rate-Limited)

0. **Queue**: Pull from the queue of successful candidates ready for PR submission.
1. **Template**: Interpolate details into the repository's PR template (or create a new one from scratch if none exists).
2. **Fork**: Fork the repository (if not already done) or reuse the existing fork on your specified `fork_org`.
3. **Push & PR**: Send the PR back to the upstream org, enforcing strict rate limits (e.g., max 10 PRs per hour) to prevent API bans.

## 🏗️ How It Works (Architecture Overview)

The workspace is divided into several specialized crates and applications:

- **`bridle-sdk`**: The core data access and execution library. Contains the dynamic Diesel schemas for SQLite and PostgreSQL, file locking mechanisms, encoding normalizers, and the FFI bridging logic that calls compiled tools.
- **`bridle-cli`**: The primary user interface for administrators. Handles batch task execution, PR syncing, and running individual tools via a Terminal UI (TUI).
- **`bridle-agent`**: The MCP Server and daemon loop. It acts as the "AI Engineer," constantly polling the local database for new Issues, cloning the local repository, executing FFI tools to fulfill the Issue, and submitting local PRs.
- **`bridle-ui`**: A modern Angular 18+ application providing a visual dashboard to monitor AI agent activity, review pending PRs, and manually trigger batch operations.
- **`bridle-rest` & `bridle-rpc`**: API gateways for external tools or scripts to interact with the local Git Forge and tool registry.

For a deeper dive, see [ARCHITECTURE.md](./ARCHITECTURE.md).

## 🛠️ How to Use It

To get started, you can run the agent loop, start the REST APIs, or execute a batch pipeline directly from the CLI.

```bash
# Start the AI Agent daemon
bridle agent

# Run a batch fix across multiple repositories
bridle batch-fix --org "my-org" --issue "Fix deprecated API" --tools "ffi_fixer" --db-url "postgres://user:pass@localhost/bridle"

# Sync approved local PRs to GitHub (creates forks automatically)
bridle sync-prs --org "my-org" --db-url "sqlite://bridle.db" --max-prs-per-hour 10
```

For full usage instructions, including environment variables and batch configuration, see [USAGE.md](./USAGE.md).

## 🔌 How to Extend It (Adding Tools)

The true power of `bridle-ctl` lies in its extensible toolchain. Instead of teaching an LLM how to perfectly rewrite a complex regex, you write a deterministic tool (in Rust, Go, C++, etc.), compile it to a shared library, and register it via FFI. The AI agent simply decides _when_ and _where_ to run the tool.

With the new dynamic TOML plugin architecture, you can now add compiled FFI tools without recompiling the orchestrator. Simply drop a `.toml` configuration file mapping your tool's ABI functions into the `.bridle-plugins/` directory and enable it in `bridle-tools.toml`.

For a step-by-step guide on creating, wrapping, and registering new tools, see [ADD_NEW_TOOLS.md](./ADD_NEW_TOOLS.md).

## 🧰 Built-in Tool Registry

`bridle-ctl` ships with several built-in FFI wrappers and utilities:

| Tool Name                      | Description                                                      | Target Language |
| :----------------------------- | :--------------------------------------------------------------- | :-------------- |
| `go-auto-err-handling`         | Automatically injects `if err != nil { return err }` blocks.     | Go              |
| `type-correct`                 | Resolves standard C/C++ type inconsistencies via AST parsing.    | C/C++           |
| `lib2notebook2lib`             | Bi-directional sync between Python source and Jupyter notebooks. | Python          |
| `cdd-extern-c`                 | Safely wraps headers in `extern "C"` blocks.                     | C/C++           |
| `encoding-normalizer`          | Standardizes file encodings (UTF-8) and line endings (LF).       | Any             |
| `rust-unwrap-to-question-mark` | Safely refactors `.unwrap()` calls to idiomatic `?` usage.       | Rust            |

## 📖 Further Reading

- [ARCHITECTURE.md](./ARCHITECTURE.md) - System design and database schema details.
- [USAGE.md](./USAGE.md) - CLI commands, server execution, and pipeline config.
- [ADD_NEW_TOOLS.md](./ADD_NEW_TOOLS.md) - Guide to extending the system with new FFI tools.
- [AGENT_GUIDELINES.md](./AGENT_GUIDELINES.md) - Operational rules for AI Contexts.
- [SKILLS.md](./SKILLS.md) - Out-of-the-box capabilities provided by the framework.
- [ROADMAP.md](./ROADMAP.md) - Project history, completed phases, and future plans.
