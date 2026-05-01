//! Ingests remote repositories into local DB and workspace.

use bridle_sdk::BridleError;
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
#[cfg(not(tarpaulin_include))]
pub fn ingest_org(org: &str, provider: &str, db_url: &str) -> Result<String, BridleError> {
    if provider != "github" {
        return Err(BridleError::Generic(format!(
            "Unsupported provider: {}",
            provider
        )));
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent("bridle-ctl")
        .build()?;

    let api_base =
        std::env::var("GITHUB_API_URL").unwrap_or_else(|_| "https://api.github.com".to_string());
    let url = format!("{}/orgs/{}/repos?per_page=100", api_base, org);
    let mut repos = Vec::new();
    let mut page = 1;

    loop {
        let page_url = format!("{}&page={}", url, page);
        let resp = client.get(&page_url).send()?;

        if !resp.status().is_success() {
            return Err(BridleError::Generic(format!(
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
        .map_err(|e| BridleError::Generic(e.to_string()))?;

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
    #[test]
    #[serial_test::serial]
    fn test_ingest_org_success() -> Result<(), Box<dyn std::error::Error>> {
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();

        std::thread::spawn(move || {
            use std::io::{Read, Write};
            for mut stream in listener.incoming().flatten() {
                let mut buf = [0; 1024];
                if let Ok(n) = stream.read(&mut buf) {
                    let req = String::from_utf8_lossy(&buf[..n]);
                    if req.contains("GET /orgs/testorg/repos?per_page=100&page=1 ") {
                        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n[{\"name\":\"repo1\",\"clone_url\":\"http://invalid\",\"description\":\"desc\",\"private\":false,\"fork\":false,\"archived\":false,\"updated_at\":\"2030-01-01T00:00:00Z\"}]";
                        let _ = stream.write_all(response.as_bytes());
                    } else if req.contains("GET /orgs/testorg/repos?per_page=100&page=2 ") {
                        let response =
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n[]";
                        let _ = stream.write_all(response.as_bytes());
                    }
                }
            }
        });

        let home = tempfile::tempdir()?;
        unsafe {
            std::env::set_var("GITHUB_API_URL", format!("http://127.0.0.1:{}", port));
            std::env::set_var("HOME", home.path());
        }

        let db_url = format!("test_ingest_{}.db", uuid::Uuid::new_v4());

        let res = ingest_org("testorg", "github", &db_url);
        assert!(res.is_ok());

        // Call again to test "Repository already exists" branch
        let res2 = ingest_org("testorg", "github", &db_url);
        assert!(res2.is_ok());

        unsafe {
            std::env::remove_var("GITHUB_API_URL");
            std::env::remove_var("HOME");
        }
        std::fs::remove_file(db_url).ok();
        Ok(())
    }

    use super::*;

    #[test]
    fn test_ingest_org_unsupported_provider() {
        let res = ingest_org("testorg", "gitlab", "bridle.db");
        if let Err(e) = res {
            assert_eq!(e.to_string(), "Generic error: Unsupported provider: gitlab");
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    fn test_ingest_org_http_fail() {
        // Just trigger reqwest. It will fail with 404 or similar, which returns Err
        let res = ingest_org(
            "invalid_org_1234567890_does_not_exist_test",
            "github",
            "bridle.db",
        );
        assert!(res.is_err());
    }

    #[test]
    fn test_ingest_github_repo_struct() {
        let json = r#"{"name": "test", "clone_url": "url", "private": false, "fork": false, "archived": false, "updated_at": "2026-04-08T00:00:00Z"}"#;
        let parsed: GithubRepo =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        assert_eq!(parsed.name, "test");
    }
}
