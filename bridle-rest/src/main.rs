#![deny(missing_docs)]
#![warn(missing_docs)]
//! REST API Interface for bridle-ctl.

#[cfg(not(test))]
use actix_web::{App, HttpServer};
use actix_web::{HttpResponse, Responder, web};
use bridle_sdk::BridleError;

/// Shared application state.
pub struct AppState {
    /// Database URL for connections.
    pub db_url: String,
}

/// Request payload for ingesting an organization via REST.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct IngestOrgRequest {
    /// Organization name.
    pub org: String,
    /// Upstream provider (e.g. "github").
    pub provider: String,
    /// Optional database URL override.
    #[serde(rename = "dbUrl")]
    pub db_url: Option<String>,
}

/// Request payload for executing a CLI database action via REST.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DevDbRequest {
    /// Action name (e.g. "get_user", "create_user").
    pub action: String,
    /// Optional numeric ID.
    pub id: Option<i32>,
    /// Optional JSON payload object.
    pub payload: Option<serde_json::Value>,
}

/// Health check endpoint to ensure the server is running.
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy")
}

/// System health endpoint returning JSON status for the UI dashboard.
pub async fn api_health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "rest": "UP",
        "rpc": "UP",
        "agent": "UP"
    }))
}

/// Endpoint to start the agent.
pub async fn start_agent() -> Result<HttpResponse, BridleError> {
    let msg = bridle_agent::start_agent()?;
    Ok(HttpResponse::Ok().body(msg))
}

/// Endpoint to run tools using ToolRunRequest.
pub async fn run_tools(
    req: web::Json<bridle_sdk::models::ToolRunRequest>,
) -> Result<HttpResponse, BridleError> {
    let payload = req.into_inner();
    let action = match payload.action.as_deref() {
        Some("audit") => bridle_cli::runner::Action::Audit,
        _ => {
            let is_dry_run = matches!(payload.dry_run, Some(true));
            bridle_cli::runner::Action::Fix {
                dry_run: is_dry_run,
            }
        }
    };

    bridle_cli::runner::run(action, payload)?;
    Ok(HttpResponse::Ok().body("Tools executed successfully"))
}

/// Endpoint to run batch pipeline.
pub async fn batch_run(
    data: web::Data<AppState>,
    req: web::Json<bridle_sdk::models::BatchRunRequest>,
) -> Result<HttpResponse, BridleError> {
    let payload = req.into_inner();
    let msg = bridle_cli::batch_pipeline::run_pipeline(
        &payload.config_path,
        &data.db_url,
        payload.safety_mode,
        payload.max_repos,
        payload.max_prs_per_hour,
    )?;
    Ok(HttpResponse::Ok().body(msg))
}

/// Endpoint to run batch fix across an organization.
pub async fn batch_fix_endpoint(
    data: web::Data<AppState>,
    req: web::Json<bridle_sdk::models::BatchFixRequest>,
) -> Result<HttpResponse, BridleError> {
    let payload = req.into_inner();
    let msg = bridle_cli::batch_fix::batch_fix(
        &payload.org,
        &payload.issue,
        payload.pattern,
        payload.tools,
        payload.tool_args,
        &data.db_url,
        payload.safety_mode,
        payload.max_repos,
        payload.max_prs_per_hour,
    )?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": msg })))
}

/// Endpoint to resume a batch job by ID.
pub async fn batch_resume_endpoint(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, BridleError> {
    let job_id = path.into_inner();
    let msg = bridle_cli::batch_pipeline::resume_pipeline(job_id, &data.db_url)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "id": job_id, "message": msg })))
}

/// Endpoint to sync pull requests.
pub async fn sync_prs(
    data: web::Data<AppState>,
    req: web::Json<bridle_sdk::models::SyncPrsRequest>,
) -> Result<HttpResponse, BridleError> {
    let payload = req.into_inner();
    let msg = bridle_cli::sync_prs::sync_prs(
        &payload.org,
        &data.db_url,
        payload.max_prs_per_hour,
        payload.fork_org,
    )?;
    Ok(HttpResponse::Ok().body(msg))
}

/// Endpoint to ingest an organization via REST.
pub async fn ingest_org_endpoint(
    data: web::Data<AppState>,
    req: web::Json<IngestOrgRequest>,
) -> Result<HttpResponse, BridleError> {
    let payload = req.into_inner();
    let target_db = payload.db_url.unwrap_or_else(|| data.db_url.clone());
    let handle = std::thread::spawn(move || {
        bridle_cli::ingest::ingest_org(&payload.org, &payload.provider, &target_db)
    });
    let msg = handle
        .join()
        .map_err(|_| BridleError::Daemon("Ingest thread panicked".to_string()))??;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "message": msg })))
}

