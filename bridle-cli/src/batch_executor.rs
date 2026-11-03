use crate::workspace::git_command;
use bridle_sdk::BridleError;
use bridle_sdk::models::TaskStatus;
use bridle_sdk::pipeline::{PipelineConfig, Step, StepType};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// Execute a single step with timeout.
#[tracing::instrument(skip(dir, step), fields(step_name = %step.name, step_type = ?step.step_type))]
pub async fn execute_step(dir: &Path, step: &Step) -> Result<(i32, String, String), BridleError> {
    let to_secs = step.timeout_seconds.unwrap_or(300);

    let mut cmd = Command::new(&step.command);
    if let Some(args) = &step.args {
        cmd.args(args);
    }
    cmd.current_dir(dir);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let child = cmd.spawn()?;

    let res = timeout(Duration::from_secs(to_secs), child.wait_with_output()).await;

    match res {
        Ok(Ok(output)) => {
            let code = output.status.code().unwrap_or(1);
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Ok((code, stdout, stderr))
        }
        Ok(Err(e)) => Err(BridleError::Io(e)),
        Err(_) => Err(BridleError::Tool("Timeout occurred".to_string())),
    }
}

/// Runs the execution engine for a repo workspace.
#[tracing::instrument(skip(dir, config), fields(pipeline = %config.name))]
pub async fn run_engine(dir: &Path, config: &PipelineConfig) -> Result<TaskStatus, BridleError> {
    for step in &config.steps {
        if step.step_type == StepType::Detect {
            let (code, _, _) = execute_step(dir, step).await?;
            if code == 0 {
                return Ok(TaskStatus::Clean);
            }
        } else if step.step_type == StepType::MkconfBuild {
            let (code, _stdout, _stderr) = execute_step(dir, step).await?;
            if code != 0 {
                return Ok(TaskStatus::FailedValidation);
            }
        } else if step.step_type == StepType::Fix {
            let _ = execute_step(dir, step).await?;
            let status = git_command()
                .current_dir(dir)
                .args(["diff", "--quiet"])
                .status()?;

            if status.success() {
                return Ok(TaskStatus::Clean);
            }
        } else if step.step_type == StepType::Validate {
            let (code, _, _stderr) = execute_step(dir, step).await?;
            if code != 0 {
                let _ = git_command()
                    .current_dir(dir)
                    .args(["reset", "--hard"])
                    .status();
                let _ = git_command()
                    .current_dir(dir)
                    .args(["clean", "-fd"])
                    .status();
                return Ok(TaskStatus::FailedValidation);
            }
        }
    }

    Ok(TaskStatus::PRSubmitted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_step_success() -> Result<(), BridleError> {
        let dir = std::env::current_dir()?;
        let step = Step {
            name: "test".to_string(),
            step_type: StepType::Detect,
            command: "echo".to_string(),
            args: Some(vec!["hello".to_string()]),
            timeout_seconds: Some(10),
            expected_exit_codes: None,
        };

        let (code, stdout, _) = execute_step(&dir, &step).await?;
        assert_eq!(code, 0);
        assert!(stdout.contains("hello"));
        Ok(())
    }

    #[tokio::test]
    async fn test_execute_step_timeout() -> Result<(), BridleError> {
        let dir = std::env::current_dir()?;
        let step = Step {
            name: "test_timeout".to_string(),
            step_type: StepType::Detect,
            command: "sleep".to_string(),
            args: Some(vec!["2".to_string()]),
            timeout_seconds: Some(1),
            expected_exit_codes: None,
        };

        let err = execute_step(&dir, &step).await;
        assert!(matches!(
            err,
            Err(BridleError::Tool(ref msg)) if msg == "Timeout occurred"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_execute_step_spawn_fail() -> Result<(), BridleError> {
        let dir = std::env::current_dir()?;
        let step = Step {
            name: "fail".to_string(),
            step_type: StepType::Detect,
            command: "nonexistent_command_123".to_string(),
            args: None,
            timeout_seconds: None,
            expected_exit_codes: None,
        };

        let err = execute_step(&dir, &step).await;
        assert!(matches!(err, Err(BridleError::Io(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_run_engine_detect_clean() -> Result<(), BridleError> {
        let dir = std::env::current_dir()?;
        let config = PipelineConfig {
            name: "pipe".to_string(),
            description: None,
            author: None,
            allowed_paths: None,
            ignored_paths: None,
            selectors: bridle_sdk::pipeline::Selectors {
                require_files: None,
                topics: None,
                languages: None,
            },
            pr_template: None,
            steps: vec![Step {
                name: "detect".to_string(),
                step_type: StepType::Detect,
                command: "true".to_string(),
                args: None,
                timeout_seconds: None,
                expected_exit_codes: None,
            }],
        };

        let status = run_engine(&dir, &config).await?;
        assert_eq!(status, TaskStatus::Clean);
        Ok(())
    }

    #[tokio::test]
    async fn test_run_engine_mkconf_fail_and_success() -> Result<(), BridleError> {
        let dir = std::env::current_dir()?;
        let mut config = PipelineConfig {
            name: "pipe".to_string(),
            description: None,
            author: None,
            allowed_paths: None,
            ignored_paths: None,
            selectors: bridle_sdk::pipeline::Selectors {
                require_files: None,
                topics: None,
                languages: None,
            },
            pr_template: None,
            steps: vec![Step {
                name: "mkconf".to_string(),
                step_type: StepType::MkconfBuild,
                command: "false".to_string(),
                args: None,
                timeout_seconds: None,
                expected_exit_codes: None,
            }],
        };

        let status = run_engine(&dir, &config).await?;
        assert_eq!(status, TaskStatus::FailedValidation);

        config.steps[0].command = "true".to_string();
        let status_ok = run_engine(&dir, &config).await?;
        assert_eq!(status_ok, TaskStatus::PRSubmitted);
        Ok(())
    }

    #[tokio::test]
    async fn test_run_engine_fix_clean_and_dirty() -> Result<(), BridleError> {
        let dir = tempfile::tempdir()?;
        git_command().arg("init").current_dir(dir.path()).status()?;
        git_command()
            .args(["config", "user.name", "Test User"])
            .current_dir(dir.path())
            .status()?;
        git_command()
            .args(["config", "user.email", "test@example.com"])
            .current_dir(dir.path())
            .status()?;
        std::fs::write(dir.path().join("file.txt"), "initial")?;
        git_command()
            .args(["add", "."])
            .current_dir(dir.path())
            .status()?;
        git_command()
            .args(["commit", "-m", "init"])
            .current_dir(dir.path())
            .status()?;

        let config = PipelineConfig {
            name: "pipe".to_string(),
            description: None,
            author: None,
            allowed_paths: None,
            ignored_paths: None,
            selectors: bridle_sdk::pipeline::Selectors {
                require_files: None,
                topics: None,
                languages: None,
            },
            pr_template: None,
            steps: vec![Step {
                name: "fix".to_string(),
                step_type: StepType::Fix,
                command: "true".to_string(),
                args: None,
                timeout_seconds: None,
                expected_exit_codes: None,
            }],
        };

        let status = run_engine(dir.path(), &config).await?;
        assert_eq!(status, TaskStatus::Clean);

        // Make tracked file dirty so `git diff --quiet` returns 1 and proceeds to PRSubmitted
        std::fs::write(dir.path().join("file.txt"), "modified")?;
        let status_dirty = run_engine(dir.path(), &config).await?;
        assert_eq!(status_dirty, TaskStatus::PRSubmitted);
        Ok(())
    }

    #[tokio::test]
    async fn test_run_engine_validate_fail_and_pass() -> Result<(), BridleError> {
        let dir = tempfile::tempdir()?;
        git_command().arg("init").current_dir(dir.path()).status()?;

        let mut config = PipelineConfig {
            name: "pipe".to_string(),
            description: None,
            author: None,
            allowed_paths: None,
            ignored_paths: None,
            selectors: bridle_sdk::pipeline::Selectors {
                require_files: None,
                topics: None,
                languages: None,
            },
            pr_template: None,
            steps: vec![Step {
                name: "validate".to_string(),
                step_type: StepType::Validate,
                command: "false".to_string(),
                args: None,
                timeout_seconds: None,
                expected_exit_codes: None,
            }],
        };

        let status = run_engine(dir.path(), &config).await?;
        assert_eq!(status, TaskStatus::FailedValidation);

        config.steps[0].command = "true".to_string();
        let status_pass = run_engine(dir.path(), &config).await?;
        assert_eq!(status_pass, TaskStatus::PRSubmitted);
        Ok(())
    }
}
