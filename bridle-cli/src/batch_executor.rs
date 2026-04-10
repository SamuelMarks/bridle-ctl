use crate::error::CliError;
use bridle_sdk::models::TaskStatus;
use bridle_sdk::pipeline::{PipelineConfig, Step, StepType};
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// Execute a single step with timeout.
#[tracing::instrument(skip(dir, step), fields(step_name = %step.name, step_type = ?step.step_type))]
#[cfg(not(tarpaulin_include))]
pub async fn execute_step(dir: &Path, step: &Step) -> Result<(i32, String, String), CliError> {
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
        .map_err(|e| CliError::Execution(e.to_string()))?;

    let res = timeout(Duration::from_secs(to_secs), child.wait_with_output()).await;

    match res {
        Ok(Ok(output)) => {
            let code = output.status.code().unwrap_or(1);
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Ok((code, stdout, stderr))
        }
        Ok(Err(e)) => Err(CliError::Execution(e.to_string())),
        Err(_) => Err(CliError::Execution("Timeout occurred".to_string())),
    }
}

/// Runs the execution engine for a repo workspace.
#[tracing::instrument(skip(dir, config), fields(pipeline = %config.name))]
#[cfg(not(tarpaulin_include))]
pub async fn run_engine(dir: &Path, config: &PipelineConfig) -> Result<TaskStatus, CliError> {
    for step in &config.steps {
        if step.step_type == StepType::Detect {
            let (code, _, _) = execute_step(dir, step).await?;
            if code == 0 {
                // Clean. No issue detected.
                return Ok(TaskStatus::Clean);
            }
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
                .map_err(|e| CliError::Execution(e.to_string()))?;

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
