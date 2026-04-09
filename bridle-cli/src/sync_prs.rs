//! Upstream PR synchronization.

use crate::error::CliError;

/// Simulates checking if a fork exists and returning its URL, or creating one.
fn ensure_fork(org: &str, repo_name: &str) -> String {
    println!(
        "Checking if fork exists for {}/{} in user or org accounts...",
        org, repo_name
    );
    // Simulate finding an existing fork
    let fork_url = format!("https://github.com/my-username/{}", repo_name);
    println!("Found or created fork: {}", fork_url);
    fork_url
}

/// Syncs pending pull requests to the upstream provider.
pub fn sync_prs(
    org: &str,
    _db_url: &str,
    max_prs_per_hour: Option<usize>,
) -> Result<String, CliError> {
    println!("Syncing PRs for organization {}...", org);

    if let Some(limit) = max_prs_per_hour {
        println!("Global PR limit active: max {} PRs per hour.", limit);
    }

    // In a real implementation, we'd query the DB for pending PRs for this org
    // e.g., PRs where `state = 'open'` and lack an upstream remote ID (if added to schema).
    // Let's pretend we queried and found 5 pending PRs.
    let pending_prs = 5;
    if pending_prs == 0 {
        return Ok("No pending PRs to sync.".to_string());
    }

    println!("Found {} pending PR(s).", pending_prs);

    let mut synced_count = 0;
    for i in 0..pending_prs {
        if let Some(limit) = max_prs_per_hour {
            if synced_count >= limit {
                println!(
                    "Reached global PR limit of {} PRs per hour. Stopping sync.",
                    limit
                );
                break;
            }
        }

        let repo_name = format!("repo-{}", i);

        // 1. Fork repo (first check if I have a fork in any of my orgs or personal account; in which case reuse)
        let fork_url = ensure_fork(org, &repo_name);

        // 2. For each pending PR, perform `git push` to the fork
        println!(
            "Pushing local branch to remote fork ({}) for {}...",
            fork_url, repo_name
        );

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
    fn test_sync_prs() -> Result<(), Box<dyn std::error::Error>> {
        let res = sync_prs("testorg", "bridle.db", None);
        assert!(res.is_ok());
        assert_eq!(res?, "Successfully synced 5 PR(s).");
        Ok(())
    }

    #[test]
    fn test_sync_prs_with_limit() -> Result<(), Box<dyn std::error::Error>> {
        let res = sync_prs("testorg", "bridle.db", Some(2));
        assert!(res.is_ok());
        assert_eq!(res?, "Successfully synced 2 PR(s).");
        Ok(())
    }
}
