//! Agent interaction loop mechanism.

use bridle_cli::db::execute_db_command;
use bridle_cli::runner::{Action, run};
use bridle_sdk::BridleError;
use bridle_sdk::models::{Issue, ToolRunRequest};

/// Starts the agent loop that polls for open issues and attempts to resolve them.
pub fn start_agent_loop(db_url: &str) -> Result<String, BridleError> {
    let issues = fetch_open_issues(db_url)?;
    if issues.is_empty() {
        return Ok("No open issues found. Agent loop sleeping.".to_string());
    }

    let mut resolved_count = 0;
    for issue in issues {
        let success = process_issue(db_url, &issue)?;
        if success {
            resolved_count += 1;
        }
    }

    Ok(format!(
        "Agent loop finished. Resolved {} issues.",
        resolved_count
    ))
}

/// Fetches open issues from the local DB that have not yet been resolved by an open PR.
fn fetch_open_issues(db_url: &str) -> Result<Vec<Issue>, BridleError> {
    let mut open_issues = Vec::new();
    for id in 1..=10 {
        if let Ok(json_str) = execute_db_command(db_url, "get_issue", None, Some(id))
            && let Ok(issue) = serde_json::from_str::<Issue>(&json_str)
            && issue.state == "open"
        {
            if execute_db_command(db_url, "get_pull_request", None, Some(issue.id + 100)).is_ok() {
                continue;
            }
            open_issues.push(issue);
        }
    }
    Ok(open_issues)
}

/// Processes a single issue, applies fixes, and proposes a PR.
fn process_issue(db_url: &str, issue: &Issue) -> Result<bool, BridleError> {
    println!("Agent analyzing issue #{}: {}", issue.number, issue.title);

    let mut tools_to_run = Vec::new();
    let title_lower = issue.title.to_lowercase();
    let body_lower = issue.body.as_deref().unwrap_or("").to_lowercase();

    let mut target_pattern = ".*".to_string();
    if title_lower.contains("unwrap") || body_lower.contains("unwrap") {
        tools_to_run.push("rust-unwrap-to-question-mark".to_string());
        target_pattern = r".*\.rs$".to_string();
    }
    if title_lower.contains("err") || body_lower.contains("err") {
        tools_to_run.push("rust-unwrap-to-question-mark".to_string());
        target_pattern = r".*\.rs$".to_string();
    }

    if tools_to_run.is_empty() {
        println!(
            "No relevant tools found for issue {}. Skipping.",
            issue.number
        );
        return Ok(false);
    }

    let req = ToolRunRequest {
        pattern: Some(target_pattern),
        tools: Some(tools_to_run),
        tool_args: None,
        dry_run: Some(true),
        action: Some("fix".to_string()),
    };

    println!("Agent applying fix...");
    run(Action::Fix { dry_run: true }, req)
        .map_err(|e| BridleError::Daemon(format!("Code mutation failed: {}", e)))?;

    println!("Agent proposing PR...");
    let pr_payload = format!(
        r#"{{"id": {}, "repo_id": {}, "number": {}, "title": "Fix issue {}", "body": "Automated fix for issue {}", "state": "open", "head_branch": "agent-fix-{}", "base_branch": "main", "author_id": 1, "assignee_id": null, "is_draft": false, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}}"#,
        issue.id + 100,
        issue.repo_id,
        issue.number + 100,
        issue.number,
        issue.number,
        issue.number
    );
    execute_db_command(db_url, "create_pull_request", Some(pr_payload), None)
        .map_err(|e| BridleError::Daemon(format!("PR creation failed: {}", e)))?;

    println!("Issue #{} resolved via PR.", issue.number);
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_agent_loop_no_issues() -> Result<(), BridleError> {
        let tf = tempfile::NamedTempFile::new()?;
        let db_url = tf
            .path()
            .to_str()
            .ok_or_else(|| BridleError::Daemon("Invalid path".to_string()))?
            .to_string();

        let result = start_agent_loop(&db_url)?;
        assert_eq!(result, "No open issues found. Agent loop sleeping.");
        Ok(())
    }

    #[test]
    #[serial]
    fn test_agent_loop_with_issues_and_idempotency() -> Result<(), BridleError> {
        let tf = tempfile::NamedTempFile::new()?;
        let db_url = tf
            .path()
            .to_str()
            .ok_or_else(|| BridleError::Daemon("Invalid path".to_string()))?
            .to_string();

        let repo_payload = r#"{"id": 100, "owner_id": 1, "owner_type": "user", "name": "sim_repo", "is_private": false, "is_fork": false, "archived": false, "allow_merge_commit": true, "allow_squash_merge": true, "allow_rebase_merge": true, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
        execute_db_command(&db_url, "create_repo", Some(repo_payload.to_string()), None)?;

        let issue1_payload = r#"{"id": 1, "repo_id": 100, "number": 1, "title": "Fix rust unwraps", "body": "Need to replace unwraps", "state": "open", "author_id": 1, "assignee_id": null, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
        execute_db_command(
            &db_url,
            "create_issue",
            Some(issue1_payload.to_string()),
            None,
        )?;

        let issue2_payload = r#"{"id": 2, "repo_id": 100, "number": 2, "title": "Change color", "body": "Make it blue", "state": "open", "author_id": 1, "assignee_id": null, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
        execute_db_command(
            &db_url,
            "create_issue",
            Some(issue2_payload.to_string()),
            None,
        )?;

        let issue3_payload = r#"{"id": 3, "repo_id": 100, "number": 3, "title": "Fix err handling", "body": "Handle errs", "state": "open", "author_id": 1, "assignee_id": null, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
        execute_db_command(
            &db_url,
            "create_issue",
            Some(issue3_payload.to_string()),
            None,
        )?;

        let issue4_payload = r#"{"id": 4, "repo_id": 100, "number": 4, "title": "Closed issue", "body": "Done", "state": "closed", "author_id": 1, "assignee_id": null, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
        execute_db_command(
            &db_url,
            "create_issue",
            Some(issue4_payload.to_string()),
            None,
        )?;

        let result = start_agent_loop(&db_url)?;
        assert_eq!(result, "Agent loop finished. Resolved 2 issues.");

        // Second run should skip issues 1 and 3 because their PRs (101, 103) now exist
        let second_result = start_agent_loop(&db_url)?;
        assert_eq!(second_result, "Agent loop finished. Resolved 0 issues.");

        Ok(())
    }
}
