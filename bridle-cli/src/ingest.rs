//! Ingests remote repositories into local DB and workspace.

use crate::error::CliError;
use bridle_sdk::models::{Organisation, Repository};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::process::Command;

/// GitHub API Response for Repository
#[derive(Debug, Deserialize)]
pub struct GithubRepo {
    /// Name of repo
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Clone URL
    pub clone_url: String,
    /// Private or not
    pub private: bool,
    /// Fork or not
    pub fork: bool,
    /// Archived or not
    pub archived: bool,
    /// Updated at
    pub updated_at: String,
}

/// Ingests all repositories for an organization from GitHub.
#[cfg(not(tarpaulin_include))]
pub fn ingest_org(org: &str, provider: &str, db_url: &str) -> Result<String, CliError> {
    if provider != "github" {
        return Err(CliError::Execution(format!(
            "Unsupported provider: {}",
            provider
        )));
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent("bridle-ctl")
        .build()?;

    let url = format!("https://api.github.com/orgs/{}/repos?per_page=100", org);
    let mut repos = Vec::new();
    let mut page = 1;

    loop {
        let page_url = format!("{}&page={}", url, page);
        let resp = client.get(&page_url).send()?;

        if !resp.status().is_success() {
            return Err(CliError::Execution(format!(
                "Failed to fetch repositories: {}",
                resp.status()
            )));
        }

        let page_repos: Vec<GithubRepo> = resp.json()?;
        if page_repos.is_empty() {
            break;
        }

        let one_year_ago = chrono::Utc::now() - chrono::Duration::days(365);
        for repo in page_repos {
            if repo.fork || repo.archived {
                continue;
            }
            if let Ok(updated_time) = chrono::DateTime::parse_from_rfc3339(&repo.updated_at)
                && updated_time.with_timezone(&chrono::Utc) > one_year_ago
            {
                repos.push(repo);
            }
        }

        page += 1;
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let workspace = Path::new(&home).join(".bridle").join("workspace").join(org);
    fs::create_dir_all(&workspace)?;

    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(db_url)
        .map_err(|e| CliError::Execution(e.to_string()))?;

    let now = chrono::Utc::now().naive_utc();

    // We create a dummy organisation. In real implementation, we'd fetch org details.
    let new_org = Organisation {
        id: 1, // Need to handle ID properly or rely on autoincrement. Let's assume 1 for now.
        name: org.to_string(),
        description: None,
        verified_domain: None,
        billing_plan: "free".to_string(),
        created_at: now,
        updated_at: now,
    };

    // Attempt to insert org, ignore error if it exists.
    let _ = bridle_sdk::db::insert_organisation(&mut conn, &new_org);

    // We also need a dummy user for owner_id.
    let dummy_user = bridle_sdk::models::User {
        id: 1,
        username: "bridle_system".to_string(),
        email: "system@bridle.local".to_string(),
        password_hash: "".to_string(),
        avatar_url: None,
        bio: None,
        status: None,
        created_at: now,
        updated_at: now,
    };
    let _ = bridle_sdk::db::insert_user(&mut conn, &dummy_user);

    for (i, repo) in repos.iter().enumerate() {
        let repo_dir = workspace.join(&repo.name);
        if !repo_dir.exists() {
            println!("Cloning {}...", repo.name);
            let status = Command::new("git")
                .env_remove("GIT_DIR")
                .env_remove("GIT_WORK_TREE")
                .env_remove("GIT_INDEX_FILE")
                .args(["clone", &repo.clone_url])
                .current_dir(&workspace)
                .status()?;

            if !status.success() {
                println!("Failed to clone {}", repo.name);
            }
        } else {
            println!("Repository {} already exists. Skipping clone.", repo.name);
        }

        let new_repo = Repository {
            id: (i + 1) as i32,
            owner_id: dummy_user.id,
            owner_type: "user".to_string(),
            name: repo.name.clone(),
            description: repo.description.clone(),
            is_private: repo.private,
            is_fork: repo.fork,
            archived: repo.archived,
            allow_merge_commit: true,
            allow_squash_merge: true,
            allow_rebase_merge: true,
            created_at: now,
            updated_at: now,
        };

        let _ = bridle_sdk::db::insert_repository(&mut conn, &new_repo);
    }

    Ok(format!("Ingested {} repositories for {}", repos.len(), org))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingest_org_unsupported_provider() {
        let res = ingest_org("testorg", "gitlab", "bridle.db");
        if let Err(e) = res {
            assert_eq!(
                e.to_string(),
                "Execution Error: Unsupported provider: gitlab"
            );
        } else {
            panic!("Expected error");
        }
    }
}
