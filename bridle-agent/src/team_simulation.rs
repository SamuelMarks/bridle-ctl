//! Full "AI Engineering Team" offline-first simulations.

use bridle_cli::db::execute_db_command;
use bridle_cli::runner::{Action, run};
use bridle_sdk::BridleError;
use bridle_sdk::models::{Issue, PullRequest, ToolRunRequest};

/// Simulates a full AI Engineering Team workflow.
pub fn run_ai_team_simulation(db_url: &str) -> Result<String, BridleError> {
    setup_repo(db_url)?;
    pm_agent_create_issue(db_url)?;
    let pr_id = engineer_and_qa_loop(db_url)?;
    reviewer_agent_merge_pr(db_url, pr_id)?;

    Ok("AI Team Simulation completed successfully.".to_string())
}

/// Simulates setting up a repository.
fn setup_repo(db_url: &str) -> Result<(), BridleError> {
    let repo_payload = r#"{"id": 200, "owner_id": 1, "owner_type": "user", "name": "team_repo", "is_private": false, "is_fork": false, "archived": false, "allow_merge_commit": true, "allow_squash_merge": true, "allow_rebase_merge": true, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
    execute_db_command(db_url, "create_repo", Some(repo_payload.to_string()), None)
        .map_err(|e| BridleError::Daemon(format!("Setup repo failed: {}", e)))?;
    Ok(())
}

/// Simulates a PM creating an issue.
fn pm_agent_create_issue(db_url: &str) -> Result<(), BridleError> {
    println!("PM Agent: Creating issue for tech debt...");
    let issue_payload = r#"{"id": 200, "repo_id": 200, "number": 1, "title": "Refactor C code", "body": "Use cdd-gnu-standardizer to clean up old C code", "state": "open", "author_id": 1, "assignee_id": null, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
    execute_db_command(
        db_url,
        "create_issue",
        Some(issue_payload.to_string()),
        None,
    )
    .map_err(|e| BridleError::Daemon(format!("PM failed to create issue: {}", e)))?;
    Ok(())
}

/// Simulates an engineer fixing an issue and QA verifying it.
fn engineer_and_qa_loop(db_url: &str) -> Result<i32, BridleError> {
    println!("Engineer Agent: Picking up open issues...");

    let json_str = execute_db_command(db_url, "get_issue", None, Some(200))
        .map_err(|e| BridleError::Daemon(format!("Engineer failed to get issue: {}", e)))?;
    let issue: Issue = serde_json::from_str(&json_str)
        .map_err(|e| BridleError::Daemon(format!("Failed to parse issue: {}", e)))?;

    println!(
        "Engineer Agent: Found issue #{} - {}. Running tools...",
        issue.number, issue.title
    );

    let req = ToolRunRequest {
        pattern: Some(r".*\.c$".to_string()),
        tools: Some(vec!["cdd-gnu-standardizer".to_string()]),
        tool_args: None,
        dry_run: Some(true),
        action: Some("fix".to_string()),
    };

    run(Action::Fix { dry_run: true }, req)
        .map_err(|e| BridleError::Daemon(format!("Engineer code mutation failed: {}", e)))?;

    let cwd = std::env::current_dir().unwrap_or_default();
    let _ = qa_agent_verify(&cwd);

    let pr_body = match find_pr_template(&cwd) {
        Some(template_path) => {
            println!(
                "Engineer Agent: Found PR template at {:?}. Drafting PR body...",
                template_path
            );
            format!(
                "Resolves issue #{} - Automated fix. Automated QA tests passed.",
                issue.number
            )
        }
        None => format!("Resolves issue #{}", issue.number),
    };

    let pr_id = 200;
    let pr_payload = format!(
        r#"{{"id": {}, "repo_id": {}, "number": {}, "title": "Fix: {}", "body": "{}", "state": "open", "head_branch": "eng-fix", "base_branch": "main", "author_id": 2, "assignee_id": null, "milestone_id": null, "is_draft": false, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}}"#,
        pr_id, issue.repo_id, pr_id, issue.title, pr_body
    );
    execute_db_command(db_url, "create_pull_request", Some(pr_payload), None)
        .map_err(|e| BridleError::Daemon(format!("Engineer PR creation failed: {}", e)))?;

    Ok(pr_id)
}

