#![cfg(not(tarpaulin_include))]
//! Full "AI Engineering Team" offline-first simulations.

use crate::error::AgentError;
use bridle_cli::db::execute_db_command;
use bridle_cli::runner::{Action, run};
use bridle_sdk::models::{Issue, PullRequest, ToolRunRequest};

/// Simulates a full AI Engineering Team workflow.
///
/// Roles simulated:
/// - Product Manager: Creates issues based on backlog.
/// - Engineer: Picks up issues, mutates code natively (via tools), opens a Pull Request.
/// - QA: Verifies the code with automated tests before the PR is created.
/// - Reviewer: Reviews the Pull Request and merges it natively in the DB.
pub fn run_ai_team_simulation(db_url: &str) -> Result<String, AgentError> {
    // 0. Setup mock Repository
    setup_repo(db_url)?;

    // 1. Product Manager Agent
    pm_agent_create_issue(db_url)?;

    // 2. Engineer & QA Agent Loop
    let pr_id = engineer_and_qa_loop(db_url)?;

    // 3. Reviewer Agent
    reviewer_agent_merge_pr(db_url, pr_id)?;

    Ok("AI Team Simulation completed successfully.".to_string())
}

/// Simulates setting up a repository.
fn setup_repo(db_url: &str) -> Result<(), AgentError> {
    let repo_payload = r#"{"id": 200, "owner_id": 1, "owner_type": "user", "name": "team_repo", "is_private": false, "is_fork": false, "archived": false, "allow_merge_commit": true, "allow_squash_merge": true, "allow_rebase_merge": true, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
    execute_db_command(db_url, "create_repo", Some(repo_payload.to_string()), None)
        .map_err(|e| AgentError::Daemon(format!("Setup repo failed: {}", e)))?;
    Ok(())
}

/// Simulates a PM creating an issue.
fn pm_agent_create_issue(db_url: &str) -> Result<(), AgentError> {
    println!("PM Agent: Creating issue for tech debt...");
    let issue_payload = r#"{"id": 200, "repo_id": 200, "number": 1, "title": "Refactor C code", "body": "Use cdd-gnu-standardizer to clean up old C code", "state": "open", "author_id": 1, "assignee_id": null, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}"#;
    execute_db_command(
        db_url,
        "create_issue",
        Some(issue_payload.to_string()),
        None,
    )
    .map_err(|e| AgentError::Daemon(format!("PM failed to create issue: {}", e)))?;
    Ok(())
}

/// Simulates an engineer fixing an issue and QA verifying it, looping if needed.
fn engineer_and_qa_loop(db_url: &str) -> Result<i32, AgentError> {
    println!("Engineer Agent: Picking up open issues...");

    // Simulate fetching the issue we just made
    let json_str = execute_db_command(db_url, "get_issue", None, Some(200))
        .map_err(|e| AgentError::Daemon(format!("Engineer failed to get issue: {}", e)))?;
    let issue: Issue = serde_json::from_str(&json_str)
        .map_err(|e| AgentError::Daemon(format!("Failed to parse issue: {}", e)))?;

    println!(
        "Engineer Agent: Found issue #{} - {}. Running tools...",
        issue.number, issue.title
    );

    let max_attempts = 3;
    let mut success = false;

    for attempt in 1..=max_attempts {
        println!("Engineer Agent: Attempt {} to fix...", attempt);
        // Hardcoded tool match for simulation purposes
        let req = ToolRunRequest {
            pattern: Some(".*\\.c$".to_string()),
            tools: Some(vec!["cdd-gnu-standardizer".to_string()]),
            tool_args: None,
            dry_run: Some(true), // We use dry_run for simulation so it doesn't change real files if run here
            action: Some("fix".to_string()),
        };

        run(Action::Fix { dry_run: true }, req)
            .map_err(|e| AgentError::Daemon(format!("Engineer code mutation failed: {}", e)))?;

        println!("QA Agent: Verifying codebase...");
        if qa_agent_verify() {
            println!("QA Agent: Tests passed!");
            success = true;
            break;
        } else {
            println!("QA Agent: Tests failed. Sending feedback to Engineer.");
        }
    }

    if !success {
        println!(
            "Engineer Agent failed to fix after {} attempts. Halting for human-in-the-loop intervention (Semi-Autonomous mode).",
            max_attempts
        );
        // Git rollback hatch
        let _ = std::process::Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .args(["reset", "--hard"])
            .status();
        let _ = std::process::Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .args(["clean", "-fd"])
            .status();

        return Err(AgentError::Daemon(
            "Semi-Autonomous mode: human intervention required".to_string(),
        ));
    }

    println!("Engineer Agent: Code fixed and verified. Proposing Pull Request...");

    // Template-Aware PR Drafting
    let pr_body = match find_pr_template() {
        Some(template_path) => {
            println!(
                "Engineer Agent: Found PR template at {:?}. Drafting PR body...",
                template_path
            );
            // Simulate using LLM to map changes to the template structure
            format!(
                "Resolves issue #{}\n\n## Motivation\nAutomated fix.\n\n## Testing Done\nAutomated QA tests passed.",
                issue.number
            )
        }
        None => {
            format!("Resolves issue #{}", issue.number)
        }
    };

    let pr_id = 200;
    let pr_payload = format!(
        r#"{{"id": {}, "repo_id": {}, "number": {}, "title": "Fix: {}", "body": "{}", "state": "open", "head_branch": "eng-fix", "base_branch": "main", "author_id": 2, "assignee_id": null, "milestone_id": null, "is_draft": false, "created_at": "2026-04-08T00:00:00", "updated_at": "2026-04-08T00:00:00"}}"#,
        pr_id,
        issue.repo_id,
        pr_id,
        issue.title,
        pr_body.replace("\n", "\\n")
    );
    execute_db_command(db_url, "create_pull_request", Some(pr_payload), None)
        .map_err(|e| AgentError::Daemon(format!("Engineer PR creation failed: {}", e)))?;

    Ok(pr_id)
}

