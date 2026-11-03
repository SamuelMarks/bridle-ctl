//! Ingests remote repositories into local DB and workspace.

use crate::workspace::git_command;
use bridle_sdk::BridleError;
use bridle_sdk::models::{Organisation, Repository};
use serde::Deserialize;
use std::fs;
use std::path::Path;

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

/// Ingests all repositories for an organization from GitHub using default settings.
pub fn ingest_org(org: &str, provider: &str, db_url: &str) -> Result<String, BridleError> {
    let api_base =
        std::env::var("GITHUB_API_URL").unwrap_or_else(|_| "https://api.github.com".to_string());
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let workspace_root = Path::new(&home).join(".bridle").join("workspace");
    ingest_org_with_config(org, provider, db_url, &api_base, &workspace_root)
}

/// Ingests all repositories for an organization using explicit API base URL and workspace root.
pub fn ingest_org_with_config(
    org: &str,
    provider: &str,
    db_url: &str,
    api_base: &str,
    workspace_root: &Path,
) -> Result<String, BridleError> {
    if provider != "github" {
        return Err(BridleError::Config(format!(
            "Unsupported provider: {}",
            provider
        )));
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent("bridle-ctl")
        .build()?;

    let url = format!("{}/orgs/{}/repos?per_page=100", api_base, org);
    let mut repos = Vec::new();
    let mut page = 1;

    loop {
        let page_url = format!("{}&page={}", url, page);
        let resp = client.get(&page_url).send()?;

        if !resp.status().is_success() {
            return Err(BridleError::Tool(format!(
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

    let workspace = workspace_root.join(org);
    fs::create_dir_all(&workspace)?;

    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(db_url)?;

    let now = chrono::Utc::now().naive_utc();

    let new_org = Organisation {
        id: 1,
        name: org.to_string(),
        description: None,
        verified_domain: None,
        billing_plan: "free".to_string(),
        created_at: now,
        updated_at: now,
    };

    let _ = bridle_sdk::db::insert_organisation(&mut conn, &new_org);

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
            let status = git_command()
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
    use std::io::{Read, Write};

    #[test]
    fn test_ingest_org_success_and_error_branches() -> Result<(), BridleError> {
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();

        let handle = std::thread::spawn(move || {
            for attempt in 0..5 {
                if let Ok((mut stream, _)) = listener.accept() {
                    let mut buf = [0; 1024];
                    if let Ok(n) = stream.read(&mut buf) {
                        let req = String::from_utf8_lossy(&buf[..n]);
                        let response = if attempt == 4 {
                            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n"
                        } else if req.contains("page=1 ") {
                            r#"HTTP/1.1 200 OK
Content-Type: application/json

[{"name":"repo1","clone_url":"http://127.0.0.1:1/invalid.git","description":"desc","private":false,"fork":false,"archived":false,"updated_at":"2030-01-01T00:00:00Z"},{"name":"forkrepo","clone_url":"http://127.0.0.1:1/fork.git","description":null,"private":false,"fork":true,"archived":false,"updated_at":"2030-01-01T00:00:00Z"}]"#
                        } else {
                            r#"HTTP/1.1 200 OK
Content-Type: application/json

[]"#
                        };
                        let _ = stream.write_all(response.as_bytes());
                    }
                }
            }
        });

        let ws_root = tempfile::tempdir()?;
        let tf = tempfile::NamedTempFile::new()?;
        let db_url = tf.path().to_str().unwrap_or(":memory:");
        let api_base = format!("http://127.0.0.1:{}", port);

        let res = ingest_org_with_config("testorg", "github", db_url, &api_base, ws_root.path())?;
        assert_eq!(res, "Ingested 1 repositories for testorg");

        // Pre-create repo1 dir so second call exercises the "already exists" branch
        std::fs::create_dir_all(ws_root.path().join("testorg").join("repo1"))?;
        let res2 = ingest_org_with_config("testorg", "github", db_url, &api_base, ws_root.path())?;
        assert_eq!(res2, "Ingested 1 repositories for testorg");

        // 5th request returns 404 NOT FOUND
        let err_res =
            ingest_org_with_config("missing", "github", db_url, &api_base, ws_root.path());
        assert!(matches!(err_res, Err(BridleError::Tool(_))));

        let _ = handle.join();
        Ok(())
    }

    #[test]
    fn test_ingest_org_unsupported_provider() {
        let res = ingest_org("testorg", "gitlab", ":memory:");
        assert!(matches!(
            res,
            Err(BridleError::Config(ref msg)) if msg == "Unsupported provider: gitlab"
        ));
    }

    #[test]
    fn test_ingest_github_repo_struct() -> Result<(), BridleError> {
        let json = r#"{"name": "test", "clone_url": "url", "private": false, "fork": false, "archived": false, "updated_at": "2026-04-08T00:00:00Z"}"#;
        let parsed: GithubRepo = serde_json::from_str(json)?;
        assert_eq!(parsed.name, "test");
        Ok(())
    }
}
