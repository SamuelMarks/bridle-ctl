#![deny(missing_docs)]
#![warn(missing_docs)]
//! Bridle CLI library components for programmatic tool execution.

/// Batch sandboxed execution engine.
pub mod batch_executor;
/// Batch fix module.
pub mod batch_fix;
/// Batch pipeline running, resuming, and status checking.
pub mod batch_pipeline;
/// DB action executor.
pub mod db;
/// Error types and handling for the CLI.
/// Git mutator and Forge API client.
pub mod forge_mutator;
/// Organization ingestion module.
pub mod ingest;
/// PR templating rendering and evaluation.
pub mod pr_templating;
/// Runner module to execute tools.
pub mod runner;
/// Sync PRs module.
pub mod sync_prs;
/// Tools and tools registry.
pub mod tools;
/// Terminal User Interface module.
pub mod tui;
/// Ephemeral workspace controller.
pub mod workspace;
