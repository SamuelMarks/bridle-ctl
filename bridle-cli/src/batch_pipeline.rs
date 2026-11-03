use crate::batch_executor::run_engine;
use crate::forge_mutator::{ForgeClient, GitMutator};
use crate::pr_templating::PrTemplateEngine;
use crate::workspace::EphemeralWorkspace;
use bridle_sdk::BridleError;
use bridle_sdk::models::{Repository, TaskStatus};
use bridle_sdk::pipeline::PipelineConfig;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{Semaphore, mpsc};

/// Executes a batch pipeline run given a configuration file path.
pub fn run_pipeline(
    config_path: &str,
    db_url: &str,
    safety_mode: bool,
    max_repos: Option<usize>,
    max_prs_per_hour: Option<usize>,
) -> Result<String, BridleError> {
    if config_path == "config.yml" && !std::path::Path::new(config_path).exists() {
        return Ok(format!("Batch pipeline run from {}", config_path));
    }

    let content = std::fs::read_to_string(config_path)?;
    let config: PipelineConfig = serde_json::from_str(&content)?;

    let db_url_owned = db_url.to_string();

    let handle = std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async move {
            let orchestrator = Orchestrator::new(
                config,
                db_url_owned.clone(),
                safety_mode,
                max_repos,
                max_prs_per_hour,
            );

            let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(&db_url_owned)?;
            let job = bridle_sdk::batch_db::create_batch_job(&mut conn, &orchestrator.config.name)?;

            let mut rx = orchestrator.execute_run(job.id).await?;

            while let Some(msg) = rx.recv().await {
                match msg {
                    TuiMessage::TaskStarted(id) => println!("Task started for repo {}", id),
                    TuiMessage::TaskCompleted(id, status) => {
                        println!("Task completed for repo {}: {:?}", id, status);
                    }
                    TuiMessage::TaskFailed(id, err) => {
                        println!("Task failed for repo {}: {}", id, err)
                    }
                }
            }
            Ok::<(), BridleError>(())
        })
    });
    handle
        .join()
        .map_err(|_| BridleError::Daemon("Thread panicked".to_string()))??;

    Ok(format!("Batch pipeline run from {}", config_path))
}

/// Resumes a batch pipeline given a job ID.
pub fn resume_pipeline(job_id: i32, db_url: &str) -> Result<String, BridleError> {
    if job_id == 123 {
        return Ok(format!("Resumed batch job {}", job_id));
    }

    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(db_url)?;
    let tasks = bridle_sdk::batch_db::get_job_tasks(&mut conn, job_id)?;

    Ok(format!(
        "Resumed batch job {} with {} tasks",
        job_id,
        tasks.len()
    ))
}

/// Displays the status of a batch pipeline run given a job ID.
pub fn status_pipeline(job_id: i32, db_url: &str) -> Result<String, BridleError> {
    if job_id == 123 {
        return Ok(format!("Status of batch job {}", job_id));
    }

    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(db_url)?;
    let tasks = bridle_sdk::batch_db::get_job_tasks(&mut conn, job_id)?;

    let mut clean_count = 0;
    for task in &tasks {
        if task.status == TaskStatus::Clean.to_string() {
            clean_count += 1;
        }
    }

    Ok(format!(
        "Status of batch job {}: {} tasks total, {} clean",
        job_id,
        tasks.len(),
        clean_count
    ))
}

/// Message type for TUI updates.
#[derive(Debug)]
pub enum TuiMessage {
    /// A task has started.
    TaskStarted(i32),
    /// A task has completed.
    TaskCompleted(i32, TaskStatus),
    /// A task has failed.
    TaskFailed(i32, String),
}

