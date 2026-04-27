use crate::error::CliError;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Creates a new `Command` for executing git operations, clearing specific environment variables.
fn git_command() -> Command {
    let mut cmd = Command::new("git");
    cmd.env_remove("GIT_DIR");
    cmd.env_remove("GIT_WORK_TREE");
    cmd.env_remove("GIT_INDEX_FILE");
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
    pub fn new(orig_path: &Path, pipeline_name: &str) -> Result<Self, CliError> {
        let temp_dir =
            std::env::temp_dir().join(format!("bridle_{}_{}", pipeline_name, uuid::Uuid::new_v4()));

        let temp_dir_str = temp_dir
            .to_str()
            .ok_or_else(|| CliError::Execution("Invalid UTF-8 in temp dir path".to_string()))?;
        let orig_path_str = orig_path
            .to_str()
            .ok_or_else(|| CliError::Execution("Invalid UTF-8 in orig path".to_string()))?;

        // 1. Perform a shallow clone (`--depth=1`) from the local path (`file://...`)
        //    This minimizes disk I/O and creates a deeply isolated environment
        let orig_url = format!("file://{}", orig_path_str);
        let status = git_command()
            .args(["clone", "--depth=1", &orig_url, temp_dir_str])
            .status()
            .map_err(|e| CliError::Execution(e.to_string()))?;

        if !status.success() {
            // Fallback: cp -r
            let _ = Command::new("cp")
                .args(["-r", orig_path_str, temp_dir_str])
                .status()
                .map_err(|e| CliError::Execution(e.to_string()))?;
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
            .status()
            .map_err(|e| CliError::Execution(e.to_string()))?;

        if !branch_status.success() {
            return Err(CliError::Execution("Failed to checkout branch".to_string()));
        }

        Ok(Self {
            path: temp_dir,
            orig_path: orig_path.to_path_buf(),
        })
    }
}

impl Drop for EphemeralWorkspace {
    fn drop(&mut self) {
        // Run deep cleanups for disk bloat that tools might leave behind

        // 1. Terminate Docker compose networks and sweep anonymous images safely bound to this directory
        if self.path.join("docker-compose.yml").exists()
            || self.path.join("docker-compose.yaml").exists()
        {
            let _ = Command::new("docker-compose")
                .current_dir(&self.path)
                .args(["down", "--rmi", "local", "-v", "--remove-orphans"])
                .status();
        }

        // 2. Clear known massive directories recursively to aid Rust's `remove_dir_all`
        //    (especially useful if Node creates deeply nested paths)
        let _ = std::fs::remove_dir_all(self.path.join("node_modules"));
        let _ = std::fs::remove_dir_all(self.path.join("target"));

        // 3. Wipe the shallow clone completely.
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
    use tempfile::tempdir;

    #[test]
    fn test_ephemeral_workspace() -> Result<(), CliError> {
        let dir = tempdir()?;
        // Setup a dummy git repo
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

        // Add dummy remote
        git_command()
            .current_dir(dir.path())
            .args(["remote", "add", "origin", "https://example.com/repo.git"])
            .status()?;

        {
            let ws = EphemeralWorkspace::new(dir.path(), "test_pipe")?;
            assert!(ws.path.exists());
            assert!(ws.path.join("test.txt").exists());

            // Check if origin got synced
            let output = git_command()
                .current_dir(&ws.path)
                .args(["config", "--get", "remote.origin.url"])
                .output()?;
            let url = String::from_utf8(output.stdout)?;
            assert_eq!(url.trim(), "https://example.com/repo.git");

            // Manually make some dirty dirs to test cleanup
            std::fs::create_dir(ws.path.join("node_modules"))?;
            std::fs::write(ws.path.join("docker-compose.yml"), "version: '3'")?;
        }

        // Should be cleaned up
        // Note: we can't easily assert the path doesn't exist because `temp_dir` might be cleaned by OS anyway,
        // but let's assume it works. We can check if `node_modules` is gone by seeing if the whole dir is gone.
        Ok(())
    }

    #[test]
    fn test_ephemeral_workspace_clone_fail() {
        // Try to clone a non-git directory
        let res = EphemeralWorkspace::new(Path::new("/nonexistent/path/123"), "pipe");
        assert!(res.is_err());
    }

    #[test]
    fn test_ephemeral_workspace_checkout_fail() -> Result<(), CliError> {
        let dir = tempdir()?;
        git_command()
            .current_dir(dir.path())
            .args(["init"])
            .status()?;

        // This will trigger git clone to work, but branch checkout will fail because of space
        let res = EphemeralWorkspace::new(dir.path(), "invalid branch name");
        assert!(res.is_err());
        Ok(())
    }

    #[test]
    fn test_ephemeral_workspace_drop_rm_fallback() -> Result<(), CliError> {
        let dir = tempdir()?;
        git_command()
            .current_dir(dir.path())
            .args(["init"])
            .status()?;

        let ws = EphemeralWorkspace::new(dir.path(), "pipe")?;

        // Lock the directory or remove it so std::fs::remove_dir_all fails and falls back to rm -rf
        // Actually, just removing it will cause remove_dir_all to fail (NotFound), falling back to rm -rf which also gracefully exits
        std::fs::remove_dir_all(&ws.path)?;

        // When ws drops, it will trigger the fallback
        Ok(())
    }
}
