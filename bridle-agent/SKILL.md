# Bridle-Agent Skill

## Purpose
This document defines the specialized skill guidelines for any `bridle-agent` interacting with the `bridle-ctl` ecosystem.

## Goal
As a `bridle-agent`, your primary objective is to execute highly targeted codebase manipulations using the `ToolRunRequest` model AND to track your workflow state using the embedded Git Forge Database. The current state of the repository leverages a unified `CodeTool` interface with regex-based matching alongside a SQLite/Diesel Git backend.

## Usage Model

You must build predictable routines by executing precise regex-targeted fixes rather than uncontrolled LLM refactoring sessions. Concurrently, you should use the Git Forge DB to manage your state. You interact with CLI, JSON-RPC, or REST using `ToolRunRequest` or the `db` operations.

### Determinism and Consistency

By enforcing a system where structural manipulations happen inside FFI-bound tools (`cdd-c`, `type-correct`, etc.), we eliminate hallucinations and ensure perfect consistency. Ensure you use the provided tools (e.g., `gha-improver`, `rust-unwrap-to-question-mark`) to accomplish complex edits rather than manually editing large source files.

### Context Size Preservation

Instead of processing and maintaining memory of hundreds of files (which degrades reasoning limits and bloats context size rapidly), pass targeted regex descriptors:
`\.github/workflows/.*\.ya?ml$`
`.*\.rs$`
`.*\.py$`
This strictly bounds the target domain and limits your context size usage only to the final JSON reporting.

### Workflow Management (Git Forge)

Instead of maintaining a long context history of tasks, persist your workflow into the embedded Git Forge:
- **Track Bugs:** Use `bridle db --action create_issue` to log bugs you find.
- **Isolate Work:** Use `bridle db --action create_branch` to log your feature branches.
- **Propose Fixes:** After using `bridle fix`, use `bridle db --action create_pull_request` to package the fix logically.

### Execution Plan

1. Perform audits using targeted regexes via the `ToolRunRequest`.
2. Evaluate issues inside the JSON report and optionally track them in the local DB (`create_issue`).
3. Perform dry-runs of the necessary fixes using `--dry-run`.
4. Apply the final fixes based on user instructions or automated rules.
5. Record the final state by creating a PR (`create_pull_request`) in the local Git Forge.