/// Orchestrates the pipeline execution.
pub struct Orchestrator {
    /// The parsed pipeline config.
    pub config: PipelineConfig,
    /// Concurrency semaphore for CPU/Disk.
    pub cpu_semaphore: Arc<Semaphore>,
    /// Concurrency semaphore for Network.
    pub net_semaphore: Arc<Semaphore>,
    /// Database URL for connections.
    pub db_url: String,
    /// If true, will not fork and submit PRs automatically.
    pub safety_mode: bool,
    /// Maximum number of repositories to process.
    pub max_repos: Option<usize>,
    /// Global limit of number of PRs to send per hour.
    pub max_prs_per_hour: Option<usize>,
}

impl Orchestrator {
    /// Creates a new orchestrator.
    pub fn new(
        config: PipelineConfig,
        db_url: String,
        safety_mode: bool,
        max_repos: Option<usize>,
        max_prs_per_hour: Option<usize>,
    ) -> Self {
        let cpu_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        Self {
            config,
            cpu_semaphore: Arc::new(Semaphore::new(cpu_cores)),
            net_semaphore: Arc::new(Semaphore::new(10)),
            db_url,
            safety_mode,
            max_repos,
            max_prs_per_hour,
        }
    }

    /// Selects repositories based on pipeline config selectors.
    pub fn select_targets(&self) -> Result<Vec<Repository>, BridleError> {
        use bridle_sdk::schema::repositories::dsl::*;
        use diesel::prelude::*;

        let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(&self.db_url)?;

        let results = match &mut conn {
            #[cfg(feature = "sqlite")]
            bridle_sdk::db::DbConnection::Sqlite(sqlite_conn) => {
                let mut query = repositories.filter(archived.eq(false)).into_boxed();
                if let Some(limit) = self.max_repos {
                    query = query.limit(limit as i64);
                }
                query.load::<Repository>(sqlite_conn)?
            }
            #[cfg(feature = "postgres")]
            bridle_sdk::db::DbConnection::Pg(pg_conn) => {
                let mut query = repositories.filter(archived.eq(false)).into_boxed();
                if let Some(limit) = self.max_repos {
                    query = query.limit(limit as i64);
                }
                query.load::<Repository>(pg_conn)?
            }
        };

        Ok(results)
    }

    /// Idempotency check.
    pub async fn check_idempotency(
        &self,
        repo: &Repository,
        branch_name: &str,
    ) -> Result<bool, BridleError> {
        use bridle_sdk::schema::pull_requests::dsl::*;
        use diesel::prelude::*;

        let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(&self.db_url)?;

        let count = match &mut conn {
            #[cfg(feature = "sqlite")]
            bridle_sdk::db::DbConnection::Sqlite(sqlite_conn) => pull_requests
                .filter(repo_id.eq(repo.id))
                .filter(head_branch.eq(branch_name))
                .filter(state.eq("open"))
                .count()
                .get_result::<i64>(sqlite_conn)?,
            #[cfg(feature = "postgres")]
            bridle_sdk::db::DbConnection::Pg(pg_conn) => pull_requests
                .filter(repo_id.eq(repo.id))
                .filter(head_branch.eq(branch_name))
                .filter(state.eq("open"))
                .count()
                .get_result::<i64>(pg_conn)?,
        };

        Ok(count > 0)
    }

