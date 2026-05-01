#![cfg(not(tarpaulin_include))]
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
    let to_secs = step.timeout_seconds.unwrap_or(300); // 5 mins default

    let mut cmd = Command::new(&step.command);
    if let Some(args) = &step.args {
        cmd.args(args);
    }
    cmd.current_dir(dir);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let child = cmd
        .spawn()
        .map_err(|e| BridleError::Generic(e.to_string()))?;

    let res = timeout(Duration::from_secs(to_secs), child.wait_with_output()).await;

    match res {
        Ok(Ok(output)) => {
            let code = output.status.code().unwrap_or(1);
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Ok((code, stdout, stderr))
        }
        Ok(Err(e)) => Err(BridleError::Generic(e.to_string())),
        Err(_) => Err(BridleError::Generic("Timeout occurred".to_string())),
    }
}

/// Runs the execution engine for a repo workspace.
#[tracing::instrument(skip(dir, config), fields(pipeline = %config.name))]
pub async fn run_engine(dir: &Path, config: &PipelineConfig) -> Result<TaskStatus, BridleError> {
    for step in &config.steps {
        if step.step_type == StepType::Detect {
            let (code, _, _) = execute_step(dir, step).await?;
            if code == 0 {
                // Clean. No issue detected.
                return Ok(TaskStatus::Clean);
            }
        } else if step.step_type == StepType::MkconfBuild {
            // Generate Dockerfiles and build using mkconf
            let (code, _stdout, _stderr) = execute_step(dir, step).await?;
            if code != 0 {
                // Build failed, abort pipeline for this repo
                return Ok(TaskStatus::FailedValidation);
            }
            // If successful, we proceed to next step
        } else if step.step_type == StepType::Fix {
            let _ = execute_step(dir, step).await?;
            // Detect changes
            let status = std::process::Command::new("git")
                .env_remove("GIT_DIR")
                .env_remove("GIT_WORK_TREE")
                .env_remove("GIT_INDEX_FILE")
                .current_dir(dir)
                .args(["diff", "--quiet"])
                .status()
                .map_err(|e| BridleError::Generic(e.to_string()))?;

            if status.success() {
                // No changes
                return Ok(TaskStatus::Clean);
            }
        } else if step.step_type == StepType::Validate {
            let (code, _, _stderr) = execute_step(dir, step).await?;
            if code != 0 {
                // Rollback
                let _ = std::process::Command::new("git")
                    .env_remove("GIT_DIR")
                    .env_remove("GIT_WORK_TREE")
                    .env_remove("GIT_INDEX_FILE")
                    .current_dir(dir)
                    .args(["reset", "--hard"])
                    .status();
                let _ = std::process::Command::new("git")
                    .env_remove("GIT_DIR")
                    .env_remove("GIT_WORK_TREE")
                    .env_remove("GIT_INDEX_FILE")
                    .current_dir(dir)
                    .args(["clean", "-fd"])
                    .status();
                return Ok(TaskStatus::FailedValidation);
            }
        }
    }

    Ok(TaskStatus::PRSubmitted) // Next step is PR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_step_success() {
        let dir = std::env::current_dir().unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        let step = Step {
            name: "test".to_string(),
            step_type: StepType::Detect,
            command: "echo".to_string(),
            args: Some(vec!["hello".to_string()]),
            timeout_seconds: Some(10),
            expected_exit_codes: None,
        };

        let (code, stdout, _) = execute_step(&dir, &step)
            .await
            .unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        assert_eq!(code, 0);
        assert!(stdout.contains("hello"));
    }

    #[tokio::test]
    async fn test_execute_step_timeout() {
        let dir = std::env::current_dir().unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        let step = Step {
            name: "test_timeout".to_string(),
            step_type: StepType::Detect,
            command: "sleep".to_string(),
            args: Some(vec!["2".to_string()]),
            timeout_seconds: Some(1),
            expected_exit_codes: None,
        };

        let err = execute_step(&dir, &step).await;
        assert!(err.is_err());
        if let Err(BridleError::Generic(msg)) = err {
            assert_eq!(msg, "Timeout occurred");
        } else {
            panic!("Expected Timeout occurred");
        }
    }

    #[tokio::test]
    async fn test_execute_step_spawn_fail() {
        let dir = std::env::current_dir().unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        let step = Step {
            name: "fail".to_string(),
            step_type: StepType::Detect,
            command: "nonexistent_command_123".to_string(),
            args: None,
            timeout_seconds: None,
            expected_exit_codes: None,
        };

        let err = execute_step(&dir, &step).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_run_engine_detect_clean() {
        let dir = std::env::current_dir().unwrap_or_else(|e| panic!("must succeed: {:?}", e));
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

        let status = run_engine(&dir, &config)
            .await
            .unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        assert_eq!(status, TaskStatus::Clean);
    }

    #[tokio::test]
    async fn test_run_engine_mkconf_fail() {
        let dir = std::env::current_dir().unwrap_or_else(|e| panic!("must succeed: {:?}", e));
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
                name: "mkconf".to_string(),
                step_type: StepType::MkconfBuild,
                command: "false".to_string(),
                args: None,
                timeout_seconds: None,
                expected_exit_codes: None,
            }],
        };

        let status = run_engine(&dir, &config)
            .await
            .unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        assert_eq!(status, TaskStatus::FailedValidation);
    }

    #[tokio::test]
    async fn test_run_engine_fix_clean() {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        std::process::Command::new("git")
            .arg("init")
            .current_dir(dir.path())
            .status()
            .unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        std::process::Command::new("git")
            .args(["commit", "--allow-empty", "-m", "init"])
            .current_dir(dir.path())
            .status()
            .unwrap_or_else(|e| panic!("must succeed: {:?}", e));

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
                command: "true".to_string(), // dummy command, does not change git status
                args: None,
                timeout_seconds: None,
                expected_exit_codes: None,
            }],
        };

        let status = run_engine(dir.path(), &config)
            .await
            .unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        assert_eq!(status, TaskStatus::Clean);
    }

    #[tokio::test]
    async fn test_run_engine_validate_fail() {
        let dir = tempfile::tempdir().unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        std::process::Command::new("git")
            .arg("init")
            .current_dir(dir.path())
            .status()
            .unwrap_or_else(|e| panic!("must succeed: {:?}", e));

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
                name: "validate".to_string(),
                step_type: StepType::Validate,
                command: "false".to_string(),
                args: None,
                timeout_seconds: None,
                expected_exit_codes: None,
            }],
        };

        let status = run_engine(dir.path(), &config)
            .await
            .unwrap_or_else(|e| panic!("must succeed: {:?}", e));
        assert_eq!(status, TaskStatus::FailedValidation);
    }
}
