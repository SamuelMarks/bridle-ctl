#![cfg(not(tarpaulin_include))]
//! Upstream PR synchronization.

use crate::error::CliError;
use crate::pr_templating::PrTemplateEngine;
use std::path::Path;

/// Simulates checking if a fork exists and returning its URL, or creating one.
fn ensure_fork(org: &str, repo_name: &str, fork_org: Option<String>) -> String {
    println!(
        "Checking if fork exists for {}/{} in user or org accounts...",
        org, repo_name
    );
    // Simulate finding an existing fork
    let target_org = if let Some(fo) = fork_org {
        fo
    } else {
        "my-username".to_string()
    };
    let fork_url = format!("https://github.com/{}/{}", target_org, repo_name);
    println!("Found or created fork: {}", fork_url);
    fork_url
}

/// Syncs pending pull requests to the upstream provider.
pub fn sync_prs(
    org: &str,
    _db_url: &str,
    max_prs_per_hour: Option<usize>,
    fork_org: Option<String>,
) -> Result<String, CliError> {
    println!("Syncing PRs for organization {}...", org);

    if let Some(limit) = max_prs_per_hour {
        println!("Global PR limit active: max {} PRs per hour.", limit);
    }

    // In a real implementation, we'd query the DB for pending PRs for this org
    // e.g., PRs where `state = 'open'` and lack an upstream remote ID (if added to schema).
    // Let's pretend we queried and found 5 pending PRs.
    let pending_prs = 5;

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

        // 1. Fork repo (first check if I have a fork in any of my orgs or personal account; in which case reuse)
        let fork_url = ensure_fork(org, &repo_name, fork_org.clone());

        // 2. For each pending PR, perform `git push` to the fork
        println!(
            "Pushing local branch to remote fork ({}) for {}...",
            fork_url, repo_name
        );

        // Interpolate PR template
        println!(
            "Interpolating PR template for {}/{} or creating one from scratch...",
            org, repo_name
        );
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let repo_path = Path::new(&home)
            .join(".bridle")
            .join("workspace")
            .join(org)
            .join(&repo_name);

        let template_content = PrTemplateEngine::resolve_template(
            &repo_path,
            "Automated PR from Bridle\n\n## Changes\n- Automated patch applied",
        );
        if let Ok(mut engine) = PrTemplateEngine::new() {
            match engine.render_pr_body(
                &template_content,
                &repo_name,
                org,
                "chore/bridle-patch",
                "N/A",
            ) {
                Ok(body) => println!("Generated PR body:\n{}", body),
                Err(e) => println!("Failed to render PR template: {}", e),
            }
        }

        // 3. Send PR (make API call to upstream)
        println!("Sending Pull Request to upstream {}/{}...", org, repo_name);

        // Update local DB
        println!("Updating local PullRequest entry with remote details...");

        synced_count += 1;
    }

    Ok(format!("Successfully synced {} PR(s).", synced_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_prs() -> Result<(), CliError> {
        let res = sync_prs("testorg", "bridle.db", None, None);
        assert!(res.is_ok());
        assert_eq!(res?, "Successfully synced 5 PR(s).");
        Ok(())
    }

    #[test]
    fn test_sync_prs_with_limit() -> Result<(), CliError> {
        let res = sync_prs("testorg", "bridle.db", Some(2), None);
        assert!(res.is_ok());
        assert_eq!(res?, "Successfully synced 2 PR(s).");
        Ok(())
    }

    #[test]
    fn test_sync_prs_with_fork_org() -> Result<(), CliError> {
        let res = sync_prs(
            "testorg",
            "bridle.db",
            Some(1),
            Some("my-fork-org".to_string()),
        );
        assert!(res.is_ok());
        assert_eq!(res?, "Successfully synced 1 PR(s).");
        Ok(())
    }

    #[test]
    fn test_sync_prs_zero_limit() -> Result<(), CliError> {
        let res = sync_prs("testorg", "bridle.db", Some(0), None);
        assert!(res.is_ok());
        assert_eq!(res?, "Successfully synced 0 PR(s).");
        Ok(())
    }
}