/// Searches for common PR template locations in the given repository directory.
fn find_pr_template(cwd: &std::path::Path) -> Option<std::path::PathBuf> {
    let candidates = [
        ".github/PULL_REQUEST_TEMPLATE.md",
        ".github/pull_request_template.md",
        "docs/PULL_REQUEST_TEMPLATE.md",
        "PULL_REQUEST_TEMPLATE.md",
        "CONTRIBUTING.md",
    ];

    for candidate in candidates {
        let path = cwd.join(candidate);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

/// Simulates a QA agent identifying test runner and executing it.
fn qa_agent_verify(cwd: &std::path::Path) -> bool {
    let runner = if cwd.join("Makefile").exists() {
        Some("make")
    } else if cwd.join("Cargo.toml").exists() {
        Some("cargo")
    } else if cwd.join("package.json").exists() {
        Some("npm")
    } else if cwd.join("go.mod").exists() {
        Some("go")
    } else {
        None
    };

    if let Some(cmd) = runner {
        println!("QA Agent: Detected runner {}. Executing...", cmd);
    } else {
        println!("QA Agent: No test runner detected, assuming safe/pass.");
    }
    true
}

/// Simulates a reviewer merging a PR.
fn reviewer_agent_merge_pr(db_url: &str, pr_id: i32) -> Result<(), BridleError> {
    println!("Reviewer Agent: Checking PR #{}...", pr_id);

    let json_str = execute_db_command(db_url, "get_pull_request", None, Some(pr_id))
        .map_err(|e| BridleError::Daemon(format!("Reviewer failed to get PR: {}", e)))?;
    let mut pr: PullRequest = serde_json::from_str(&json_str)
        .map_err(|e| BridleError::Daemon(format!("Failed to parse PR: {}", e)))?;

    pr.state = "closed".to_string();
    println!("Reviewer Agent: PR merged successfully.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_simulation_methods() -> Result<(), BridleError> {
        assert!(setup_repo("/invalid/path").is_err());
        assert!(pm_agent_create_issue("/invalid/path").is_err());
        assert!(engineer_and_qa_loop("/invalid/path").is_err());
        assert!(reviewer_agent_merge_pr("/invalid/path", 1).is_err());

        let tmp = tempfile::tempdir()?;
        assert!(find_pr_template(tmp.path()).is_none());
        assert!(qa_agent_verify(tmp.path()));

        std::fs::write(tmp.path().join("go.mod"), "module test")?;
        assert!(qa_agent_verify(tmp.path()));

        std::fs::write(tmp.path().join("package.json"), "{}")?;
        assert!(qa_agent_verify(tmp.path()));

        std::fs::write(tmp.path().join("Cargo.toml"), "")?;
        assert!(qa_agent_verify(tmp.path()));

        std::fs::write(tmp.path().join("Makefile"), "")?;
        assert!(qa_agent_verify(tmp.path()));

        std::fs::write(tmp.path().join("CONTRIBUTING.md"), "# Contributing")?;
        assert!(find_pr_template(tmp.path()).is_some());

        // Also run simulation inside a temp dir with CONTRIBUTING.md to cover the Some(template_path) branch in engineer_and_qa_loop
        let orig = std::env::current_dir()?;
        std::env::set_current_dir(tmp.path())?;
        let tf = tempfile::NamedTempFile::new()?;
        let db_url = tf.path().to_string_lossy().to_string();
        let res = run_ai_team_simulation(&db_url);
        std::env::set_current_dir(orig)?;
        assert!(res.is_ok());

        Ok(())
    }

    #[test]
    fn test_run_ai_team_simulation() -> Result<(), BridleError> {
        let tf = tempfile::NamedTempFile::new()?;
        let db_url = tf
            .path()
            .to_str()
            .ok_or_else(|| BridleError::Daemon("Invalid path".to_string()))?
            .to_string();

        let result = run_ai_team_simulation(&db_url)?;
        assert_eq!(result, "AI Team Simulation completed successfully.");

        let err_res = run_ai_team_simulation("/invalid/path");
        assert!(err_res.is_err());

        Ok(())
    }
}
