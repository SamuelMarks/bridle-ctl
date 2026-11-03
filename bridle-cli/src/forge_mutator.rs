use crate::workspace::git_command;
use bridle_sdk::BridleError;
use reqwest::Client;
use serde_json::json;
use std::path::Path;
use std::time::Duration;

/// Returns true if `BRIDLE_ALLOW_GITHUB_MUTATION` is explicitly set to `"1"` or `"true"`.
pub fn is_github_mutation_allowed() -> bool {
    std::env::var("BRIDLE_ALLOW_GITHUB_MUTATION")
        .is_ok_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

/// Checks whether a mutating operation against the given target URL/host is permitted.
/// Read-only operations are always allowed; mutating operations against `github.com` or `api.github.com`
/// require `BRIDLE_ALLOW_GITHUB_MUTATION=1` and emit a reminder warning to `stderr` when blocked.
pub fn ensure_github_mutation_allowed(operation: &str, target: &str) -> Result<(), BridleError> {
    if target.contains("github.com") && !is_github_mutation_allowed() {
        let msg = format!(
            "WARNING: Live GitHub mutation ({}) to '{}' is disabled by default. Set BRIDLE_ALLOW_GITHUB_MUTATION=1 to allow creating orgs/repos/issues/PRs/forks or pushing to github.com.",
            operation, target
        );
        eprintln!("{}", msg);
        return Err(BridleError::Config(msg));
    }
    Ok(())
}

/// Resolves the GitHub git host for fork/PR remotes (`GITHUB_URL` if set, otherwise `github.com`).
pub fn resolve_github_host() -> String {
    std::env::var("GITHUB_URL").unwrap_or_else(|_| "github.com".to_string())
}

/// Git mutator functions.
pub struct GitMutator;

impl GitMutator {
    /// Adds a remote repository URL, falling back to `remote set-url` if the remote already exists.
    pub async fn add_remote(dir: &Path, remote_name: &str, url: &str) -> Result<(), BridleError> {
        ensure_github_mutation_allowed("git remote add/set-url", url)?;
        let status = git_command()
            .current_dir(dir)
            .args(["remote", "add", remote_name, url])
            .status()?;

        if !status.success() {
            let set_status = git_command()
                .current_dir(dir)
                .args(["remote", "set-url", remote_name, url])
                .status()?;
            if !set_status.success() {
                return Err(BridleError::Tool(format!(
                    "Failed to configure git remote {}",
                    remote_name
                )));
            }
        }
        Ok(())
    }

    /// Commits and pushes changes with retries for transient network issues.
    pub async fn commit_and_push(
        dir: &Path,
        commit_message: &str,
        branch_name: &str,
        remote: &str,
    ) -> Result<(), BridleError> {
        if let Ok(out) = git_command()
            .current_dir(dir)
            .args(["remote", "get-url", remote])
            .output()
            && out.status.success()
        {
            let remote_url = String::from_utf8_lossy(&out.stdout);
            ensure_github_mutation_allowed("git push", remote_url.trim())?;
        }
        ensure_github_mutation_allowed("git push", remote)?;

        let add_status = git_command()
            .current_dir(dir)
            .args(["add", "-A"])
            .status()?;
        if !add_status.success() {
            return Err(BridleError::Tool("Failed to stage changes".to_string()));
        }

        let _ = git_command()
            .current_dir(dir)
            .args(["commit", "-m", commit_message])
            .status()?;

        let mut retries = 0;
        loop {
            let status = git_command()
                .current_dir(dir)
                .args(["push", remote, branch_name, "--force-with-lease"])
                .status()?;

            if status.success() {
                return Ok(());
            }

            retries += 1;
            if retries >= 3 {
                return Err(BridleError::Tool("Failed to push changes".to_string()));
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
}

/// HTTP Client with Retry and Rate Limiting
pub struct ForgeClient {
    /// The underlying reqwest client.
    client: Client,
    /// The authentication token.
    pub token: String,
    /// API Base URL
    pub api_base: String,
}

impl ForgeClient {
    /// Create a new client using `GITHUB_API_URL` if set, or `https://api.github.com`.
    pub fn new(token: String) -> Result<Self, BridleError> {
        let api_base = std::env::var("GITHUB_API_URL")
            .unwrap_or_else(|_| "https://api.github.com".to_string());
        Self::with_base_url(token, api_base)
    }

    /// Create a new client with an explicit base URL.
    pub fn with_base_url(token: String, api_base: String) -> Result<Self, BridleError> {
        Ok(Self {
            client: Client::builder().user_agent("bridle-ctl/0.1.0").build()?,
            token,
            api_base,
        })
    }

    /// Internal helper to send requests with retries and rate limiting handling.
    async fn send_request(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<serde_json::Value, BridleError> {
        let mut retries = 0;
        loop {
            let request = req
                .try_clone()
                .ok_or_else(|| BridleError::Config("Failed to clone request".to_string()))?;
            match request.send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() || status == reqwest::StatusCode::ACCEPTED {
                        return Ok(resp.json().await?);
                    } else if (status == reqwest::StatusCode::TOO_MANY_REQUESTS
                        || status.is_server_error())
                        && retries < 3
                    {
                        retries += 1;
                        tokio::time::sleep(Duration::from_millis(10)).await;
                        continue;
                    }
                    return Err(BridleError::Tool(format!("API Error: {}", status)));
                }
                Err(e) => {
                    if retries >= 3 {
                        return Err(BridleError::Request(e));
                    }
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
        }
    }

    /// Fetches the current authenticated user's login.
    pub async fn get_current_user(&self) -> Result<String, BridleError> {
        let url = format!("{}/user", self.api_base);
        let req = self.client.get(&url).bearer_auth(&self.token);
        let json = self.send_request(req).await?;
        let login = json["login"]
            .as_str()
            .ok_or_else(|| BridleError::Tool("Missing login in response".to_string()))?;
        Ok(login.to_string())
    }

    /// Creates a fork of the specified repository and returns the fork owner's login.
    pub async fn create_fork(
        &self,
        repo_owner: &str,
        repo_name: &str,
    ) -> Result<String, BridleError> {
        let url = format!("{}/repos/{}/{}/forks", self.api_base, repo_owner, repo_name);
        ensure_github_mutation_allowed("create_fork", &url)?;
        let req = self.client.post(&url).bearer_auth(&self.token);
        let json = self.send_request(req).await?;
        let fork_owner = json["owner"]["login"]
            .as_str()
            .ok_or_else(|| BridleError::Tool("Missing fork owner in response".to_string()))?;
        Ok(fork_owner.to_string())
    }

    /// Submits a PR (GitHub as example)
    pub async fn submit_pr(
        &self,
        repo_owner: &str,
        repo_name: &str,
        title: &str,
        body: &str,
        head: &str,
        base: &str,
    ) -> Result<String, BridleError> {
        let url = format!("{}/repos/{}/{}/pulls", self.api_base, repo_owner, repo_name);
        ensure_github_mutation_allowed("submit_pr", &url)?;
        let payload = json!({
            "title": title,
            "body": body,
            "head": head,
            "base": base,
        });

        let req = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .json(&payload);
        let json = self.send_request(req).await?;
        let html_url = json["html_url"]
            .as_str()
            .ok_or_else(|| BridleError::Tool("Missing html_url in response".to_string()))?;
        Ok(html_url.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_forge_client_success_and_retry_paths() -> Result<(), BridleError> {
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();

        let handle = std::thread::spawn(move || {
            for attempt in 0..6 {
                if let Ok((mut stream, _)) = listener.accept() {
                    let mut buf = [0; 1024];
                    if let Ok(n) = stream.read(&mut buf) {
                        let req = String::from_utf8_lossy(&buf[..n]);
                        let response = if attempt == 0 {
                            "HTTP/1.1 429 TOO MANY REQUESTS\r\nContent-Length: 0\r\n\r\n"
                        } else if attempt == 1 {
                            "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Length: 0\r\n\r\n"
                        } else if req.contains("GET /user ") {
                            r#"HTTP/1.1 200 OK
Content-Type: application/json

{"login":"testuser"}"#
                        } else if req.contains("POST /repos/test/repo/forks ") {
                            r#"HTTP/1.1 202 ACCEPTED
Content-Type: application/json

{"owner":{"login":"testfork"}}"#
                        } else if req.contains("POST /repos/test/repo/pulls ") {
                            r#"HTTP/1.1 201 CREATED
Content-Type: application/json

{"html_url":"http://test"}"#
                        } else {
                            "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n"
                        };
                        let _ = stream.write_all(response.as_bytes());
                    }
                }
            }
        });

        let client =
            ForgeClient::with_base_url("dummy".to_string(), format!("http://127.0.0.1:{}", port))?;

        let user = client.get_current_user().await?;
        assert_eq!(user, "testuser");

        let fork = client.create_fork("test", "repo").await?;
        assert_eq!(fork, "testfork");

        let pr = client
            .submit_pr("test", "repo", "t", "b", "h", "base")
            .await?;
        assert_eq!(pr, "http://test");

        let bad_fork = client.create_fork("other", "repo").await;
        assert!(matches!(bad_fork, Err(BridleError::Tool(_))));

        let _ = handle.join();
        Ok(())
    }

    #[tokio::test]
    async fn test_git_mutator_add_remote_and_push_success() -> Result<(), BridleError> {
        let bare_dir = tempdir()?;
        git_command()
            .current_dir(bare_dir.path())
            .args(["init", "--bare"])
            .status()?;

        let work_dir = tempdir()?;
        git_command()
            .current_dir(work_dir.path())
            .args(["init", "-b", "main"])
            .status()?;
        git_command()
            .current_dir(work_dir.path())
            .args(["config", "user.name", "Test User"])
            .status()?;
        git_command()
            .current_dir(work_dir.path())
            .args(["config", "user.email", "test@example.com"])
            .status()?;

        let remote_url = bare_dir.path().to_str().unwrap_or("");
        GitMutator::add_remote(work_dir.path(), "origin", remote_url).await?;
        // Second call tests the fallback `remote set-url` branch
        GitMutator::add_remote(work_dir.path(), "origin", remote_url).await?;

        std::fs::write(work_dir.path().join("test.txt"), "hello")?;
        GitMutator::commit_and_push(work_dir.path(), "test msg", "main", "origin").await?;

        // Test failing push against invalid remote
        let err =
            GitMutator::commit_and_push(work_dir.path(), "test msg", "main", "bad_remote").await;
        assert!(matches!(err, Err(BridleError::Tool(_))));

        // Test failing add_remote on non-git dir
        let non_git = tempdir()?;
        let add_err = GitMutator::add_remote(non_git.path(), "origin", remote_url).await;
        assert!(matches!(add_err, Err(BridleError::Tool(_))));

        let commit_err = GitMutator::commit_and_push(non_git.path(), "msg", "main", "origin").await;
        assert!(matches!(commit_err, Err(BridleError::Tool(_))));

        Ok(())
    }

    #[tokio::test]
    async fn test_forge_client_network_error() -> Result<(), BridleError> {
        unsafe {
            std::env::remove_var("GITHUB_API_URL");
            std::env::remove_var("GITHUB_URL");
            std::env::remove_var("BRIDLE_ALLOW_GITHUB_MUTATION");
        }
        assert!(!is_github_mutation_allowed());
        assert_eq!(resolve_github_host(), "github.com");
        assert!(
            ensure_github_mutation_allowed("submit_pr", "https://api.github.com/repos/o/r/pulls")
                .is_err()
        );
        assert!(
            ensure_github_mutation_allowed("submit_pr", "http://127.0.0.1:8080/repos/o/r/pulls")
                .is_ok()
        );

        unsafe {
            std::env::set_var("BRIDLE_ALLOW_GITHUB_MUTATION", "1");
        }
        assert!(is_github_mutation_allowed());
        assert!(
            ensure_github_mutation_allowed("submit_pr", "https://api.github.com/repos/o/r/pulls")
                .is_ok()
        );
        unsafe {
            std::env::remove_var("BRIDLE_ALLOW_GITHUB_MUTATION");
        }

        let client = ForgeClient::new("token123".to_string())?;
        assert_eq!(client.token, "token123");
        assert!(client.create_fork("o", "r").await.is_err());
        assert!(
            client
                .submit_pr("o", "r", "t", "b", "h", "base")
                .await
                .is_err()
        );

        let unreachable =
            ForgeClient::with_base_url("token123".to_string(), "http://127.0.0.1:1".to_string())?;
        assert!(matches!(
            unreachable.get_current_user().await,
            Err(BridleError::Request(_))
        ));
        Ok(())
    }
}
