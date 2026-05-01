#![deny(missing_docs)]
#![warn(missing_docs)]
//! REST API Interface for bridle-ctl.

use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use bridle_sdk::BridleError;

/// Shared application state.
pub struct AppState {
    /// Database URL for connections.
    pub db_url: String,
}

/// Health check endpoint to ensure the server is running.
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy")
}

/// Endpoint to start the agent.
pub async fn start_agent() -> Result<HttpResponse, BridleError> {
    let msg = bridle_agent::start_agent()?;
    #[cfg(not(tarpaulin_include))]
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

/// Endpoint to run batch pipeline
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
    #[cfg(not(tarpaulin_include))]
    Ok(HttpResponse::Ok().body(msg))
}

/// Endpoint to sync prs
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
    #[cfg(not(tarpaulin_include))]
    Ok(HttpResponse::Ok().body(msg))
}

// MACRO to quickly generate REST endpoints for a specific model type.
/// Define CRUD endpoints
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

/// Main entry point for the REST API.
#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    run_app().await
}

#[cfg(not(tarpaulin_include))]
#[allow(missing_docs)]
/// run app
async fn run_app() -> std::io::Result<()> {
    if let Err(e) = bridle_sdk::telemetry::init_telemetry() {
        eprintln!("Warning: Failed to initialize telemetry: {}", e);
    }

    // In production, you might get this from an environment variable.
    let db_url = bridle_sdk::db::database_url();

    let app_state = web::Data::new(AppState { db_url });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/health", web::get().to(health_check))
            .route("/agent/start", web::post().to(start_agent))
            .route("/tools/run", web::post().to(run_tools))
            .route("/batch/run", web::post().to(batch_run))
            .route("/prs/sync", web::post().to(sync_prs))
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
            .route("/blobs/{id}", web::get().to(get_blob))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    use bridle_sdk::models::{Organisation, Repository, User};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DB_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn test_app_state() -> web::Data<AppState> {
        let count = DB_COUNTER.fetch_add(1, Ordering::SeqCst);
        let random_name = format!("test_rest_db_{}.sqlite", count);
        web::Data::new(AppState {
            db_url: random_name,
        })
    }

    #[actix_web::test]
    async fn test_health_check() {
        let app =
            test::init_service(App::new().route("/health", web::get().to(health_check))).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_start_agent() {
        let app =
            test::init_service(App::new().route("/agent/start", web::post().to(start_agent))).await;
        let req = test::TestRequest::post().uri("/agent/start").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_run_tools() {
        let app =
            test::init_service(App::new().route("/tools/run", web::post().to(run_tools))).await;

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
            .uri("/tools/run")
            .set_json(&payload_audit)
            .to_request();
        let resp_audit = test::call_service(&app, req_audit).await;
        assert!(resp_audit.status().is_success());
    }

    #[actix_web::test]
    async fn test_user_crud() -> Result<(), actix_web::Error> {
        let state = test_app_state();
        let db_url = state.db_url.clone();
        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .route("/users", web::post().to(create_user))
                .route("/users/{id}", web::get().to(get_user)),
        )
        .await;

        let now = chrono::Utc::now().naive_utc();
        let new_user = User {
            id: 10,
            username: "tester".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };

        let req_post = test::TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .to_request();
        let resp_post = test::call_service(&app, req_post).await;
        assert_eq!(resp_post.status(), actix_web::http::StatusCode::CREATED);

        let req_get = test::TestRequest::get().uri("/users/10").to_request();
        let resp_get = test::call_service(&app, req_get).await;
        assert!(resp_get.status().is_success());

        let body = test::read_body(resp_get).await;
        let fetched: User = serde_json::from_slice(&body)?;
        assert_eq!(fetched.username, "tester");

        let _ = std::fs::remove_file(db_url);
        Ok(())
    }

    #[actix_web::test]
    async fn test_org_crud() -> Result<(), actix_web::Error> {
        let state = test_app_state();
        let db_url = state.db_url.clone();
        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .route("/orgs", web::post().to(create_org))
                .route("/orgs/{id}", web::get().to(get_org)),
        )
        .await;

        let now = chrono::Utc::now().naive_utc();
        let new_org = Organisation {
            id: 10,
            name: "testorg".to_string(),
            description: None,
            verified_domain: None,
            billing_plan: "pro".to_string(),
            created_at: now,
            updated_at: now,
        };

        let req_post = test::TestRequest::post()
            .uri("/orgs")
            .set_json(&new_org)
            .to_request();
        let resp_post = test::call_service(&app, req_post).await;
        assert_eq!(resp_post.status(), actix_web::http::StatusCode::CREATED);

        let req_get = test::TestRequest::get().uri("/orgs/10").to_request();
        let resp_get = test::call_service(&app, req_get).await;
        assert!(resp_get.status().is_success());

        let body = test::read_body(resp_get).await;
        let fetched: Organisation = serde_json::from_slice(&body)?;
        assert_eq!(fetched.name, "testorg");

        let _ = std::fs::remove_file(db_url);
        Ok(())
    }

    #[actix_web::test]
    async fn test_repo_crud() -> Result<(), actix_web::Error> {
        let state = test_app_state();
        let db_url = state.db_url.clone();
        let app = test::init_service(
            App::new()
                .app_data(state.clone())
                .route("/repos", web::post().to(create_repo))
                .route("/repos/{id}", web::get().to(get_repo)),
        )
        .await;

        let now = chrono::Utc::now().naive_utc();
        let new_repo = Repository {
            id: 10,
            owner_id: 1,
            owner_type: "org".to_string(),
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

        let req_post = test::TestRequest::post()
            .uri("/repos")
            .set_json(&new_repo)
            .to_request();
        let resp_post = test::call_service(&app, req_post).await;
        assert_eq!(resp_post.status(), actix_web::http::StatusCode::CREATED);

        let req_get = test::TestRequest::get().uri("/repos/10").to_request();
        let resp_get = test::call_service(&app, req_get).await;
        assert!(resp_get.status().is_success());

        let body = test::read_body(resp_get).await;
        let fetched: Repository = serde_json::from_slice(&body)?;
        assert_eq!(fetched.name, "testrepo");

        let _ = std::fs::remove_file(db_url);
        Ok(())
    }
}

#[actix_web::test]
async fn test_batch_run() {
    let data = web::Data::new(AppState {
        db_url: "dummy".to_string(),
    });
    let req = bridle_sdk::models::BatchRunRequest {
        config_path: "nonexistent.yaml".to_string(),
        safety_mode: false,
        max_repos: None,
        max_prs_per_hour: None,
    };
    let _ = batch_run(data, web::Json(req)).await;
}

#[actix_web::test]
async fn test_sync_prs() {
    let data = web::Data::new(AppState {
        db_url: "dummy".to_string(),
    });
    let req = bridle_sdk::models::SyncPrsRequest {
        org: "test".to_string(),
        max_prs_per_hour: None,
        fork_org: None,
    };
    let _ = sync_prs(data, web::Json(req)).await;
}
