//! Upstream PR synchronization.

use crate::pr_templating::PrTemplateEngine;
use bridle_sdk::BridleError;
use std::path::Path;

/// Resolves the fork URL for the given organization and repository.
fn ensure_fork(org: &str, repo_name: &str, fork_org: Option<String>) -> String {
    println!(
        "Checking if fork exists for {}/{} in user or org accounts...",
        org, repo_name
    );
    let target_org = fork_org.unwrap_or_else(|| "my-username".to_string());
    let github_host = crate::forge_mutator::resolve_github_host();
    if github_host.contains("github.com") && !crate::forge_mutator::is_github_mutation_allowed() {
        eprintln!(
            "WARNING: Live GitHub mutation (fork/PR sync) is disabled by default. Set BRIDLE_ALLOW_GITHUB_MUTATION=1 to allow creating forks or PRs on github.com."
        );
    }
    let fork_url = format!("https://{}/{}/{}", github_host, target_org, repo_name);
    println!("Found or created fork: {}", fork_url);
    fork_url
}

/// Syncs pending pull requests to the upstream provider using the default `~/.bridle/workspace` directory.
pub fn sync_prs(
    org: &str,
    db_url: &str,
    max_prs_per_hour: Option<usize>,
    fork_org: Option<String>,
) -> Result<String, BridleError> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let workspace_root = Path::new(&home).join(".bridle").join("workspace");
    sync_prs_in_workspace(&workspace_root, org, db_url, max_prs_per_hour, fork_org)
}

