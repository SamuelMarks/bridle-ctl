#![allow(missing_docs)]
//! REST API Interface for bridle-ctl.

/// Error module.
pub mod error;

use crate::error::RestError;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

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
pub async fn start_agent() -> Result<HttpResponse, RestError> {
    let msg = bridle_agent::start_agent()?;
    Ok(HttpResponse::Ok().body(msg))
}

/// Endpoint to add numbers using SDK.
pub async fn sdk_add(path: web::Path<(usize, usize)>) -> impl Responder {
    let (a, b) = path.into_inner();
    let res = bridle_sdk::add(a, b);
    HttpResponse::Ok().body(res.to_string())
}

/// Endpoint to run tools using ToolRunRequest.
#[cfg(not(tarpaulin_include))]
pub async fn run_tools(
    req: web::Json<bridle_sdk::models::ToolRunRequest>,
) -> Result<HttpResponse, RestError> {
    let payload = req.into_inner();
    let action = match payload.action.as_deref() {
        Some("audit") => bridle_cli::runner::Action::Audit,
        _ => {
            let is_dry_run = match payload.dry_run {
                Some(true) => true,
                _ => false,
            };
            bridle_cli::runner::Action::Fix {
                dry_run: is_dry_run,
            }
        }
    };

    bridle_cli::runner::run(action, payload)?;
    Ok(HttpResponse::Ok().body("Tools executed successfully"))
}

/// Endpoint to fetch a sample user (kept for legacy tests).
pub async fn get_sample_user(data: web::Data<AppState>) -> Result<HttpResponse, RestError> {
    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(&data.db_url)
        .map_err(RestError::Sdk)?;

    let now = chrono::Utc::now().naive_utc();
    let new_user = bridle_sdk::models::User {
        id: 1,
        username: "agent_smith".to_string(),
        email: "smith@matrix.com".to_string(),
        password_hash: "hashed".to_string(),
        avatar_url: None,
        bio: Some("I am inevitable.".to_string()),
        status: None,
        created_at: now,
        updated_at: now,
    };

    // Ignore error if already exists
    let _ = bridle_sdk::db::insert_user(&mut conn, &new_user);

    let fetched = bridle_sdk::db::get_user(&mut conn, 1).map_err(RestError::Sdk)?;

    Ok(HttpResponse::Ok().json(fetched))
}

// MACRO to quickly generate REST endpoints for a specific model type.
macro_rules! define_crud_endpoints {
    ($get_fn:ident, $create_fn:ident, $sdk_get:path, $sdk_insert:path, $model:ty) => {
        /// Creates a new item in the database.
        pub async fn $create_fn(
            data: web::Data<AppState>,
            payload: web::Json<$model>,
        ) -> Result<HttpResponse, RestError> {
            let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(&data.db_url)
                .map_err(RestError::Sdk)?;
            $sdk_insert(&mut conn, &payload.into_inner()).map_err(RestError::Sdk)?;
            Ok(HttpResponse::Created().finish())
        }

        /// Retrieves an item from the database.
        pub async fn $get_fn(
            data: web::Data<AppState>,
            path: web::Path<i32>,
        ) -> Result<HttpResponse, RestError> {
            let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(&data.db_url)
                .map_err(RestError::Sdk)?;
            let id = path.into_inner();
            let item = $sdk_get(&mut conn, id).map_err(RestError::Sdk)?;
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
#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // In production, you might get this from an environment variable.
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "bridle.db".to_string());

    let app_state = web::Data::new(AppState { db_url });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/health", web::get().to(health_check))
            .route("/agent/start", web::post().to(start_agent))
            .route("/sdk/add/{a}/{b}", web::get().to(sdk_add))
            .route("/users/sample", web::get().to(get_sample_user))
            .route("/tools/run", web::post().to(run_tools))
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
    async fn test_sdk_add() {
        let app =
            test::init_service(App::new().route("/sdk/add/{a}/{b}", web::get().to(sdk_add))).await;
        let req = test::TestRequest::get().uri("/sdk/add/5/7").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_sample_user() -> Result<(), actix_web::Error> {
        let state = test_app_state();
        let db_url = state.db_url.clone();
        let app = test::init_service(
            App::new()
                .app_data(state)
                .route("/users/sample", web::get().to(get_sample_user)),
        )
        .await;
        let req = test::TestRequest::get().uri("/users/sample").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let user: bridle_sdk::models::User = serde_json::from_slice(&body)?;
        assert_eq!(user.username, "agent_smith");

        // cleanup
        let _ = std::fs::remove_file(db_url);
        Ok(())
    }

    #[actix_web::test]
    async fn test_run_tools() {
        let app =
            test::init_service(App::new().route("/tools/run", web::post().to(run_tools))).await;

        let payload_fix = bridle_sdk::models::ToolRunRequest {
            pattern: Some(r".*\.go$".to_string()),
            tools: Some(vec!["go-err-check".to_string()]),
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
            tools: Some(vec!["go-err-check".to_string()]),
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