/// Endpoint to execute a database command from the Dev Tools UI.
pub async fn dev_db_endpoint(
    data: web::Data<AppState>,
    req: web::Json<DevDbRequest>,
) -> Result<HttpResponse, BridleError> {
    let payload = req.into_inner();
    let payload_str = payload.payload.map(|v| v.to_string());
    let out =
        bridle_cli::db::execute_db_command(&data.db_url, &payload.action, payload_str, payload.id)?;
    Ok(HttpResponse::Ok().json(serde_json::json!({ "result": out })))
}

/// Define CRUD endpoints for a specific model type.
macro_rules! define_crud_endpoints {
    ($get_fn:ident, $create_fn:ident, $sdk_get:path, $sdk_insert:path, $model:ty) => {
        /// Creates a new item in the database.
        pub async fn $create_fn(
            data: web::Data<AppState>,
            payload: web::Json<$model>,
        ) -> Result<HttpResponse, BridleError> {
            let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(&data.db_url)?;
            $sdk_insert(&mut conn, &payload.into_inner())?;
            Ok(HttpResponse::Created().finish())
        }

        /// Retrieves an item from the database.
        pub async fn $get_fn(
            data: web::Data<AppState>,
            path: web::Path<i32>,
        ) -> Result<HttpResponse, BridleError> {
            let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(&data.db_url)?;
            let id = path.into_inner();
            let item = $sdk_get(&mut conn, id)?;
            Ok(HttpResponse::Ok().json(item))
        }
    };
}

define_crud_endpoints!(
    get_user,
    create_user,
    bridle_sdk::db::get_user,
    bridle_sdk::db::insert_user,
    bridle_sdk::models::User
);
define_crud_endpoints!(
    get_org,
    create_org,
    bridle_sdk::db::get_organisation,
    bridle_sdk::db::insert_organisation,
    bridle_sdk::models::Organisation
);
define_crud_endpoints!(
    get_repo,
    create_repo,
    bridle_sdk::db::get_repository,
    bridle_sdk::db::insert_repository,
    bridle_sdk::models::Repository
);
define_crud_endpoints!(
    get_team,
    create_team,
    bridle_sdk::db::get_team,
    bridle_sdk::db::insert_team,
    bridle_sdk::models::Team
);
define_crud_endpoints!(
    get_branch,
    create_branch,
    bridle_sdk::db::get_branch,
    bridle_sdk::db::insert_branch,
    bridle_sdk::models::Branch
);
define_crud_endpoints!(
    get_branch_protection_rule,
    create_branch_protection_rule,
    bridle_sdk::db::get_branch_protection_rule,
    bridle_sdk::db::insert_branch_protection_rule,
    bridle_sdk::models::BranchProtectionRule
);
define_crud_endpoints!(
    get_key,
    create_key,
    bridle_sdk::db::get_key,
    bridle_sdk::db::insert_key,
    bridle_sdk::models::Key
);
define_crud_endpoints!(
    get_follow,
    create_follow,
    bridle_sdk::db::get_follow,
    bridle_sdk::db::insert_follow,
    bridle_sdk::models::Follow
);
define_crud_endpoints!(
    get_star,
    create_star,
    bridle_sdk::db::get_star,
    bridle_sdk::db::insert_star,
    bridle_sdk::models::Star
);
define_crud_endpoints!(
    get_org_membership,
    create_org_membership,
    bridle_sdk::db::get_org_membership,
    bridle_sdk::db::insert_org_membership,
    bridle_sdk::models::OrgMembership
);
define_crud_endpoints!(
    get_repo_collaborator,
    create_repo_collaborator,
    bridle_sdk::db::get_repo_collaborator,
    bridle_sdk::db::insert_repo_collaborator,
    bridle_sdk::models::RepoCollaborator
);
define_crud_endpoints!(
    get_milestone,
    create_milestone,
    bridle_sdk::db::get_milestone,
    bridle_sdk::db::insert_milestone,
    bridle_sdk::models::Milestone
);
define_crud_endpoints!(
    get_label,
    create_label,
    bridle_sdk::db::get_label,
    bridle_sdk::db::insert_label,
    bridle_sdk::models::Label
);
define_crud_endpoints!(
    get_issue,
    create_issue,
    bridle_sdk::db::get_issue,
    bridle_sdk::db::insert_issue,
    bridle_sdk::models::Issue
);
define_crud_endpoints!(
    get_issue_label,
    create_issue_label,
    bridle_sdk::db::get_issue_label,
    bridle_sdk::db::insert_issue_label,
    bridle_sdk::models::IssueLabel
);
define_crud_endpoints!(
    get_pull_request,
    create_pull_request,
    bridle_sdk::db::get_pull_request,
    bridle_sdk::db::insert_pull_request,
    bridle_sdk::models::PullRequest
);
define_crud_endpoints!(
    get_pull_request_review,
    create_pull_request_review,
    bridle_sdk::db::get_pull_request_review,
    bridle_sdk::db::insert_pull_request_review,
    bridle_sdk::models::PullRequestReview
);
define_crud_endpoints!(
    get_release,
    create_release,
    bridle_sdk::db::get_release,
    bridle_sdk::db::insert_release,
    bridle_sdk::models::Release
);
define_crud_endpoints!(
    get_webhook,
    create_webhook,
    bridle_sdk::db::get_webhook,
    bridle_sdk::db::insert_webhook,
    bridle_sdk::models::Webhook
);
define_crud_endpoints!(
    get_commit,
    create_commit,
    bridle_sdk::db::get_commit,
    bridle_sdk::db::insert_commit,
    bridle_sdk::models::Commit
);
define_crud_endpoints!(
    get_tree,
    create_tree,
    bridle_sdk::db::get_tree,
    bridle_sdk::db::insert_tree,
    bridle_sdk::models::Tree
);
define_crud_endpoints!(
    get_blob,
    create_blob,
    bridle_sdk::db::get_blob,
    bridle_sdk::db::insert_blob,
    bridle_sdk::models::Blob
);
define_crud_endpoints!(
    get_batch_job,
    create_batch_job,
    bridle_sdk::batch_db::get_batch_job,
    bridle_sdk::batch_db::insert_batch_job,
    bridle_sdk::models::BatchJob
);