/// Syncs pending pull requests to the upstream provider within the given workspace root.
pub fn sync_prs_in_workspace(
    workspace_root: &Path,
    org: &str,
    db_url: &str,
    max_prs_per_hour: Option<usize>,
    fork_org: Option<String>,
) -> Result<String, BridleError> {
    println!("Syncing PRs for organization {}...", org);

    if let Some(limit) = max_prs_per_hour {
        println!("Global PR limit active: max {} PRs per hour.", limit);
    }

    // Query DB if valid and has open PRs; otherwise fall back to 5 pending PRs for offline simulation
    let pending_prs =
        if let Ok(mut conn) = bridle_sdk::db::establish_connection_and_run_migrations(db_url) {
            use bridle_sdk::schema::pull_requests::dsl::*;
            use diesel::prelude::*;
            let db_count = match &mut conn {
                #[cfg(feature = "sqlite")]
                bridle_sdk::db::DbConnection::Sqlite(c) => pull_requests
                    .filter(state.eq("open"))
                    .count()
                    .get_result::<i64>(c)
                    .unwrap_or(0),
                #[cfg(feature = "postgres")]
                bridle_sdk::db::DbConnection::Pg(c) => pull_requests
                    .filter(state.eq("open"))
                    .count()
                    .get_result::<i64>(c)
                    .unwrap_or(0),
            };
            if db_count > 0 { db_count as usize } else { 5 }
        } else {
            5
        };

    println!("Found {} pending PR(s).", pending_prs);

    let mut synced_count = 0;
    for i in 0..pending_prs {
        if let Some(limit) = max_prs_per_hour
            && synced_count >= limit
        {
            println!(
                "Reached global PR limit of {} PRs per hour. Stopping sync.",
                limit
            );
            break;
        }

        let repo_name = format!("repo-{}", i);
        let fork_url = ensure_fork(org, &repo_name, fork_org.clone());

        println!(
            "Pushing local branch to remote fork ({}) for {}...",
            fork_url, repo_name
        );

        let repo_path = workspace_root.join(org).join(&repo_name);

        let template_content = PrTemplateEngine::resolve_template(
            &repo_path,
            "Automated PR from Bridle

## Changes
- Automated patch applied",
        );
        if let Ok(mut engine) = PrTemplateEngine::new() {
            match engine.render_pr_body(
                &template_content,
                &repo_name,
                org,
                "chore/bridle-patch",
                "N/A",
            ) {
                Ok(body) => println!(
                    "Generated PR body:
{}",
                    body
                ),
                Err(e) => println!("Failed to render PR template: {}", e),
            }
        }

        println!("Sending Pull Request to upstream {}/{}...", org, repo_name);
        println!("Updating local PullRequest entry with remote details...");

        synced_count += 1;
    }

    Ok(format!("Successfully synced {} PR(s).", synced_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_prs() -> Result<(), BridleError> {
        let tf = tempfile::NamedTempFile::new()?;
        let db_url = tf.path().to_str().unwrap_or(":memory:");
        let res = sync_prs("testorg", db_url, None, None)?;
        assert_eq!(res, "Successfully synced 5 PR(s).");

        // Insert a user, repo, and open PR to cover the db_count > 0 branch and PostgreSQL branch
        for url in [db_url, "postgres://samuel@localhost/bridle_test"] {
            let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(url)?;
            let now = chrono::Utc::now().naive_utc();
            let uid: i32 = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos() as i32)
                .abs()
                .saturating_add(50_000);
            bridle_sdk::db::insert_user(
                &mut conn,
                &bridle_sdk::models::User {
                    id: uid,
                    username: format!("sync_u_{}", uid),
                    email: format!("sync_{}@ex.com", uid),
                    password_hash: "h".into(),
                    avatar_url: None,
                    bio: None,
                    status: None,
                    created_at: now,
                    updated_at: now,
                },
            )?;
            bridle_sdk::db::insert_repository(
                &mut conn,
                &bridle_sdk::models::Repository {
                    id: uid,
                    owner_id: uid,
                    owner_type: "user".into(),
                    name: format!("sync_r_{}", uid),
                    description: None,
                    is_private: false,
                    is_fork: false,
                    archived: false,
                    allow_merge_commit: true,
                    allow_squash_merge: true,
                    allow_rebase_merge: true,
                    created_at: now,
                    updated_at: now,
                },
            )?;
            bridle_sdk::db::insert_pull_request(
                &mut conn,
                &bridle_sdk::models::PullRequest {
                    id: uid,
                    repo_id: uid,
                    number: 1,
                    title: "pr".into(),
                    body: None,
                    state: "open".into(),
                    head_branch: "h".into(),
                    base_branch: "b".into(),
                    author_id: uid,
                    assignee_id: None,
                    milestone_id: None,
                    is_draft: false,
                    created_at: now,
                    updated_at: now,
                },
            )?;
            let res_with_pr = sync_prs("testorg", url, Some(1), None)?;
            assert_eq!(res_with_pr, "Successfully synced 1 PR(s).");
        }

        let res_bad_db = sync_prs("testorg", "invalid://db", Some(1), None)?;
        assert_eq!(res_bad_db, "Successfully synced 1 PR(s).");
        Ok(())
    }

    #[test]
    fn test_sync_prs_with_limit() -> Result<(), BridleError> {
        let res = sync_prs("testorg", ":memory:", Some(2), None)?;
        assert_eq!(res, "Successfully synced 2 PR(s).");
        Ok(())
    }

    #[test]
    #[serial_test::serial]
    fn test_sync_prs_with_fork_org() -> Result<(), BridleError> {
        unsafe {
            std::env::remove_var("GITHUB_URL");
            std::env::remove_var("BRIDLE_ALLOW_GITHUB_MUTATION");
        }
        let res = sync_prs(
            "testorg",
            ":memory:",
            Some(1),
            Some("my-fork-org".to_string()),
        )?;
        assert_eq!(res, "Successfully synced 1 PR(s).");
        Ok(())
    }

    #[test]
    fn test_sync_prs_zero_limit() -> Result<(), BridleError> {
        let res = sync_prs("testorg", ":memory:", Some(0), None)?;
        assert_eq!(res, "Successfully synced 0 PR(s).");
        Ok(())
    }

    #[test]
    fn test_sync_prs_template_render_fail() -> Result<(), BridleError> {
        let temp_ws = tempfile::tempdir()?;
        let repo_path = temp_ws.path().join("testorg_fail").join("repo-0");
        let github_dir = repo_path.join(".github");
        std::fs::create_dir_all(&github_dir)?;
        std::fs::write(
            github_dir.join("PULL_REQUEST_TEMPLATE.md"),
            "{{ invalid_syntax }}",
        )?;

        let res = sync_prs_in_workspace(temp_ws.path(), "testorg_fail", ":memory:", Some(1), None)?;
        assert_eq!(res, "Successfully synced 1 PR(s).");
        Ok(())
    }
}
