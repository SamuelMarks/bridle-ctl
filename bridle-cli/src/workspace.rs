use bridle_sdk::BridleError;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Creates a new `Command` for executing git operations, using the `GIT_COMMAND` environment variable if set, otherwise defaulting to `git`, and clearing conflicting git environment variables.
pub fn git_command() -> Command {
    let git_bin = std::env::var("GIT_COMMAND").unwrap_or_else(|_| "git".to_string());
    let mut cmd = Command::new(git_bin);
    cmd.env_remove("GIT_DIR");
    cmd.env_remove("GIT_WORK_TREE");
    cmd.env_remove("GIT_INDEX_FILE");
    cmd.env("GIT_AUTHOR_NAME", "Bridle Bot");
    cmd.env("GIT_AUTHOR_EMAIL", "bot@bridle.local");
    cmd.env("GIT_COMMITTER_NAME", "Bridle Bot");
    cmd.env("GIT_COMMITTER_EMAIL", "bot@bridle.local");
    cmd
}

/// Controller for an ephemeral git workspace.
#[derive(Debug)]
pub struct EphemeralWorkspace {
    /// The temporary directory path.
    pub path: PathBuf,
    /// The original repository path.
    pub orig_path: PathBuf,
}

impl EphemeralWorkspace {
    /// Creates a new ephemeral workspace.
    pub fn new(orig_path: &Path, pipeline_name: &str) -> Result<Self, BridleError> {
        let temp_dir =
            std::env::temp_dir().join(format!("bridle_{}_{}", pipeline_name, uuid::Uuid::new_v4()));

        let temp_dir_str = temp_dir
            .to_str()
            .ok_or_else(|| BridleError::Config("Invalid UTF-8 in temp dir path".to_string()))?;
        let orig_path_str = orig_path
            .to_str()
            .ok_or_else(|| BridleError::Config("Invalid UTF-8 in orig path".to_string()))?;

        // 1. Perform a shallow clone (`--depth=1`) from the local path (`file://...`)
        let orig_url = format!("file://{}", orig_path_str);
        let status = git_command()
            .args(["clone", "--depth=1", &orig_url, temp_dir_str])
            .status()?;

        if !status.success() {
            let cp_status = Command::new("cp")
                .args(["-r", orig_path_str, temp_dir_str])
                .status()?;
            if !cp_status.success() {
                return Err(BridleError::Tool("Failed to copy workspace".to_string()));
            }
        }

        // 2. Synchronize the `origin` to the true upstream remote, replacing the `file://` local reference.
        if let Ok(output) = git_command()
            .current_dir(orig_path)
            .args(["config", "--get", "remote.origin.url"])
            .output()
            && let Ok(url) = String::from_utf8(output.stdout)
        {
            let url = url.trim();
            if !url.is_empty() {
                let _ = git_command()
                    .current_dir(&temp_dir)
                    .args(["remote", "set-url", "origin", url])
                    .status();
            }
        }

        // 3. Checkout the target ephemeral branch
        let branch_name = format!("chore/bridle-auto/{}", pipeline_name);
        let branch_status = git_command()
            .current_dir(&temp_dir)
            .args(["checkout", "-b", &branch_name])
            .status()?;

        if !branch_status.success() {
            return Err(BridleError::Tool("Failed to checkout branch".to_string()));
        }

        Ok(Self {
            path: temp_dir,
            orig_path: orig_path.to_path_buf(),
        })
    }
}

impl Drop for EphemeralWorkspace {
    fn drop(&mut self) {
        if self.path.join("docker-compose.yml").exists()
            || self.path.join("docker-compose.yaml").exists()
        {
            let _ = Command::new("docker-compose")
                .current_dir(&self.path)
                .args(["down", "--rmi", "local", "-v", "--remove-orphans"])
                .status();
        }

        let _ = std::fs::remove_dir_all(self.path.join("node_modules"));
        let _ = std::fs::remove_dir_all(self.path.join("target"));

        if std::fs::remove_dir_all(&self.path).is_err()
            && let Some(path_str) = self.path.to_str()
        {
            let _ = Command::new("rm").args(["-rf", path_str]).status();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use tempfile::tempdir;

    #[test]
    #[serial]
    fn test_ephemeral_workspace() -> Result<(), BridleError> {
        let dir = tempdir()?;
        git_command()
            .current_dir(dir.path())
            .args(["init"])
            .status()?;
        git_command()
            .current_dir(dir.path())
            .args(["config", "user.name", "Test User"])
            .status()?;
        git_command()
            .current_dir(dir.path())
            .args(["config", "user.email", "test@example.com"])
            .status()?;
        std::fs::write(dir.path().join("test.txt"), "hello")?;
        git_command()
            .current_dir(dir.path())
            .args(["add", "."])
            .status()?;
        git_command()
            .current_dir(dir.path())
            .args(["commit", "-m", "init"])
            .status()?;

        git_command()
            .current_dir(dir.path())
            .args(["remote", "add", "origin", "https://example.com/repo.git"])
            .status()?;

        let ws_path = {
            let ws = EphemeralWorkspace::new(dir.path(), "test_pipe")?;
            assert!(ws.path.exists());
            assert!(ws.path.join("test.txt").exists());
            assert_eq!(ws.orig_path, dir.path());

            let output = git_command()
                .current_dir(&ws.path)
                .args(["config", "--get", "remote.origin.url"])
                .output()?;
            let url = String::from_utf8(output.stdout)?;
            assert_eq!(url.trim(), "https://example.com/repo.git");

            std::fs::create_dir(ws.path.join("node_modules"))?;
            std::fs::write(ws.path.join("docker-compose.yml"), "version: '3'")?;
            ws.path.clone()
        };

        assert!(!ws_path.exists());
        Ok(())
    }

    #[test]
    #[serial]
    fn test_ephemeral_workspace_clone_fail() {
        let res = EphemeralWorkspace::new(Path::new("/nonexistent/path/123"), "pipe");
        assert!(matches!(res, Err(BridleError::Tool(_))));
    }

    #[test]
    #[serial]
    fn test_ephemeral_workspace_checkout_fail() -> Result<(), BridleError> {
        let dir = tempdir()?;
        git_command()
            .current_dir(dir.path())
            .args(["init"])
            .status()?;

        let res = EphemeralWorkspace::new(dir.path(), "invalid branch name");
        assert!(matches!(res, Err(BridleError::Tool(_))));
        Ok(())
    }

    #[test]
    #[serial]
    fn test_ephemeral_workspace_drop_rm_fallback() -> Result<(), BridleError> {
        let dir = tempdir()?;
        git_command()
            .current_dir(dir.path())
            .args(["init"])
            .status()?;

        let ws = EphemeralWorkspace::new(dir.path(), "pipe")?;
        std::fs::remove_dir_all(&ws.path)?;
        Ok(())
    }
}
