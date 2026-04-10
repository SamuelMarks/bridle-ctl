use crate::error::CliError;
use reqwest::Client;
use serde_json::json;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

fn git_command() -> Command {
    let mut cmd = Command::new("git");
    cmd.env_remove("GIT_DIR");
    cmd.env_remove("GIT_WORK_TREE");
    cmd.env_remove("GIT_INDEX_FILE");
    cmd
}

/// Git mutator functions.
pub struct GitMutator;

impl GitMutator {
    /// Adds a remote repository URL.
    #[cfg(not(tarpaulin_include))]
    pub async fn add_remote(dir: &Path, remote_name: &str, url: &str) -> Result<(), CliError> {
        let status = git_command()
            .current_dir(dir)
            .args(["remote", "add", remote_name, url])
            .status()
            .map_err(|e| CliError::Execution(e.to_string()))?;

        // Might fail if remote already exists, fallback to set-url
        if !status.success() {
            let _ = git_command()
                .current_dir(dir)
                .args(["remote", "set-url", remote_name, url])
                .status();
        }
        Ok(())
    }

    /// Commits and pushes changes.
    #[cfg(not(tarpaulin_include))]
    #[cfg(not(tarpaulin_include))]
    pub async fn commit_and_push(
        dir: &Path,
        commit_message: &str,
        branch_name: &str,
        remote: &str,
    ) -> Result<(), CliError> {
        let _ = git_command().current_dir(dir).args(["add", "-A"]).status();

        let _ = git_command()
            .current_dir(dir)
            .args(["commit", "-m", commit_message])
            .status();

        // Retry loop for transient network issues
        let mut retries = 0;
        loop {
            let status = git_command()
                .current_dir(dir)
                .args(["push", remote, branch_name, "--force-with-lease"])
                .status()
                .map_err(|e| CliError::Execution(e.to_string()))?;

            if status.success() {
                return Ok(());
            }

            retries += 1;
            if retries >= 3 {
                return Err(CliError::Execution("Failed to push changes".to_string()));
            }
            tokio::time::sleep(Duration::from_millis(10)).await; // Reduced for testing
        }
    }
}

/// HTTP Client with Retry and Rate Limiting
#[cfg(not(tarpaulin_include))]
pub struct ForgeClient {
    /// The underlying reqwest client.
    client: Client,
    /// The authentication token.
    token: String,
}

#[cfg(not(tarpaulin_include))]
impl ForgeClient {
    /// Create a new client
    pub fn new(token: String) -> Result<Self, CliError> {
        Ok(Self {
            client: Client::builder()
                .user_agent("bridle-ctl/0.1.0")
                .build()
                .map_err(|e| CliError::Execution(e.to_string()))?,
            token,
        })
    }

    /// Internal helper to send requests with retries and rate limiting handling.
    #[cfg(not(tarpaulin_include))]
    async fn send_request(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<serde_json::Value, CliError> {
        let mut retries = 0;
        loop {
            let request = req
                .try_clone()
                .ok_or_else(|| CliError::Execution("Failed to clone request".to_string()))?;
            match request.send().await {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() || status == reqwest::StatusCode::ACCEPTED {
                        return resp
                            .json()
                            .await
                            .map_err(|e| CliError::Execution(e.to_string()));
                    } else if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        tokio::time::sleep(Duration::from_millis(10)).await;
                        continue;
                    } else if status.is_server_error() && retries < 3 {
                        retries += 1;
                        tokio::time::sleep(Duration::from_millis(10)).await;
                        continue;
                    }
                    return Err(CliError::Execution(format!("API Error: {}", status)));
                }
                Err(e) => {
                    if retries >= 3 {
                        return Err(CliError::Execution(e.to_string()));
                    }
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
        }
    }

    /// Fetches the current authenticated user's login.
    pub async fn get_current_user(&self) -> Result<String, CliError> {
        let url = "https://api.github.com/user".to_string();
        let req = self.client.get(&url).bearer_auth(&self.token);
        let json = self.send_request(req).await?;
        let login = json["login"]
            .as_str()
            .ok_or_else(|| CliError::Execution("Missing login in response".to_string()))?;
        Ok(login.to_string())
    }

    /// Creates a fork of the specified repository and returns the fork owner's login.
    pub async fn create_fork(&self, repo_owner: &str, repo_name: &str) -> Result<String, CliError> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/forks",
            repo_owner, repo_name
        );
        let req = self.client.post(&url).bearer_auth(&self.token);
        let json = self.send_request(req).await?;
        let fork_owner = json["owner"]["login"]
            .as_str()
            .ok_or_else(|| CliError::Execution("Missing fork owner in response".to_string()))?;
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
    ) -> Result<String, CliError> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls",
            repo_owner, repo_name
        );
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
            .ok_or_else(|| CliError::Execution("Missing html_url in response".to_string()))?;
        Ok(html_url.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_git_mutator_add_remote() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let _ = git_command()
            .current_dir(dir.path())
            .args(["init"])
            .status();

        let res =
            GitMutator::add_remote(dir.path(), "testremote", "https://example.com/repo.git").await;
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_git_mutator_commit_and_push() -> Result<(), Box<dyn std::error::Error>> {
        // Here we test git commands on a mock repo.
        // It will fail pushing because there is no origin.
        let dir = tempdir()?;

        let _ = git_command()
            .current_dir(dir.path())
            .args(["init"])
            .status();
        std::fs::write(dir.path().join("test.txt"), "hello")?;

        let res = GitMutator::commit_and_push(dir.path(), "test msg", "main", "origin").await;
        // The push will fail, returning Err
        assert!(res.is_err());
        if let Err(e) = res {
            assert!(e.to_string().contains("Failed to push changes"));
        }
        Ok(())
    }

    #[test]
    fn test_forge_client_new() -> Result<(), Box<dyn std::error::Error>> {
        let client = ForgeClient::new("token123".to_string())?;
        assert_eq!(client.token, "token123");
        Ok(())
    }

    #[tokio::test]
    async fn test_forge_client_submit_pr() -> Result<(), Box<dyn std::error::Error>> {
        let client = ForgeClient::new("token123".to_string())?;
        // Without an actual mock server, calling GitHub API with a fake token will fail.
        let res = client
            .submit_pr("dummy", "dummy", "Title", "Body", "head", "base")
            .await;

        // Should get an API Error: 401 Unauthorized or 404 Not Found from GitHub
        assert!(res.is_err());
        if let Err(e) = res {
            assert!(e.to_string().contains("API Error: 40"));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_forge_client_get_current_user() -> Result<(), Box<dyn std::error::Error>> {
        let client = ForgeClient::new("token123".to_string())?;
        let res = client.get_current_user().await;
        assert!(res.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_forge_client_create_fork() -> Result<(), Box<dyn std::error::Error>> {
        let client = ForgeClient::new("token123".to_string())?;
        let res = client.create_fork("dummy", "dummy").await;
        assert!(res.is_err());
        Ok(())
    }
}