/// Configures all REST routes on the provided `web::ServiceConfig`.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check))
        .route("/api/health", web::get().to(api_health_check))
        .route("/agent/start", web::post().to(start_agent))
        .route("/tools/run", web::post().to(run_tools))
        .route("/api/local/audit", web::post().to(run_tools))
        .route("/api/local/fix", web::post().to(run_tools))
        .route("/batch/run", web::post().to(batch_run))
        .route("/api/batch/run", web::post().to(batch_run))
        .route("/api/batch/fix", web::post().to(batch_fix_endpoint))
        .route("/api/batch/jobs", web::post().to(create_batch_job))
        .route("/api/batch/jobs/{id}", web::get().to(get_batch_job))
        .route(
            "/api/batch/jobs/{id}/resume",
            web::post().to(batch_resume_endpoint),
        )
        .route("/prs/sync", web::post().to(sync_prs))
        .route("/api/prs/sync", web::post().to(sync_prs))
        .route("/api/orgs/ingest", web::post().to(ingest_org_endpoint))
        .route("/api/dev/db", web::post().to(dev_db_endpoint))
        .route("/users", web::post().to(create_user))
        .route("/users/{id}", web::get().to(get_user))
        .route("/orgs", web::post().to(create_org))
        .route("/orgs/{id}", web::get().to(get_org))
        .route("/repos", web::post().to(create_repo))
        .route("/repos/{id}", web::get().to(get_repo))
        .route("/teams", web::post().to(create_team))
        .route("/teams/{id}", web::get().to(get_team))
        .route("/branches", web::post().to(create_branch))
        .route("/branches/{id}", web::get().to(get_branch))
        .route(
            "/branch_protection_rules",
            web::post().to(create_branch_protection_rule),
        )
        .route(
            "/branch_protection_rules/{id}",
            web::get().to(get_branch_protection_rule),
        )
        .route("/keys", web::post().to(create_key))
        .route("/keys/{id}", web::get().to(get_key))
        .route("/follows", web::post().to(create_follow))
        .route("/follows/{id}", web::get().to(get_follow))
        .route("/stars", web::post().to(create_star))
        .route("/stars/{id}", web::get().to(get_star))
        .route("/org_memberships", web::post().to(create_org_membership))
        .route("/org_memberships/{id}", web::get().to(get_org_membership))
        .route(
            "/repo_collaborators",
            web::post().to(create_repo_collaborator),
        )
        .route(
            "/repo_collaborators/{id}",
            web::get().to(get_repo_collaborator),
        )
        .route("/milestones", web::post().to(create_milestone))
        .route("/milestones/{id}", web::get().to(get_milestone))
        .route("/labels", web::post().to(create_label))
        .route("/labels/{id}", web::get().to(get_label))
        .route("/issues", web::post().to(create_issue))
        .route("/issues/{id}", web::get().to(get_issue))
        .route("/issue_labels", web::post().to(create_issue_label))
        .route("/issue_labels/{id}", web::get().to(get_issue_label))
        .route("/pull_requests", web::post().to(create_pull_request))
        .route("/pull_requests/{id}", web::get().to(get_pull_request))
        .route(
            "/pull_request_reviews",
            web::post().to(create_pull_request_review),
        )
        .route(
            "/pull_request_reviews/{id}",
            web::get().to(get_pull_request_review),
        )
        .route("/releases", web::post().to(create_release))
        .route("/releases/{id}", web::get().to(get_release))
        .route("/webhooks", web::post().to(create_webhook))
        .route("/webhooks/{id}", web::get().to(get_webhook))
        .route("/commits", web::post().to(create_commit))
        .route("/commits/{id}", web::get().to(get_commit))
        .route("/trees", web::post().to(create_tree))
        .route("/trees/{id}", web::get().to(get_tree))
        .route("/blobs", web::post().to(create_blob))
        .route("/blobs/{id}", web::get().to(get_blob));
}

