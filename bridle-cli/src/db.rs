#![cfg(not(tarpaulin_include))]
//! DB action executor.

use crate::error;

/// Macro to simplify generating CRUD CLI commands that serialize and deserialize JSON logic.
macro_rules! handle_cli_crud {
    ($action:expr, $db_url:expr, $payload:expr, $id:expr, $( ($create_name:expr, $get_name:expr, $sdk_insert:path, $sdk_get:path, $model:ty) ),+ $(,)?) => {
        match $action {
            $(
                $create_name => {
                    let data = $payload.ok_or_else(|| error::CliError::Execution(format!("Missing payload for {}", $create_name)))?;
                    let parsed: $model = serde_json::from_str(&data).map_err(cli_exec_err)?;
                    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations($db_url)
                        .map_err(cli_exec_err)?;
                    $sdk_insert(&mut conn, &parsed)
                        .map_err(cli_exec_err)?;
                    Ok(format!("Successfully executed {}", $create_name))
                }
                $get_name => {
                    let target_id = $id.ok_or_else(|| error::CliError::Execution(format!("Missing id for {}", $get_name)))?;
                    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations($db_url)
                        .map_err(cli_exec_err)?;
                    let fetched = $sdk_get(&mut conn, target_id)
                        .map_err(cli_exec_err)?;
                    let json = serde_json::to_string_pretty(&fetched).map_err(cli_exec_err)?;
                    Ok(json)
                }
            )+
            _ => Err(error::CliError::Execution(format!("Unknown action: {}", $action)))
        }
    };
}

/// Executes a DB action.
fn cli_exec_err<T: std::fmt::Display>(e: T) -> error::CliError {
    error::CliError::Execution(e.to_string())
}

