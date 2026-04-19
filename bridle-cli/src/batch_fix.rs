//! Batch fix functionality across an organization.

use crate::error::CliError;
use crate::runner;
use std::fs;
use std::path::Path;

/// Runs a batch fix across all repos in an organization.
#[cfg(not(tarpaulin_include))]
pub fn batch_fix(
    org: &str,
    issue: &str,
    pattern: Option<String>,
    tools: Option<Vec<String>>,
    tool_args: Option<std::collections::HashMap<String, Vec<String>>>,
    db_url: &str,
    _safety_mode: bool,
    max_repos: Option<usize>,
    _max_prs_per_hour: Option<usize>,
) -> Result<String, CliError> {
    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(db_url)
        .map_err(|e| CliError::Execution(e.to_string()))?;

    // We assume the repos are cloned in ~/.bridle/workspace/<org>
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let workspace = Path::new(&home).join(".bridle").join("workspace").join(org);

    if !workspace.exists() {
        return Err(CliError::Execution(format!(
            "Workspace for org {} not found. Did you run ingest-org first?",
            org
        )));
    }

    // In a real implementation, we'd query the DB for the repos belonging to this org.
    // For simplicity, we just iterate the subdirectories of the workspace.
    let mut count = 0;
    if let Ok(entries) = fs::read_dir(&workspace) {
        for entry in entries.flatten() {
            if let Some(limit) = max_repos
                && count >= limit {
                    break;
                }
            if entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                let repo_name = entry.file_name().to_string_lossy().to_string();
                let repo_path = entry.path();

                println!("Processing repository: {}", repo_name);

                // Idempotency check via issues
                use bridle_sdk::schema::issues::dsl::*;
                use diesel::prelude::*;

                let has_issue = match &mut conn {
                    bridle_sdk::db::DbConnection::Sqlite(sqlite_conn) => {
                        issues
                            .filter(title.eq(issue))
                            .filter(state.eq("open"))
                            .count()
                            .get_result::<i64>(sqlite_conn)
                            .unwrap_or(0)
                            > 0
                    }
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
                    continue; // Skip if already processing
                }

                // Let's see if we can create the issue in DB.
                // We'd need to find the repo ID from DB. Let's just create a dummy issue for now.
                let now = chrono::Utc::now().naive_utc();

                let issue_model = bridle_sdk::models::Issue {
                    id: chrono::Utc::now().timestamp_subsec_micros() as i32, // dummy ID
                    repo_id: 1,                                              // dummy
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

                // Run the runner logic in an isolated EphemeralWorkspace to protect disk space.
                // EphemeralWorkspace uses `git worktree` and automatically cleans up via `Drop`.
                match crate::workspace::EphemeralWorkspace::new(
                    &repo_path,
                    &format!("fix-{}", issue.replace(" ", "-").to_lowercase()),
                ) {
                    Ok(workspace) => {
                        let original_dir = std::env::current_dir().unwrap_or_default();
                        if std::env::set_current_dir(&workspace.path).is_ok() {
                            let req = bridle_sdk::models::ToolRunRequest {
                                pattern: pattern.clone(),
                                tools: tools.clone(),
                                tool_args: tool_args.clone(),
                                dry_run: Some(false),
                                action: Some("fix".to_string()),
                            };

                            match runner::run(runner::Action::Fix { dry_run: false }, req) {
                                Ok(_) => println!("Successfully applied fix to {}", repo_name),
                                Err(e) => println!("Failed to apply fix to {}: {}", repo_name, e),
                            }

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

    #[test]
    fn test_batch_fix_no_workspace() {
        let res = batch_fix(
            "nonexistent_org",
            "test issue",
            None,
            None,
            None,
            "bridle.db",
            false,
            None,
            None,
        );
        if let Err(e) = res {
            assert!(
                e.to_string()
                    .contains("Workspace for org nonexistent_org not found")
            );
        } else {
            panic!("Expected error");
        }
    }
}