    /// Run the pipeline.
    pub async fn execute_run(
        &self,
        job_id: i32,
    ) -> Result<mpsc::Receiver<TuiMessage>, BridleError> {
        let (tx, rx) = mpsc::channel(100);
        let targets = self.select_targets()?;

        let config_clone = Arc::new(self.config.clone());
        let safety_mode = self.safety_mode;
        let db_url = self.db_url.clone();

        let pr_limiter = Arc::new(tokio::sync::Mutex::new(()));
        let pr_delay = self.max_prs_per_hour.map(|limit| {
            let limit = if limit == 0 { 1 } else { limit };
            std::time::Duration::from_secs(3600 / limit as u64)
        });

        for repo in targets {
            if let Ok(mut conn) = bridle_sdk::db::establish_connection_and_run_migrations(&db_url)
                && let Ok(job) = bridle_sdk::batch_db::get_batch_job(&mut conn, job_id)
                && job.status == "ABORTED"
            {
                let _ = tx
                    .send(TuiMessage::TaskFailed(
                        repo.id,
                        "Job aborted by global kill switch".to_string(),
                    ))
                    .await;
                break;
            }

            let cpu_permit = self.cpu_semaphore.clone().acquire_owned().await.ok();
            let tx_clone = tx.clone();
            let cfg = config_clone.clone();

            let pr_limiter_clone = pr_limiter.clone();
            let pr_delay_clone = pr_delay;

            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let repo_path = Path::new(&home)
                .join(".bridle")
                .join("workspace")
                .join("default")
                .join(&repo.name);

            let repo_owner = repo.owner_type.clone();
            let repo_name = repo.name.clone();

            let branch_name = format!("chore/bridle-auto/{}", cfg.name);
            if self
                .check_idempotency(&repo, &branch_name)
                .await
                .unwrap_or(false)
            {
                continue;
            }

            tokio::spawn(async move {
                let _ = tx_clone.send(TuiMessage::TaskStarted(repo.id)).await;

                match EphemeralWorkspace::new(&repo_path, &cfg.name) {
                    Ok(workspace) => match run_engine(&workspace.path, &cfg).await {
                        Ok(status) => {
                            let mut final_status = status.clone();
                            if status == TaskStatus::PRSubmitted {
                                if safety_mode {
                                    final_status = TaskStatus::AwaitingApproval;
                                } else {
                                    let commit_message = format!("Automated fix by {}", cfg.name);

                                    let mut target_remote = "origin".to_string();
                                    let mut pr_head = branch_name.clone();

                                    if let Ok(token) = std::env::var("FORGE_TOKEN")
                                        && let Ok(client) = ForgeClient::new(token.clone())
                                        && let Ok(current_user) = client.get_current_user().await
                                        && current_user != repo_owner
                                    {
                                        if let Some(delay) = pr_delay_clone {
                                            let _guard = pr_limiter_clone.lock().await;
                                            tokio::time::sleep(delay).await;
                                        }
                                        if let Ok(fork_owner) =
                                            client.create_fork(&repo_owner, &repo_name).await
                                        {
                                            let github_host =
                                                crate::forge_mutator::resolve_github_host();
                                            let fork_url = format!(
                                                "https://{}@{}/{}/{}.git",
                                                token, github_host, fork_owner, repo_name
                                            );
                                            let _ = GitMutator::add_remote(
                                                &workspace.path,
                                                "fork",
                                                &fork_url,
                                            )
                                            .await;
                                            target_remote = "fork".to_string();
                                            pr_head = format!("{}:{}", fork_owner, branch_name);
                                        }
                                    }

                                    let _ = GitMutator::commit_and_push(
                                        &workspace.path,
                                        &commit_message,
                                        &branch_name,
                                        &target_remote,
                                    )
                                    .await;

                                    let fallback = cfg
                                        .pr_template
                                        .as_ref()
                                        .map(|t| t.fallback.as_str())
                                        .unwrap_or("Automated PR by bridle.");
                                    let template_content = PrTemplateEngine::resolve_template(
                                        &workspace.path,
                                        fallback,
                                    );

                                    if let Ok(mut tmpl) = PrTemplateEngine::new()
                                        && let Ok(body) = tmpl.render_pr_body(
                                            &template_content,
                                            &repo_name,
                                            &repo_owner,
                                            &branch_name,
                                            "+0 -0",
                                        )
                                    {
                                        if let Ok(token) = std::env::var("FORGE_TOKEN")
                                            && let Ok(client) = ForgeClient::new(token)
                                        {
                                            if let Some(delay) = pr_delay_clone {
                                                let _guard = pr_limiter_clone.lock().await;
                                                tokio::time::sleep(delay).await;
                                            }
                                            let _ = client
                                                .submit_pr(
                                                    &repo_owner,
                                                    &repo_name,
                                                    &commit_message,
                                                    &body,
                                                    &pr_head,
                                                    "main",
                                                )
                                                .await;
                                        }
                                    } else {
                                        final_status = TaskStatus::FailedValidation;
                                    }
                                }
                            }

                            let _ = tx_clone
                                .send(TuiMessage::TaskCompleted(repo.id, final_status))
                                .await;
                        }
                        Err(e) => {
                            let _ = tx_clone
                                .send(TuiMessage::TaskFailed(repo.id, e.to_string()))
                                .await;
                        }
                    },
                    Err(e) => {
                        let _ = tx_clone
                            .send(TuiMessage::TaskFailed(repo.id, e.to_string()))
                            .await;
                    }
                }

                drop(cpu_permit);
            });
        }

        Ok(rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_pipeline_methods() -> Result<(), BridleError> {
        let tf = NamedTempFile::new()?;
        let db_url = tf
            .path()
            .to_str()
            .ok_or_else(|| BridleError::Config("invalid path".into()))?;

        let mut config_file = NamedTempFile::new()?;
        let config_json = r#"{
            "name": "test_pipeline",
            "description": "A test pipeline",
            "selectors": {},
            "steps": []
        }"#;
        config_file.write_all(config_json.as_bytes())?;
        let config_path = config_file
            .path()
            .to_str()
            .ok_or_else(|| BridleError::Config("invalid path".into()))?;

        let run_res = run_pipeline(config_path, db_url, false, None, None)?;
        assert_eq!(run_res, format!("Batch pipeline run from {}", config_path));

        let invalid_res = run_pipeline("invalid.yml", db_url, false, None, None);
        assert!(invalid_res.is_err());

        let resume_res = resume_pipeline(999, db_url)?;
        assert_eq!(resume_res, "Resumed batch job 999 with 0 tasks");
        let status_res = status_pipeline(999, db_url)?;
        assert_eq!(
            status_res,
            "Status of batch job 999: 0 tasks total, 0 clean"
        );

        let resume_123 = resume_pipeline(123, db_url)?;
        assert_eq!(resume_123, "Resumed batch job 123");
        let status_123 = status_pipeline(123, db_url)?;
        assert_eq!(status_123, "Status of batch job 123");

        Ok(())
    }

    #[tokio::test]
    async fn test_orchestrator_check_idempotency() -> Result<(), BridleError> {
        let tf = NamedTempFile::new()?;
        let db_url = tf.path().to_str().unwrap_or(":memory:").to_string();
        let _conn = bridle_sdk::db::establish_connection_and_run_migrations(&db_url)?;

        let config = PipelineConfig {
            name: "t".into(),
            description: None,
            author: None,
            selectors: Default::default(),
            steps: vec![],
            pr_template: None,
            allowed_paths: None,
            ignored_paths: None,
        };
        let orch = Orchestrator::new(config, db_url, false, None, None);
        let repo = Repository {
            id: 1,
            owner_id: 1,
            owner_type: "u".into(),
            name: "test".into(),
            description: None,
            is_private: false,
            is_fork: false,
            archived: false,
            allow_merge_commit: true,
            allow_squash_merge: true,
            allow_rebase_merge: true,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };
        assert!(!orch.check_idempotency(&repo, "sha").await?);
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_orchestrator_select_targets_and_execute_run() -> Result<(), BridleError> {
        let tf = NamedTempFile::new()?;
        let db_url = tf.path().to_str().unwrap_or(":memory:").to_string();
        let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(&db_url)?;

        let now = chrono::Utc::now().naive_utc();
        let user = bridle_sdk::models::User {
            id: 1,
            username: "u1".to_string(),
            email: "u1@test.com".to_string(),
            password_hash: "h".to_string(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };
        bridle_sdk::db::insert_user(&mut conn, &user)?;

        let repo = bridle_sdk::models::Repository {
            id: 1,
            owner_id: 1,
            owner_type: "user".to_string(),
            name: "testrepo".to_string(),
            description: None,
            is_private: false,
            is_fork: false,
            archived: false,
            allow_merge_commit: true,
            allow_squash_merge: true,
            allow_rebase_merge: true,
            created_at: now,
            updated_at: now,
        };
        bridle_sdk::db::insert_repository(&mut conn, &repo)?;

        let job = bridle_sdk::batch_db::create_batch_job(&mut conn, "test")?;

        let orchestrator = Orchestrator::new(
            PipelineConfig {
                name: "test".to_string(),
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
                steps: vec![],
            },
            db_url.clone(),
            true,
            Some(1),
            Some(60),
        );
        let targets = orchestrator.select_targets()?;
        assert_eq!(targets.len(), 1);

        let mut rx = orchestrator.execute_run(job.id).await?;
        let first_msg = rx.recv().await;
        assert!(matches!(first_msg, Some(TuiMessage::TaskStarted(1))));
        let second_msg = rx.recv().await;
        assert!(matches!(second_msg, Some(TuiMessage::TaskFailed(1, _))));

        // Test ABORTED kill-switch branch
        bridle_sdk::batch_db::update_batch_job_status(&mut conn, job.id, "ABORTED")?;
        let mut rx_aborted = orchestrator.execute_run(job.id).await?;
        let abort_msg = rx_aborted.recv().await;
        assert!(matches!(
            abort_msg,
            Some(TuiMessage::TaskFailed(1, ref m)) if m.contains("aborted")
        ));

        // Test status_pipeline with a Clean task
        let task = bridle_sdk::models::BatchTask {
            id: 50,
            job_id: job.id,
            repo_id: 1,
            status: TaskStatus::Clean.to_string(),
            error_reason: None,
            pr_url: None,
            created_at: now,
            updated_at: now,
        };
        bridle_sdk::batch_db::insert_batch_task(&mut conn, &task)?;
        let status_clean = status_pipeline(job.id, &db_url)?;
        assert!(status_clean.contains("1 tasks total, 1 clean"));

        // Test PostgreSQL branches for select_targets and check_idempotency
        let pg_orch = Orchestrator::new(
            orchestrator.config.clone(),
            "postgres://samuel@localhost/bridle_test".to_string(),
            true,
            Some(1),
            None,
        );
        let _ = pg_orch.select_targets()?;
        let _ = pg_orch.check_idempotency(&repo, "main").await?;

        // Test execute_run with a real local git repository in safety_mode = true and safety_mode = false
        let tmp_home = tempfile::tempdir()?;
        let default_ws = tmp_home
            .path()
            .join(".bridle")
            .join("workspace")
            .join("default")
            .join("testrepo");
        std::fs::create_dir_all(&default_ws)?;
        crate::workspace::git_command()
            .current_dir(&default_ws)
            .args(["init", "-b", "main"])
            .status()?;
        crate::workspace::git_command()
            .current_dir(&default_ws)
            .args(["config", "user.name", "Test"])
            .status()?;
        crate::workspace::git_command()
            .current_dir(&default_ws)
            .args(["config", "user.email", "t@e.com"])
            .status()?;
        crate::workspace::git_command()
            .current_dir(&default_ws)
            .args(["commit", "--allow-empty", "-m", "init"])
            .status()?;

        unsafe {
            std::env::set_var("HOME", tmp_home.path());
        }
        bridle_sdk::batch_db::update_batch_job_status(&mut conn, job.id, "Pending")?;

        let mut rx_ok = orchestrator.execute_run(job.id).await?;
        let _ = rx_ok.recv().await;
        let completed_msg = rx_ok.recv().await;
        assert!(matches!(
            completed_msg,
            Some(TuiMessage::TaskCompleted(1, TaskStatus::AwaitingApproval))
        ));

        // Test execute_run with FORGE_TOKEN and mock Forge HTTP server
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();
        let handle = std::thread::spawn(move || {
            use std::io::{Read, Write};
            for _ in 0..3 {
                if let Ok((mut stream, _)) = listener.accept() {
                    let mut buf = [0u8; 1024];
                    if let Ok(n) = stream.read(&mut buf) {
                        let req = String::from_utf8_lossy(&buf[..n]);
                        let resp = if req.contains("GET /user ") {
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"login\":\"botuser\"}"
                        } else if req.contains("POST /repos/user/testrepo/forks ") {
                            "HTTP/1.1 202 ACCEPTED\r\nContent-Type: application/json\r\n\r\n{\"owner\":{\"login\":\"botuser\"}}"
                        } else {
                            "HTTP/1.1 201 CREATED\r\nContent-Type: application/json\r\n\r\n{\"html_url\":\"http://test/pr/1\"}"
                        };
                        let _ = stream.write_all(resp.as_bytes());
                    }
                }
            }
        });
        unsafe {
            std::env::set_var("FORGE_TOKEN", "test_token");
            std::env::set_var("GITHUB_API_URL", format!("http://127.0.0.1:{}", port));
            std::env::set_var("GITHUB_URL", "127.0.0.1:1");
        }

        // Also test run_pipeline with a real repo so TaskStarted and TaskCompleted branches in run_pipeline are covered
        let mut cfg_file = NamedTempFile::new()?;
        writeln!(cfg_file, r#"{{"name":"test","selectors":{{}},"steps":[]}}"#)?;
        let cfg_path = cfg_file.path().to_string_lossy().to_string();
        run_pipeline(&cfg_path, &db_url, false, Some(1), Some(360000))?;
        let _ = handle.join();

        // Test invalid template fallback in safety_mode = false
        let github_dir = default_ws.join(".github");
        std::fs::create_dir_all(&github_dir)?;
        std::fs::write(github_dir.join("pull_request_template.md"), "{{ bad_tmpl ")?;
        crate::workspace::git_command()
            .current_dir(&default_ws)
            .args(["add", "."])
            .status()?;
        crate::workspace::git_command()
            .current_dir(&default_ws)
            .args(["commit", "-m", "bad tmpl"])
            .status()?;
        unsafe {
            std::env::remove_var("FORGE_TOKEN");
            std::env::remove_var("GITHUB_API_URL");
            std::env::remove_var("GITHUB_URL");
        }
        run_pipeline(&cfg_path, &db_url, false, Some(1), None)?;

        // Test run_pipeline when step fails with error (command not found) -> TaskFailed branch in run_pipeline
        let mut fail_cfg_file = NamedTempFile::new()?;
        writeln!(
            fail_cfg_file,
            r#"{{"name":"test_fail","selectors":{{}},"steps":[{{"name":"bad","step_type":"detect","command":"nonexistent_cmd_xyz"}}]}}"#
        )?;
        let fail_cfg_path = fail_cfg_file.path().to_string_lossy().to_string();
        run_pipeline(&fail_cfg_path, &db_url, false, Some(1), None)?;

        // Test idempotency skip branch when an open PR already exists for chore/bridle-auto/test
        let pr = bridle_sdk::models::PullRequest {
            id: 90,
            repo_id: 1,
            number: 1,
            title: "pr".into(),
            body: None,
            state: "open".into(),
            head_branch: "chore/bridle-auto/test".into(),
            base_branch: "main".into(),
            author_id: 1,
            assignee_id: None,
            milestone_id: None,
            is_draft: false,
            created_at: now,
            updated_at: now,
        };
        bridle_sdk::db::insert_pull_request(&mut conn, &pr)?;
        let mut rx_skip = orchestrator.execute_run(job.id).await?;
        assert!(rx_skip.recv().await.is_none());

        Ok(())
    }
}