/// Executes a database command.
pub fn execute_db_command(
    db_url: &str,
    action: &str,
    payload: Option<String>,
    id: Option<i32>,
) -> Result<String, error::CliError> {
    handle_cli_crud!(
        action,
        db_url,
        payload,
        id,
        (
            "create_user",
            "get_user",
            bridle_sdk::db::insert_user,
            bridle_sdk::db::get_user,
            bridle_sdk::models::User
        ),
        (
            "create_org",
            "get_org",
            bridle_sdk::db::insert_organisation,
            bridle_sdk::db::get_organisation,
            bridle_sdk::models::Organisation
        ),
        (
            "create_repo",
            "get_repo",
            bridle_sdk::db::insert_repository,
            bridle_sdk::db::get_repository,
            bridle_sdk::models::Repository
        ),
        (
            "create_team",
            "get_team",
            bridle_sdk::db::insert_team,
            bridle_sdk::db::get_team,
            bridle_sdk::models::Team
        ),
        (
            "create_branch",
            "get_branch",
            bridle_sdk::db::insert_branch,
            bridle_sdk::db::get_branch,
            bridle_sdk::models::Branch
        ),
        (
            "create_branch_protection_rule",
            "get_branch_protection_rule",
            bridle_sdk::db::insert_branch_protection_rule,
            bridle_sdk::db::get_branch_protection_rule,
            bridle_sdk::models::BranchProtectionRule
        ),
        (
            "create_key",
            "get_key",
            bridle_sdk::db::insert_key,
            bridle_sdk::db::get_key,
            bridle_sdk::models::Key
        ),
        (
            "create_follow",
            "get_follow",
            bridle_sdk::db::insert_follow,
            bridle_sdk::db::get_follow,
            bridle_sdk::models::Follow
        ),
        (
            "create_star",
            "get_star",
            bridle_sdk::db::insert_star,
            bridle_sdk::db::get_star,
            bridle_sdk::models::Star
        ),
        (
            "create_org_membership",
            "get_org_membership",
            bridle_sdk::db::insert_org_membership,
            bridle_sdk::db::get_org_membership,
            bridle_sdk::models::OrgMembership
        ),
        (
            "create_repo_collaborator",
            "get_repo_collaborator",
            bridle_sdk::db::insert_repo_collaborator,
            bridle_sdk::db::get_repo_collaborator,
            bridle_sdk::models::RepoCollaborator
        ),
        (
            "create_milestone",
            "get_milestone",
            bridle_sdk::db::insert_milestone,
            bridle_sdk::db::get_milestone,
            bridle_sdk::models::Milestone
        ),
        (
            "create_label",
            "get_label",
            bridle_sdk::db::insert_label,
            bridle_sdk::db::get_label,
            bridle_sdk::models::Label
        ),
        (
            "create_issue",
            "get_issue",
            bridle_sdk::db::insert_issue,
            bridle_sdk::db::get_issue,
            bridle_sdk::models::Issue
        ),
        (
            "create_issue_label",
            "get_issue_label",
            bridle_sdk::db::insert_issue_label,
            bridle_sdk::db::get_issue_label,
            bridle_sdk::models::IssueLabel
        ),
        (
            "create_pull_request",
            "get_pull_request",
            bridle_sdk::db::insert_pull_request,
            bridle_sdk::db::get_pull_request,
            bridle_sdk::models::PullRequest
        ),
        (
            "create_pull_request_review",
            "get_pull_request_review",
            bridle_sdk::db::insert_pull_request_review,
            bridle_sdk::db::get_pull_request_review,
            bridle_sdk::models::PullRequestReview
        ),
        (
            "create_release",
            "get_release",
            bridle_sdk::db::insert_release,
            bridle_sdk::db::get_release,
            bridle_sdk::models::Release
        ),
        (
            "create_webhook",
            "get_webhook",
            bridle_sdk::db::insert_webhook,
            bridle_sdk::db::get_webhook,
            bridle_sdk::models::Webhook
        ),
        (
            "create_commit",
            "get_commit",
            bridle_sdk::db::insert_commit,
            bridle_sdk::db::get_commit,
            bridle_sdk::models::Commit
        ),
        (
            "create_tree",
            "get_tree",
            bridle_sdk::db::insert_tree,
            bridle_sdk::db::get_tree,
            bridle_sdk::models::Tree
        ),
        (
            "create_blob",
            "get_blob",
            bridle_sdk::db::insert_blob,
            bridle_sdk::db::get_blob,
            bridle_sdk::models::Blob
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_db_command_unknown() {
        let res = execute_db_command("bridle.db", "unknown", None, None);
        assert!(res.is_err());
    }
}
#[test]
fn test_execute_db_command_missing_payload() {
    let res = execute_db_command("bridle.db", "create_key", None, None);
    if let Err(e) = res {
        assert!(e.to_string().contains("Missing payload"));
    } else {
        panic!("Expected error");
    }
}

#[test]
fn test_execute_db_command_bad_json() {
    let res = execute_db_command(
        "bridle.db",
        "create_key",
        Some("{ bad json".to_string()),
        None,
    );
    assert!(res.is_err());
}

#[test]
fn test_execute_db_command_missing_id() {
    let res = execute_db_command("bridle.db", "get_key", None, None);
    if let Err(e) = res {
        assert!(e.to_string().contains("Missing id"));
    } else {
        panic!("Expected error");
    }
}

#[test]
fn test_all_db_commands() {
    let commands = vec![
        "create_user",
        "get_user",
        "create_org",
        "get_org",
        "create_repo",
        "get_repo",
        "create_team",
        "get_team",
        "create_branch",
        "get_branch",
        "create_branch_protection_rule",
        "get_branch_protection_rule",
        "create_key",
        "get_key",
        "create_follow",
        "get_follow",
        "create_star",
        "get_star",
        "create_org_membership",
        "get_org_membership",
        "create_repo_collaborator",
        "get_repo_collaborator",
        "create_milestone",
        "get_milestone",
        "create_label",
        "get_label",
        "create_issue",
        "get_issue",
        "create_issue_label",
        "get_issue_label",
        "create_pull_request",
        "get_pull_request",
        "create_pull_request_review",
        "get_pull_request_review",
        "create_release",
        "get_release",
        "create_webhook",
        "get_webhook",
        "create_commit",
        "get_commit",
        "create_tree",
        "get_tree",
        "create_blob",
        "get_blob",
    ];
    for action in commands {
        let res = execute_db_command("bridle.db", action, None, None);
        assert!(res.is_err());
    }
}

#[test]
fn test_all_db_commands_connection_error() {
    let bad_db = "invalid_protocol://localhost";

    // To hit the inner connection code for get_*, we just need an ID
    let gets = vec![
        "get_user",
        "get_org",
        "get_repo",
        "get_team",
        "get_branch",
        "get_branch_protection_rule",
        "get_key",
        "get_follow",
        "get_star",
        "get_org_membership",
        "get_repo_collaborator",
        "get_milestone",
        "get_label",
        "get_issue",
        "get_issue_label",
        "get_pull_request",
        "get_pull_request_review",
        "get_release",
        "get_webhook",
        "get_commit",
        "get_tree",
        "get_blob",
    ];
    for action in gets {
        let res = execute_db_command(bad_db, action, None, Some(1));
        assert!(res.is_err());
    }
}

#[test]
fn test_all_db_commands_create_connection_error() {
    let bad_db = "invalid_protocol://localhost";

    let res = execute_db_command(bad_db, "create_user", Some(r#"{"id": 1, "username": "a", "email": "a", "password_hash": "a", "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_org", Some(r#"{"id": 1, "name": "a", "billing_plan": "a", "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_repo", Some(r#"{"id": 1, "owner_id": 1, "owner_type": "a", "name": "a", "is_private": false, "is_fork": false, "archived": false, "allow_merge_commit": false, "allow_squash_merge": false, "allow_rebase_merge": false, "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_team", Some(r#"{"id": 1, "org_id": 1, "name": "a", "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_branch", Some(r#"{"id": 1, "repo_id": 1, "name": "a", "head_sha": "a", "is_protected": false, "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_branch_protection_rule", Some(r#"{"id": 1, "branch_id": 1, "required_pr_reviews": 0, "require_code_owner_reviews": false, "require_signed_commits": false, "enforce_admins": false, "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_key", Some(r#"{"id": 1, "user_id": 1, "key_type": "a", "title": "a", "key_data": "a", "fingerprint": "a", "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_follow", Some(r#"{"id": 1, "follower_id": 1, "following_id": 1, "created_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(
        bad_db,
        "create_star",
        Some(
            r#"{"id": 1, "user_id": 1, "repo_id": 1, "created_at": "2026-04-01T00:00:00"}"#
                .to_string(),
        ),
        None,
    );
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_org_membership", Some(r#"{"id": 1, "org_id": 1, "user_id": 1, "role": "a", "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_repo_collaborator", Some(r#"{"id": 1, "repo_id": 1, "user_id": 1, "permission_level": "a", "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_milestone", Some(r#"{"id": 1, "repo_id": 1, "title": "a", "state": "a", "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_label", Some(r#"{"id": 1, "repo_id": 1, "name": "a", "color": "a", "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_issue", Some(r#"{"id": 1, "repo_id": 1, "number": 1, "title": "a", "state": "a", "author_id": 1, "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(
        bad_db,
        "create_issue_label",
        Some(r#"{"id": 1, "issue_id": 1, "label_id": 1}"#.to_string()),
        None,
    );
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_pull_request", Some(r#"{"id": 1, "repo_id": 1, "number": 1, "title": "a", "state": "a", "head_branch": "a", "base_branch": "a", "author_id": 1, "is_draft": false, "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_pull_request_review", Some(r#"{"id": 1, "pr_id": 1, "user_id": 1, "state": "a", "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_release", Some(r#"{"id": 1, "repo_id": 1, "tag_name": "a", "target_commitish": "a", "is_draft": false, "is_prerelease": false, "author_id": 1, "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_webhook", Some(r#"{"id": 1, "repo_id": 1, "url": "a", "content_type": "a", "events": "a", "is_active": false, "created_at": "2026-04-01T00:00:00", "updated_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_commit", Some(r#"{"id": 1, "repo_id": 1, "sha": "a", "tree_sha": "a", "parent_shas": "a", "message": "a", "author_name": "a", "author_email": "a", "committer_name": "a", "committer_email": "a", "author_date": "2026-04-01T00:00:00", "committer_date": "2026-04-01T00:00:00", "created_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_tree", Some(r#"{"id": 1, "repo_id": 1, "sha": "a", "entries": "a", "created_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());

    let res = execute_db_command(bad_db, "create_blob", Some(r#"{"id": 1, "repo_id": 1, "sha": "a", "size": 1, "created_at": "2026-04-01T00:00:00"}"#.to_string()), None);
    assert!(res.is_err());
}
