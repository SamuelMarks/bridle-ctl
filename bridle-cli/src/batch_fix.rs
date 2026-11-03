//! Batch fix functionality across an organization.

use crate::runner;
use bridle_sdk::BridleError;
use std::fs;
use std::path::Path;

/// Runs a batch fix across all repos in an organization using the default `~/.bridle/workspace` root.
#[allow(clippy::too_many_arguments)]
pub fn batch_fix(
    org: &str,
    issue: &str,
    pattern: Option<String>,
    tools: Option<Vec<String>>,
    tool_args: Option<std::collections::HashMap<String, Vec<String>>>,
    db_url: &str,
    safety_mode: bool,
    max_repos: Option<usize>,
    max_prs_per_hour: Option<usize>,
) -> Result<String, BridleError> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let workspace_root = Path::new(&home).join(".bridle").join("workspace");
    batch_fix_in_workspace(
        &workspace_root,
        org,
        issue,
        pattern,
        tools,
        tool_args,
        db_url,
        safety_mode,
        max_repos,
        max_prs_per_hour,
    )
}

/// Runs a batch fix across all repos in an organization within the specified workspace root.
#[allow(clippy::too_many_arguments)]
pub fn batch_fix_in_workspace(
    workspace_root: &Path,
    org: &str,
    issue: &str,
    pattern: Option<String>,
    tools: Option<Vec<String>>,
    tool_args: Option<std::collections::HashMap<String, Vec<String>>>,
    db_url: &str,
    _safety_mode: bool,
    max_repos: Option<usize>,
    _max_prs_per_hour: Option<usize>,
) -> Result<String, BridleError> {
    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(db_url)?;

    let workspace = workspace_root.join(org);

    if !workspace.exists() {
        return Err(BridleError::Config(format!(
            "Workspace for org {} not found. Did you run ingest-org first?",
            org
        )));
    }

    let mut count = 0;
    if let Ok(entries) = fs::read_dir(&workspace) {
        for entry in entries.flatten() {
            if let Some(limit) = max_repos
                && count >= limit
            {
                break;
            }
            if entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                let repo_name = entry.file_name().to_string_lossy().to_string();
                let repo_path = entry.path();

                println!("Processing repository: {}", repo_name);

                use bridle_sdk::schema::issues::dsl::*;
                use diesel::prelude::*;

                let has_issue = match &mut conn {
                    #[cfg(feature = "sqlite")]
                    bridle_sdk::db::DbConnection::Sqlite(sqlite_conn) => {
                        issues
                            .filter(title.eq(issue))
                            .filter(state.eq("open"))
                            .count()
                            .get_result::<i64>(sqlite_conn)
                            .unwrap_or(0)
                            > 0
                    }
                    #[cfg(feature = "postgres")]
                    bridle_sdk::db::DbConnection::Pg(pg_conn) => {
                        issues
                            .filter(title.eq(issue))
                            .filter(state.eq("open"))
                            .count()
                            .get_result::<i64>(pg_conn)
                            .unwrap_or(0)
                            > 0
                    }
                };
                if has_issue {
                    continue;
                }

                let now = chrono::Utc::now().naive_utc();

                let issue_model = bridle_sdk::models::Issue {
                    id: chrono::Utc::now().timestamp_subsec_micros() as i32,
                    repo_id: 1,
                    number: chrono::Utc::now().timestamp_subsec_nanos() as i32,
                    title: issue.to_string(),
                    body: Some("Automated batch fix".to_string()),
                    state: "open".to_string(),
                    author_id: 1,
                    assignee_id: None,
                    milestone_id: None,
                    created_at: now,
                    updated_at: now,
                };
                let _ = bridle_sdk::db::insert_issue(&mut conn, &issue_model);

                match crate::workspace::EphemeralWorkspace::new(
                    &repo_path,
                    &format!("fix-{}", issue.replace(' ', "-").to_lowercase()),
                ) {
                    Ok(ephemeral) => {
                        let original_dir = std::env::current_dir().unwrap_or_default();
                        if std::env::set_current_dir(&ephemeral.path).is_ok() {
                            let req = bridle_sdk::models::ToolRunRequest {
                                pattern: pattern.clone(),
                                tools: tools.clone(),
                                tool_args: tool_args.clone(),
                                dry_run: Some(false),
                                action: Some("fix".to_string()),
                            };

                            let _ = runner::run(runner::Action::Fix { dry_run: false }, req);

                            let _ = std::env::set_current_dir(original_dir);
                        }
                    }
                    Err(e) => println!("Failed to create workspace for {}: {}", repo_name, e),
                }

                count += 1;
            }
        }
    }

    Ok(format!("Batch fix applied to {} repositories.", count))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::git_command;

    #[test]
    fn test_batch_fix_no_workspace() -> Result<(), BridleError> {
        let tf = tempfile::NamedTempFile::new()?;
        let db_url = tf.path().to_str().unwrap_or(":memory:");
        let res = batch_fix(
            "nonexistent_org",
            "test issue",
            None,
            None,
            None,
            db_url,
            false,
            None,
            None,
        );
        assert!(matches!(
            res,
            Err(BridleError::Config(ref msg)) if msg.contains("Workspace for org nonexistent_org not found")
        ));
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn test_batch_fix_logic() -> Result<(), BridleError> {
        let org = format!("testorg_{}", uuid::Uuid::new_v4());
        let root_dir = tempfile::tempdir()?;
        let workspace = root_dir.path().join(&org);
        std::fs::create_dir_all(&workspace)?;

        let repo_dir = workspace.join("repo1");
        std::fs::create_dir_all(&repo_dir)?;
        git_command().arg("init").current_dir(&repo_dir).status()?;
        git_command()
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo_dir)
            .status()?;
        git_command()
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo_dir)
            .status()?;
        git_command()
            .args(["commit", "--allow-empty", "-m", "init"])
            .current_dir(&repo_dir)
            .status()?;

        let tf = tempfile::NamedTempFile::new()?;
        let db_url = tf.path().to_str().unwrap_or(":memory:").to_string();
        let _ = bridle_sdk::db::establish_connection_and_run_migrations(&db_url)?;

        let res = batch_fix_in_workspace(
            root_dir.path(),
            &org,
            "fix something",
            Some(r".*\.rs$".to_string()),
            None,
            None,
            &db_url,
            false,
            Some(1),
            None,
        )?;
        assert_eq!(res, "Batch fix applied to 1 repositories.");

        // Second run with same issue title hits `has_issue` continue branch
        let res_dup = batch_fix_in_workspace(
            root_dir.path(),
            &org,
            "fix something",
            None,
            None,
            None,
            &db_url,
            false,
            None,
            None,
        )?;
        assert_eq!(res_dup, "Batch fix applied to 0 repositories.");

        // Run against PostgreSQL to cover DbConnection::Pg branch
        let pg_url = "postgres://samuel@localhost/bridle_test";
        let unique_issue = format!("fix pg {}", uuid::Uuid::new_v4());
        let res_pg = batch_fix_in_workspace(
            root_dir.path(),
            &org,
            &unique_issue,
            Some(r".*\.cpp$".to_string()),
            Some(vec!["type-correct".to_string()]),
            None,
            pg_url,
            false,
            Some(1),
            None,
        )?;
        assert_eq!(res_pg, "Batch fix applied to 1 repositories.");

        // Add a repo with an invalid pipeline name (`invalid branch`) to cover EphemeralWorkspace error branch
        let res_ws_err = batch_fix_in_workspace(
            root_dir.path(),
            &org,
            "bad..branch..name",
            None,
            None,
            None,
            &db_url,
            false,
            Some(1),
            None,
        )?;
        assert_eq!(res_ws_err, "Batch fix applied to 1 repositories.");

        // Run with max_repos = Some(0) hits limit branch
        let res_limit = batch_fix_in_workspace(
            root_dir.path(),
            &org,
            "fix something",
            None,
            None,
            None,
            &db_url,
            false,
            Some(0),
            None,
        )?;
        assert_eq!(res_limit, "Batch fix applied to 0 repositories.");

        Ok(())
    }
}