/// Searches for common PR template locations in the current repository.
fn find_pr_template() -> Option<std::path::PathBuf> {
    let cwd = std::env::current_dir().unwrap_or_default();
    let candidates = vec![
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
fn qa_agent_verify() -> bool {
    let cwd = std::env::current_dir().unwrap_or_default();

    // Identify runner
    let runner = if cwd.join("Makefile").exists() {
        Some(("make", vec!["test"]))
    } else if cwd.join("Cargo.toml").exists() {
        Some(("cargo", vec!["test"]))
    } else if cwd.join("package.json").exists() {
        Some(("npm", vec!["test"]))
    } else if cwd.join("go.mod").exists() {
        Some(("go", vec!["test", "./..."]))
    } else {
        None
    };

    if let Some((cmd, _args)) = runner {
        println!("QA Agent: Detected runner {}. Executing...", cmd);
        // For simulation, we'll pretend we're running it but just return true.
        // We'll run it in dry-run/dummy mode to avoid actually running full tests during a simple run.
        // let status = std::process::Command::new(cmd).args(args).status();
        // status.map(|s| s.success()).unwrap_or(false)
        true
    } else {
        println!("QA Agent: No test runner detected, assuming safe/pass.");
        true
    }
}

/// Simulates a reviewer merging a PR.
fn reviewer_agent_merge_pr(db_url: &str, pr_id: i32) -> Result<(), AgentError> {
    println!("Reviewer Agent: Checking PR #{}...", pr_id);

    let json_str = execute_db_command(db_url, "get_pull_request", None, Some(pr_id))
        .map_err(|e| AgentError::Daemon(format!("Reviewer failed to get PR: {}", e)))?;
    let mut pr: PullRequest = serde_json::from_str(&json_str)
        .map_err(|e| AgentError::Daemon(format!("Failed to parse PR: {}", e)))?;

    println!("Reviewer Agent: PR looks good. Merging (updating DB state)...");

    // In a real scenario, we'd have an update_pull_request API. For simulation,
    // we'll simulate the merge log and close the state conceptually via logging,
    // or simulate an update if the SDK supports it. Since `bridle_sdk` currently
    // focuses on `insert` and `get`, we'll just log the approval and consider it merged natively.
    pr.state = "closed".to_string(); // Simulated state transition
    println!("Reviewer Agent: PR merged successfully.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_ai_team_simulation() -> Result<(), AgentError> {
        let tf = tempfile::NamedTempFile::new().map_err(|e| AgentError::Daemon(e.to_string()))?;
        let db_url = tf
            .path()
            .to_str()
            .ok_or(AgentError::Daemon("Invalid path".to_string()))?
            .to_string();

        let result = run_ai_team_simulation(&db_url)?;
        assert_eq!(result, "AI Team Simulation completed successfully.");

        // Test error
        let err_res = run_ai_team_simulation("/invalid/path");
        assert!(err_res.is_err());

        Ok(())
    }
}
