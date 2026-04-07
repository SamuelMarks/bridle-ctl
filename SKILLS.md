# Skills Map

## Purpose
This document maps out the available `CodeTool` integrations and skills that the `bridle-ctl` ecosystem currently supports as a hybrid AI Codebase Orchestrator and Embedded Git Forge.

`bridle-ctl` provides an interconnected ecosystem combining codebase manipulation tools and local database commands.

## Codebase Orchestration Tools (`CodeTool`)
- **`gha-improver`**: Specialized tool for auditing and fixing GitHub Actions (`\.github/workflows/.*\.ya?ml$`).
- **`cdd-c`**: Code generation and refactoring using `cdd-c` logic (`.*\.[ch]$`).
- **`type-correct`**: Automated type corrections for Python (`.*\.py$`).
- **`go-auto-err-handling`**: Automatic error handling generation for Go (`.*\.go$`).
- **`lib2notebook2lib`**: Converts Python libraries to notebooks and back (`.*\.py$`).
- **`rust-unwrap-to-question-mark`**: Converts unwraps to `?` operator (`.*\.rs$`).

## Git Forge Skills (Database)
- **`bridle db`**: Direct management of the local SQLite Git Forge backend.
  - **CRUD Operations**: Complete control over `Users`, `Repos`, `Issues`, `PullRequests`, `Commits`, and other Git entities.
  - **Mock Lifecycles**: Enable autonomous agents to track the state of their fixes by generating local Pull Requests and Issues inside the `bridle.db`.

## Core Principles

1. **Determinism via FFI**: Managing execution through strict FFI calls guarantees predictable, robust code manipulation across all ecosystems.
2. **Context Size Preservation**: Regex-targeting allows agents to mutate code independently of inherent prompt-size constraints.
3. **Workflow Autonomy**: The embedded Git Forge allows agents to perform end-to-end tasks (from issue discovery to PR creation) without external network dependencies.