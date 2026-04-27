//! Workflow simulation for autonomous AI engineering.

use bridle_cli::db::execute_db_command;
use bridle_cli::runner::{Action, run};
use bridle_sdk::BridleError;
use bridle_sdk::models::ToolRunRequest;

/// Runs a complete autonomous AI workflow simulation.
///
/// This simulates an agent:
/// 1. Creating a repository in the local forge.
/// 2. Logging an issue.
/// 3. Using `bridle-cli` tools to mutate code.
/// 4. Creating a pull request to resolve the issue.
pub fn run_workflow_simulation(db_url: &str) -> Result<String, BridleError> {
    // 1. Create a Repository
    let repo_payload = r#"{"id": 100, "owner_id": 1, "owner_type": "user", "name": "sim_repo", "is_private": false, "is_fork": false, "archived": false, "allow_merge_commit": true, "allow_squash_merge": true, "allow_rebase_merge": true, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
    execute_db_command(db_url, "create_repo", Some(repo_payload.to_string()), None)
        .map_err(|e| BridleError::Daemon(format!("Failed to create repo: {}", e)))?;

    // 2. Create an Issue
    let issue_payload = r#"{"id": 100, "repo_id": 100, "number": 1, "title": "Fix rust unwraps", "body": "Need to replace unwraps", "state": "open", "author_id": 1, "assignee_id": null, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
    execute_db_command(
        db_url,
        "create_issue",
        Some(issue_payload.to_string()),
        None,
    )
    .map_err(|e| BridleError::Daemon(format!("Failed to create issue: {}", e)))?;

    // 3. Mutate Code (dry-run to be safe)
    let req = ToolRunRequest {
        pattern: Some(".*\\.rs$".to_string()),
        tools: Some(vec!["rust-unwrap-to-question-mark".to_string()]),
        tool_args: None,
        dry_run: Some(true),
        action: Some("fix".to_string()),
    };
    run(Action::Fix { dry_run: true }, req)
        .map_err(|e| BridleError::Daemon(format!("Failed to run code tool: {}", e)))?;

    // 4. Create a Pull Request
    let pr_payload = r#"{"id": 100, "repo_id": 100, "number": 2, "title": "Fix unwraps", "body": "Fixed", "state": "open", "head_branch": "fix-unwraps", "base_branch": "main", "author_id": 1, "assignee_id": null, "is_draft": false, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
    execute_db_command(
        db_url,
        "create_pull_request",
        Some(pr_payload.to_string()),
        None,
    )
    .map_err(|e| BridleError::Daemon(format!("Failed to create PR: {}", e)))?;

    Ok("Workflow simulation completed successfully.".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_workflow_simulation() -> Result<(), BridleError> {
        let tf = tempfile::NamedTempFile::new().map_err(|e| BridleError::Daemon(e.to_string()))?;
        let db_url = tf
            .path()
            .to_str()
            .ok_or(BridleError::Daemon("Invalid path".to_string()))?
            .to_string();

        let result = run_workflow_simulation(&db_url)?;
        assert_eq!(result, "Workflow simulation completed successfully.");

        // Test error paths
        let err_res = run_workflow_simulation("/invalid/path/that/fails");
        assert!(err_res.is_err());

        Ok(())
    }
}