/// Main entry point for the REST API.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(e) = bridle_sdk::telemetry::init_telemetry() {
        eprintln!("Warning: Failed to initialize telemetry: {}", e);
    }

    let db_url = bridle_sdk::db::database_url();
    let app_state = web::Data::new(AppState { db_url });

    #[cfg(not(test))]
    {
        HttpServer::new(move || {
            App::new()
                .app_data(app_state.clone())
                .configure(configure_routes)
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;
    }
    #[cfg(test)]
    let _ = app_state;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    use bridle_sdk::models::{Organisation, Repository, User};

    fn test_app_state() -> Result<(tempfile::NamedTempFile, web::Data<AppState>), std::io::Error> {
        let tf = tempfile::NamedTempFile::new()?;
        let db_url = tf.path().to_string_lossy().to_string();
        Ok((tf, web::Data::new(AppState { db_url })))
    }

    #[actix_web::test]
    async fn test_health_and_operational_endpoints() -> Result<(), std::io::Error> {
        let (_tf, state) = test_app_state()?;
        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let req_api_health = test::TestRequest::get().uri("/api/health").to_request();
        let resp_api_health = test::call_service(&app, req_api_health).await;
        assert!(resp_api_health.status().is_success());

        let req_agent = test::TestRequest::post().uri("/agent/start").to_request();
        let resp_agent = test::call_service(&app, req_agent).await;
        assert!(resp_agent.status().is_success());

        let payload_fix = bridle_sdk::models::ToolRunRequest {
            pattern: Some(r".*\.go$".to_string()),
            tools: Some(vec!["rust-unwrap-to-question-mark".to_string()]),
            tool_args: None,
            dry_run: Some(true),
            action: Some("fix".to_string()),
        };
        let req_fix = test::TestRequest::post()
            .uri("/tools/run")
            .set_json(&payload_fix)
            .to_request();
        let resp_fix = test::call_service(&app, req_fix).await;
        assert!(resp_fix.status().is_success());

        let payload_audit = bridle_sdk::models::ToolRunRequest {
            pattern: Some(r".*\.go$".to_string()),
            tools: Some(vec!["rust-unwrap-to-question-mark".to_string()]),
            tool_args: None,
            dry_run: None,
            action: Some("audit".to_string()),
        };
        let req_audit = test::TestRequest::post()
            .uri("/api/local/audit")
            .set_json(&payload_audit)
            .to_request();
        let resp_audit = test::call_service(&app, req_audit).await;
        assert!(resp_audit.status().is_success());

        let req_batch_ok = test::TestRequest::post()
            .uri("/api/batch/run")
            .set_json(bridle_sdk::models::BatchRunRequest {
                config_path: "config.yml".to_string(),
                safety_mode: true,
                max_repos: None,
                max_prs_per_hour: None,
            })
            .to_request();
        let resp_batch_ok = test::call_service(&app, req_batch_ok).await;
        assert!(resp_batch_ok.status().is_success());

        let req_batch_err = test::TestRequest::post()
            .uri("/batch/run")
            .set_json(bridle_sdk::models::BatchRunRequest {
                config_path: "nonexistent.yaml".to_string(),
                safety_mode: false,
                max_repos: None,
                max_prs_per_hour: None,
            })
            .to_request();
        let resp_batch_err = test::call_service(&app, req_batch_err).await;
        assert!(resp_batch_err.status().is_server_error());

        let req_batch_fix_err = test::TestRequest::post()
            .uri("/api/batch/fix")
            .set_json(bridle_sdk::models::BatchFixRequest {
                org: "nonexistent_org_rest".to_string(),
                issue: "fix".to_string(),
                pattern: None,
                tools: None,
                tool_args: None,
                safety_mode: true,
                max_repos: None,
                max_prs_per_hour: None,
            })
            .to_request();
        let resp_batch_fix_err = test::call_service(&app, req_batch_fix_err).await;
        assert_eq!(
            resp_batch_fix_err.status(),
            actix_web::http::StatusCode::BAD_REQUEST
        );

        let req_resume = test::TestRequest::post()
            .uri("/api/batch/jobs/123/resume")
            .to_request();
        let resp_resume = test::call_service(&app, req_resume).await;
        assert!(resp_resume.status().is_success());

        let req_sync = test::TestRequest::post()
            .uri("/api/prs/sync")
            .set_json(bridle_sdk::models::SyncPrsRequest {
                org: "test".to_string(),
                max_prs_per_hour: Some(0),
                fork_org: None,
            })
            .to_request();
        let resp_sync = test::call_service(&app, req_sync).await;
        assert!(resp_sync.status().is_success());

        let tmp_ws = tempfile::tempdir()?;
        let org_dir = tmp_ws
            .path()
            .join(".bridle")
            .join("workspace")
            .join("ok_org");
        let _ = std::fs::create_dir_all(&org_dir);
        unsafe {
            std::env::set_var("HOME", tmp_ws.path());
        }
        let req_batch_fix_ok = test::TestRequest::post()
            .uri("/api/batch/fix")
            .set_json(bridle_sdk::models::BatchFixRequest {
                org: "ok_org".to_string(),
                issue: "fix".to_string(),
                pattern: None,
                tools: None,
                tool_args: None,
                safety_mode: true,
                max_repos: None,
                max_prs_per_hour: None,
            })
            .to_request();
        let resp_batch_fix_ok = test::call_service(&app, req_batch_fix_ok).await;
        assert!(resp_batch_fix_ok.status().is_success());

        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();
        let handle = std::thread::spawn(move || {
            use std::io::{Read, Write};
            if let Ok((mut stream, _)) = listener.accept() {
                let mut buf = [0u8; 512];
                let _ = stream.read(&mut buf);
                let _ = stream
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n[]");
            }
        });
        unsafe {
            std::env::set_var("GITHUB_API_URL", format!("http://127.0.0.1:{}", port));
        }
        let req_ingest_ok = test::TestRequest::post()
            .uri("/api/orgs/ingest")
            .set_json(IngestOrgRequest {
                org: "ok_org".to_string(),
                provider: "github".to_string(),
                db_url: None,
            })
            .to_request();
        let resp_ingest_ok = test::call_service(&app, req_ingest_ok).await;
        assert!(resp_ingest_ok.status().is_success());
        let _ = handle.join();
        unsafe {
            std::env::remove_var("GITHUB_API_URL");
        }

        let req_dev_db = test::TestRequest::post()
            .uri("/api/dev/db")
            .set_json(DevDbRequest {
                action: "get_user".to_string(),
                id: Some(999),
                payload: None,
            })
            .to_request();
        let resp_dev_db = test::call_service(&app, req_dev_db).await;
        assert_eq!(resp_dev_db.status(), actix_web::http::StatusCode::NOT_FOUND);

        let now = chrono::Utc::now().naive_utc();
        let dev_user = User {
            id: 77,
            username: "devuser".to_string(),
            email: "dev@example.com".to_string(),
            password_hash: "h".to_string(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };
        let req_dev_create = test::TestRequest::post()
            .uri("/api/dev/db")
            .set_json(DevDbRequest {
                action: "create_user".to_string(),
                id: None,
                payload: serde_json::to_value(&dev_user).ok(),
            })
            .to_request();
        let resp_dev_create = test::call_service(&app, req_dev_create).await;
        assert!(resp_dev_create.status().is_success());

        let _ = std::thread::spawn(super::main).join();
        Ok(())
    }

    #[actix_web::test]
    async fn test_all_crud_endpoints_create_get_and_not_found() -> Result<(), actix_web::Error> {
        let (_tf, state) = test_app_state()?;
        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .configure(configure_routes),
        )
        .await;

        let now = chrono::Utc::now().naive_utc();

        // 1. User
        let new_user = User {
            id: 1,
            username: "tester".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };
        let resp = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/users")
                .set_json(&new_user)
                .to_request(),
        )
        .await;
        assert_eq!(resp.status(), actix_web::http::StatusCode::CREATED);

        let user2 = User {
            id: 2,
            username: "tester2".to_string(),
            email: "test2@example.com".to_string(),
            password_hash: "hash".to_string(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };
        let _ = test::call_service(
            &app,
            test::TestRequest::post()
                .uri("/users")
                .set_json(&user2)
                .to_request(),
        )
        .await;

        let resp_get =
            test::call_service(&app, test::TestRequest::get().uri("/users/1").to_request()).await;
        assert!(resp_get.status().is_success());
        let fetched_user: User = serde_json::from_slice(&test::read_body(resp_get).await)?;
        assert_eq!(fetched_user, new_user);

        // 2. Org
        let new_org = Organisation {
            id: 1,
            name: "testorg".to_string(),
            description: None,
            verified_domain: None,
            billing_plan: "pro".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/orgs")
                    .set_json(&new_org)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_org =
            test::call_service(&app, test::TestRequest::get().uri("/orgs/1").to_request()).await;
        let fetched_org: Organisation = serde_json::from_slice(&test::read_body(resp_org).await)?;
        assert_eq!(fetched_org, new_org);

        // 3. Repo
        let new_repo = Repository {
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
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/repos")
                    .set_json(&new_repo)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_repo =
            test::call_service(&app, test::TestRequest::get().uri("/repos/1").to_request()).await;
        let fetched_repo: Repository = serde_json::from_slice(&test::read_body(resp_repo).await)?;
        assert_eq!(fetched_repo, new_repo);

        // 4. Team
        let new_team = bridle_sdk::models::Team {
            id: 1,
            org_id: 1,
            parent_id: None,
            name: "team".to_string(),
            description: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/teams")
                    .set_json(&new_team)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_team =
            test::call_service(&app, test::TestRequest::get().uri("/teams/1").to_request()).await;
        let fetched_team: bridle_sdk::models::Team =
            serde_json::from_slice(&test::read_body(resp_team).await)?;
        assert_eq!(fetched_team, new_team);

        // 5. Branch
        let new_branch = bridle_sdk::models::Branch {
            id: 1,
            repo_id: 1,
            name: "main".to_string(),
            head_sha: "sha1".to_string(),
            is_protected: false,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/branches")
                    .set_json(&new_branch)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_branch = test::call_service(
            &app,
            test::TestRequest::get().uri("/branches/1").to_request(),
        )
        .await;
        let fetched_branch: bridle_sdk::models::Branch =
            serde_json::from_slice(&test::read_body(resp_branch).await)?;
        assert_eq!(fetched_branch, new_branch);

        // 6. BranchProtectionRule
        let new_bpr = bridle_sdk::models::BranchProtectionRule {
            id: 1,
            branch_id: 1,
            required_pr_reviews: 1,
            require_code_owner_reviews: false,
            required_status_checks: None,
            require_signed_commits: false,
            enforce_admins: false,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/branch_protection_rules")
                    .set_json(&new_bpr)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_bpr = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/branch_protection_rules/1")
                .to_request(),
        )
        .await;
        let fetched_bpr: bridle_sdk::models::BranchProtectionRule =
            serde_json::from_slice(&test::read_body(resp_bpr).await)?;
        assert_eq!(fetched_bpr, new_bpr);

        // 7. Key
        let new_key = bridle_sdk::models::Key {
            id: 1,
            user_id: 1,
            key_type: "ssh".to_string(),
            title: "k".to_string(),
            key_data: "d".to_string(),
            fingerprint: "fp1".to_string(),
            last_used_at: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/keys")
                    .set_json(&new_key)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_key =
            test::call_service(&app, test::TestRequest::get().uri("/keys/1").to_request()).await;
        let fetched_key: bridle_sdk::models::Key =
            serde_json::from_slice(&test::read_body(resp_key).await)?;
        assert_eq!(fetched_key, new_key);

        // 8. Follow
        let new_follow = bridle_sdk::models::Follow {
            id: 1,
            follower_id: 1,
            following_id: 2,
            created_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/follows")
                    .set_json(&new_follow)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_follow = test::call_service(
            &app,
            test::TestRequest::get().uri("/follows/1").to_request(),
        )
        .await;
        let fetched_follow: bridle_sdk::models::Follow =
            serde_json::from_slice(&test::read_body(resp_follow).await)?;
        assert_eq!(fetched_follow, new_follow);

        // 9. Star
        let new_star = bridle_sdk::models::Star {
            id: 1,
            user_id: 1,
            repo_id: 1,
            created_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/stars")
                    .set_json(&new_star)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_star =
            test::call_service(&app, test::TestRequest::get().uri("/stars/1").to_request()).await;
        let fetched_star: bridle_sdk::models::Star =
            serde_json::from_slice(&test::read_body(resp_star).await)?;
        assert_eq!(fetched_star, new_star);

        // 10. OrgMembership
        let new_om = bridle_sdk::models::OrgMembership {
            id: 1,
            org_id: 1,
            user_id: 1,
            role: "admin".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/org_memberships")
                    .set_json(&new_om)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_om = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/org_memberships/1")
                .to_request(),
        )
        .await;
        let fetched_om: bridle_sdk::models::OrgMembership =
            serde_json::from_slice(&test::read_body(resp_om).await)?;
        assert_eq!(fetched_om, new_om);

        // 11. RepoCollaborator
        let new_rc = bridle_sdk::models::RepoCollaborator {
            id: 1,
            repo_id: 1,
            user_id: 1,
            permission_level: "write".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/repo_collaborators")
                    .set_json(&new_rc)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_rc = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/repo_collaborators/1")
                .to_request(),
        )
        .await;
        let fetched_rc: bridle_sdk::models::RepoCollaborator =
            serde_json::from_slice(&test::read_body(resp_rc).await)?;
        assert_eq!(fetched_rc, new_rc);

        // 12. Milestone
        let new_ms = bridle_sdk::models::Milestone {
            id: 1,
            repo_id: 1,
            title: "v1".to_string(),
            description: None,
            state: "open".to_string(),
            due_on: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/milestones")
                    .set_json(&new_ms)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_ms = test::call_service(
            &app,
            test::TestRequest::get().uri("/milestones/1").to_request(),
        )
        .await;
        let fetched_ms: bridle_sdk::models::Milestone =
            serde_json::from_slice(&test::read_body(resp_ms).await)?;
        assert_eq!(fetched_ms, new_ms);

        // 13. Label
        let new_lbl = bridle_sdk::models::Label {
            id: 1,
            repo_id: 1,
            name: "bug".to_string(),
            color: "ff0000".to_string(),
            description: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/labels")
                    .set_json(&new_lbl)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_lbl =
            test::call_service(&app, test::TestRequest::get().uri("/labels/1").to_request()).await;
        let fetched_lbl: bridle_sdk::models::Label =
            serde_json::from_slice(&test::read_body(resp_lbl).await)?;
        assert_eq!(fetched_lbl, new_lbl);

        // 14. Issue
        let new_iss = bridle_sdk::models::Issue {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "iss".to_string(),
            body: None,
            state: "open".to_string(),
            author_id: 1,
            assignee_id: None,
            milestone_id: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/issues")
                    .set_json(&new_iss)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_iss =
            test::call_service(&app, test::TestRequest::get().uri("/issues/1").to_request()).await;
        let fetched_iss: bridle_sdk::models::Issue =
            serde_json::from_slice(&test::read_body(resp_iss).await)?;
        assert_eq!(fetched_iss, new_iss);

        // 15. IssueLabel
        let new_il = bridle_sdk::models::IssueLabel {
            id: 1,
            issue_id: 1,
            label_id: 1,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/issue_labels")
                    .set_json(&new_il)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_il = test::call_service(
            &app,
            test::TestRequest::get().uri("/issue_labels/1").to_request(),
        )
        .await;
        let fetched_il: bridle_sdk::models::IssueLabel =
            serde_json::from_slice(&test::read_body(resp_il).await)?;
        assert_eq!(fetched_il, new_il);

        // 16. PullRequest
        let new_pr = bridle_sdk::models::PullRequest {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "pr".to_string(),
            body: None,
            state: "open".to_string(),
            head_branch: "feat".to_string(),
            base_branch: "main".to_string(),
            author_id: 1,
            assignee_id: None,
            milestone_id: None,
            is_draft: false,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/pull_requests")
                    .set_json(&new_pr)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_pr = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/pull_requests/1")
                .to_request(),
        )
        .await;
        let fetched_pr: bridle_sdk::models::PullRequest =
            serde_json::from_slice(&test::read_body(resp_pr).await)?;
        assert_eq!(fetched_pr, new_pr);

        // 17. PullRequestReview
        let new_rev = bridle_sdk::models::PullRequestReview {
            id: 1,
            pr_id: 1,
            user_id: 1,
            state: "approved".to_string(),
            body: None,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/pull_request_reviews")
                    .set_json(&new_rev)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_rev = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/pull_request_reviews/1")
                .to_request(),
        )
        .await;
        let fetched_rev: bridle_sdk::models::PullRequestReview =
            serde_json::from_slice(&test::read_body(resp_rev).await)?;
        assert_eq!(fetched_rev, new_rev);

        // 18. Release
        let new_rel = bridle_sdk::models::Release {
            id: 1,
            repo_id: 1,
            tag_name: "v1".to_string(),
            target_commitish: "main".to_string(),
            name: None,
            body: None,
            is_draft: false,
            is_prerelease: false,
            author_id: 1,
            created_at: now,
            published_at: None,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/releases")
                    .set_json(&new_rel)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_rel = test::call_service(
            &app,
            test::TestRequest::get().uri("/releases/1").to_request(),
        )
        .await;
        let fetched_rel: bridle_sdk::models::Release =
            serde_json::from_slice(&test::read_body(resp_rel).await)?;
        assert_eq!(fetched_rel, new_rel);

        // 19. Webhook
        let new_wh = bridle_sdk::models::Webhook {
            id: 1,
            repo_id: 1,
            url: "https://example.com".to_string(),
            content_type: "json".to_string(),
            secret: None,
            events: "push".to_string(),
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/webhooks")
                    .set_json(&new_wh)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_wh = test::call_service(
            &app,
            test::TestRequest::get().uri("/webhooks/1").to_request(),
        )
        .await;
        let fetched_wh: bridle_sdk::models::Webhook =
            serde_json::from_slice(&test::read_body(resp_wh).await)?;
        assert_eq!(fetched_wh, new_wh);

        // 20. Commit
        let new_commit = bridle_sdk::models::Commit {
            id: 1,
            repo_id: 1,
            sha: "c1".to_string(),
            tree_sha: "t1".to_string(),
            parent_shas: "".to_string(),
            message: "m".to_string(),
            author_name: "a".to_string(),
            author_email: "a@b.c".to_string(),
            author_date: now,
            committer_name: "a".to_string(),
            committer_email: "a@b.c".to_string(),
            committer_date: now,
            created_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/commits")
                    .set_json(&new_commit)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_commit = test::call_service(
            &app,
            test::TestRequest::get().uri("/commits/1").to_request(),
        )
        .await;
        let fetched_commit: bridle_sdk::models::Commit =
            serde_json::from_slice(&test::read_body(resp_commit).await)?;
        assert_eq!(fetched_commit, new_commit);

        // 21. Tree
        let new_tree = bridle_sdk::models::Tree {
            id: 1,
            repo_id: 1,
            sha: "t1".to_string(),
            entries: "[]".to_string(),
            created_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/trees")
                    .set_json(&new_tree)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_tree =
            test::call_service(&app, test::TestRequest::get().uri("/trees/1").to_request()).await;
        let fetched_tree: bridle_sdk::models::Tree =
            serde_json::from_slice(&test::read_body(resp_tree).await)?;
        assert_eq!(fetched_tree, new_tree);

        // 22. Blob
        let new_blob = bridle_sdk::models::Blob {
            id: 1,
            repo_id: 1,
            sha: "b1".to_string(),
            size: 3,
            content: Some(b"abc".to_vec()),
            created_at: now,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/blobs")
                    .set_json(&new_blob)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_blob =
            test::call_service(&app, test::TestRequest::get().uri("/blobs/1").to_request()).await;
        let fetched_blob: bridle_sdk::models::Blob =
            serde_json::from_slice(&test::read_body(resp_blob).await)?;
        assert_eq!(fetched_blob, new_blob);

        // 23. BatchJob
        let new_job = bridle_sdk::models::BatchJob {
            id: 1,
            pipeline_name: "p".to_string(),
            status: "Pending".to_string(),
            started_at: now,
            completed_at: None,
        };
        assert_eq!(
            test::call_service(
                &app,
                test::TestRequest::post()
                    .uri("/api/batch/jobs")
                    .set_json(&new_job)
                    .to_request()
            )
            .await
            .status(),
            actix_web::http::StatusCode::CREATED
        );
        let resp_job = test::call_service(
            &app,
            test::TestRequest::get()
                .uri("/api/batch/jobs/1")
                .to_request(),
        )
        .await;
        let fetched_job: bridle_sdk::models::BatchJob =
            serde_json::from_slice(&test::read_body(resp_job).await)?;
        assert_eq!(fetched_job, new_job);

        // Verify 404 NotFound across all GET /{resource}/999 endpoints
        let not_found_uris = [
            "/users/999",
            "/orgs/999",
            "/repos/999",
            "/teams/999",
            "/branches/999",
            "/branch_protection_rules/999",
            "/keys/999",
            "/follows/999",
            "/stars/999",
            "/org_memberships/999",
            "/repo_collaborators/999",
            "/milestones/999",
            "/labels/999",
            "/issues/999",
            "/issue_labels/999",
            "/pull_requests/999",
            "/pull_request_reviews/999",
            "/releases/999",
            "/webhooks/999",
            "/commits/999",
            "/trees/999",
            "/blobs/999",
            "/api/batch/jobs/999",
        ];

        for uri in not_found_uris {
            let resp_404 =
                test::call_service(&app, test::TestRequest::get().uri(uri).to_request()).await;
            assert_eq!(
                resp_404.status(),
                actix_web::http::StatusCode::NOT_FOUND,
                "Expected 404 NotFound for {}",
                uri
            );
        }

        Ok(())
    }
}
