#![deny(missing_docs)]
//! JSON RPC Interface for bridle-ctl.

use bridle_sdk::BridleError;
use jsonrpsee::RpcModule;
use jsonrpsee::server::ServerBuilder;
use std::net::SocketAddr;
use std::sync::Arc;

/// Helper to create an `BridleError` from a generic displayable error.
fn rpc_reg_err<T: std::fmt::Display>(e: T) -> BridleError {
    BridleError::Register(e.to_string())
}

/// Helper to convert a generic displayable error into an `ErrorObjectOwned`.
fn rpc_err_from<T: std::fmt::Display>(
    e: T,
) -> Result<String, jsonrpsee::types::error::ErrorObjectOwned> {
    Err(jsonrpsee::types::error::ErrorObject::owned(
        -32000,
        e.to_string(),
        None::<()>,
    ))
}

/// Helper to convert a generic displayable error into an `ErrorObjectOwned` returning a String.
fn rpc_reg_err_into<T: std::fmt::Display>(
    e: T,
) -> Result<String, jsonrpsee::types::error::ErrorObjectOwned> {
    Err(jsonrpsee::types::error::ErrorObject::owned(
        -32000,
        BridleError::Register(e.to_string()).to_string(),
        None::<()>,
    ))
}

/// State shared across RPC calls.
pub struct RpcState {
    /// The database connection string.
    pub db_url: String,
}

/// Macro to easily register CRUD operations for SDK models in JSON RPC.
macro_rules! register_crud_methods {
    ($module:expr, $create_name:expr, $get_name:expr, $sdk_get:path, $sdk_insert:path, $model:ty) => {
        $module
            .register_method($create_name, |params, state, _| {
                let (item,): ($model,) = params.parse()?;
                let mut conn =
                    bridle_sdk::db::establish_connection_and_run_migrations(&state.db_url)?;
                $sdk_insert(&mut conn, &item)?;
                Ok::<(), jsonrpsee::types::error::ErrorObjectOwned>(())
            })
            .map_err(rpc_reg_err)?;

        $module
            .register_method($get_name, |params, state, _| {
                let (id,): (i32,) = params.parse()?;
                let mut conn =
                    bridle_sdk::db::establish_connection_and_run_migrations(&state.db_url)?;
                let item = $sdk_get(&mut conn, id)?;
                Ok::<$model, jsonrpsee::types::error::ErrorObjectOwned>(item)
            })
            .map_err(rpc_reg_err)?;
    };
}

/// Starts the JSON-RPC server with a given db_url.
pub async fn run_server(db_url: String) -> Result<SocketAddr, BridleError> {
    let server = ServerBuilder::default().build("127.0.0.1:0").await?;
    let addr = server.local_addr()?;

    let state = Arc::new(RpcState { db_url });
    let mut module = RpcModule::new(state);

    module
        .register_method("health", |_, _, _| {
            Ok::<&str, jsonrpsee::types::error::ErrorObjectOwned>("Server is healthy")
        })
        .map_err(rpc_reg_err)?;

    module
        .register_method("start_agent", |_, _, _| match bridle_agent::start_agent() {
            Ok(msg) => Ok::<String, jsonrpsee::types::error::ErrorObjectOwned>(msg.to_string()),
            Err(e) => rpc_err_from(e),
        })
        .map_err(rpc_reg_err)?;

    register_crud_methods!(
        module,
        "create_user",
        "get_user",
        bridle_sdk::db::get_user,
        bridle_sdk::db::insert_user,
        bridle_sdk::models::User
    );
    register_crud_methods!(
        module,
        "create_org",
        "get_org",
        bridle_sdk::db::get_organisation,
        bridle_sdk::db::insert_organisation,
        bridle_sdk::models::Organisation
    );
    register_crud_methods!(
        module,
        "create_repo",
        "get_repo",
        bridle_sdk::db::get_repository,
        bridle_sdk::db::insert_repository,
        bridle_sdk::models::Repository
    );
    register_crud_methods!(
        module,
        "create_team",
        "get_team",
        bridle_sdk::db::get_team,
        bridle_sdk::db::insert_team,
        bridle_sdk::models::Team
    );
    register_crud_methods!(
        module,
        "create_branch",
        "get_branch",
        bridle_sdk::db::get_branch,
        bridle_sdk::db::insert_branch,
        bridle_sdk::models::Branch
    );
    register_crud_methods!(
        module,
        "create_branch_protection_rule",
        "get_branch_protection_rule",
        bridle_sdk::db::get_branch_protection_rule,
        bridle_sdk::db::insert_branch_protection_rule,
        bridle_sdk::models::BranchProtectionRule
    );
    register_crud_methods!(
        module,
        "create_key",
        "get_key",
        bridle_sdk::db::get_key,
        bridle_sdk::db::insert_key,
        bridle_sdk::models::Key
    );
    register_crud_methods!(
        module,
        "create_follow",
        "get_follow",
        bridle_sdk::db::get_follow,
        bridle_sdk::db::insert_follow,
        bridle_sdk::models::Follow
    );
    register_crud_methods!(
        module,
        "create_star",
        "get_star",
        bridle_sdk::db::get_star,
        bridle_sdk::db::insert_star,
        bridle_sdk::models::Star
    );
    register_crud_methods!(
        module,
        "create_org_membership",
        "get_org_membership",
        bridle_sdk::db::get_org_membership,
        bridle_sdk::db::insert_org_membership,
        bridle_sdk::models::OrgMembership
    );
    register_crud_methods!(
        module,
        "create_repo_collaborator",
        "get_repo_collaborator",
        bridle_sdk::db::get_repo_collaborator,
        bridle_sdk::db::insert_repo_collaborator,
        bridle_sdk::models::RepoCollaborator
    );
    register_crud_methods!(
        module,
        "create_milestone",
        "get_milestone",
        bridle_sdk::db::get_milestone,
        bridle_sdk::db::insert_milestone,
        bridle_sdk::models::Milestone
    );
    register_crud_methods!(
        module,
        "create_label",
        "get_label",
        bridle_sdk::db::get_label,
        bridle_sdk::db::insert_label,
        bridle_sdk::models::Label
    );
    register_crud_methods!(
        module,
        "create_issue",
        "get_issue",
        bridle_sdk::db::get_issue,
        bridle_sdk::db::insert_issue,
        bridle_sdk::models::Issue
    );
    register_crud_methods!(
        module,
        "create_issue_label",
        "get_issue_label",
        bridle_sdk::db::get_issue_label,
        bridle_sdk::db::insert_issue_label,
        bridle_sdk::models::IssueLabel
    );
    register_crud_methods!(
        module,
        "create_pull_request",
        "get_pull_request",
        bridle_sdk::db::get_pull_request,
        bridle_sdk::db::insert_pull_request,
        bridle_sdk::models::PullRequest
    );
    register_crud_methods!(
        module,
        "create_pull_request_review",
        "get_pull_request_review",
        bridle_sdk::db::get_pull_request_review,
        bridle_sdk::db::insert_pull_request_review,
        bridle_sdk::models::PullRequestReview
    );
    register_crud_methods!(
        module,
        "create_release",
        "get_release",
        bridle_sdk::db::get_release,
        bridle_sdk::db::insert_release,
        bridle_sdk::models::Release
    );
    register_crud_methods!(
        module,
        "create_webhook",
        "get_webhook",
        bridle_sdk::db::get_webhook,
        bridle_sdk::db::insert_webhook,
        bridle_sdk::models::Webhook
    );
    register_crud_methods!(
        module,
        "create_commit",
        "get_commit",
        bridle_sdk::db::get_commit,
        bridle_sdk::db::insert_commit,
        bridle_sdk::models::Commit
    );
    register_crud_methods!(
        module,
        "create_tree",
        "get_tree",
        bridle_sdk::db::get_tree,
        bridle_sdk::db::insert_tree,
        bridle_sdk::models::Tree
    );
    register_crud_methods!(
        module,
        "create_blob",
        "get_blob",
        bridle_sdk::db::get_blob,
        bridle_sdk::db::insert_blob,
        bridle_sdk::models::Blob
    );
    register_crud_methods!(
        module,
        "create_batch_job",
        "get_batch_job",
        bridle_sdk::batch_db::get_batch_job,
        bridle_sdk::batch_db::insert_batch_job,
        bridle_sdk::models::BatchJob
    );
    register_crud_methods!(
        module,
        "create_batch_task",
        "get_batch_task",
        bridle_sdk::batch_db::get_batch_task,
        bridle_sdk::batch_db::insert_batch_task,
        bridle_sdk::models::BatchTask
    );

    module
        .register_method("run_tools", |params, _, _| {
            let (req,): (bridle_sdk::models::ToolRunRequest,) = params.parse()?;
            let action = match req.action.as_deref() {
                Some("audit") => bridle_cli::runner::Action::Audit,
                _ => {
                    let is_dry_run = matches!(req.dry_run, Some(true));
                    bridle_cli::runner::Action::Fix {
                        dry_run: is_dry_run,
                    }
                }
            };

            match bridle_cli::runner::run(action, req) {
                Ok(_) => Ok::<String, jsonrpsee::types::error::ErrorObjectOwned>(
                    "Tools executed successfully".to_string(),
                ),
                Err(e) => rpc_err_from(e),
            }
        })
        .map_err(rpc_reg_err)?;

    module
        .register_method("batch_run", |params, state, _| {
            let (req,): (bridle_sdk::models::BatchRunRequest,) = params.parse()?;
            match bridle_cli::batch_pipeline::run_pipeline(
                &req.config_path,
                &state.db_url,
                req.safety_mode,
                req.max_repos,
                req.max_prs_per_hour,
            ) {
                Ok(msg) => Ok::<String, jsonrpsee::types::error::ErrorObjectOwned>(msg),
                Err(e) => rpc_reg_err_into(e),
            }
        })
        .map_err(rpc_reg_err)?;

    module
        .register_method("batch_fix", |params, state, _| {
            let (req,): (bridle_sdk::models::BatchFixRequest,) = params.parse()?;
            match bridle_cli::batch_fix::batch_fix(
                &req.org,
                &req.issue,
                req.pattern,
                req.tools,
                req.tool_args,
                &state.db_url,
                req.safety_mode,
                req.max_repos,
                req.max_prs_per_hour,
            ) {
                Ok(msg) => Ok::<String, jsonrpsee::types::error::ErrorObjectOwned>(msg),
                Err(e) => rpc_reg_err_into(e),
            }
        })
        .map_err(rpc_reg_err)?;

    module
        .register_method("sync_prs", |params, state, _| {
            let (req,): (bridle_sdk::models::SyncPrsRequest,) = params.parse()?;
            match bridle_cli::sync_prs::sync_prs(
                &req.org,
                &state.db_url,
                req.max_prs_per_hour,
                req.fork_org,
            ) {
                Ok(msg) => Ok::<String, jsonrpsee::types::error::ErrorObjectOwned>(msg),
                Err(e) => rpc_reg_err_into(e),
            }
        })
        .map_err(rpc_reg_err)?;

    let handle = server.start(module);

    tokio::spawn(handle.stopped());

    Ok(addr)
}

/// Main entry point for the JSON RPC server.
#[tokio::main]
async fn main() -> Result<(), BridleError> {
    if let Err(e) = bridle_sdk::telemetry::init_telemetry() {
        eprintln!("Warning: Failed to initialize telemetry: {}", e);
    }

    let db_url = bridle_sdk::db::database_url();
    let addr = run_server(db_url).await?;
    println!("JSON-RPC server running at {}", addr);

    std::future::pending::<()>().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bridle_sdk::models::{Organisation, Repository, User};
    use jsonrpsee::core::client::ClientT;
    use jsonrpsee::http_client::HttpClientBuilder;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DB_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn get_test_db() -> String {
        let count = DB_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("test_rpc_db_{}.sqlite", count)
    }

    #[tokio::test]
    async fn test_health_check_rpc() -> Result<(), BridleError> {
        let addr = run_server(get_test_db()).await?;
        let url = format!("http://{}", addr);

        let client = HttpClientBuilder::default().build(url)?;
        let response: String = client.request("health", jsonrpsee::rpc_params![]).await?;

        assert_eq!(response, "Server is healthy");
        Ok(())
    }

    #[tokio::test]
    async fn test_start_agent_rpc() -> Result<(), BridleError> {
        let addr = run_server(get_test_db()).await?;
        let url = format!("http://{}", addr);

        let client = HttpClientBuilder::default().build(url)?;
        let response: String = client
            .request("start_agent", jsonrpsee::rpc_params![])
            .await?;

        assert_eq!(response, "Agent started");
        Ok(())
    }

    #[tokio::test]
    async fn test_run_tools_rpc() -> Result<(), BridleError> {
        let addr = run_server(get_test_db()).await?;
        let url = format!("http://{}", addr);

        let client = HttpClientBuilder::default().build(url)?;

        let payload_fix = bridle_sdk::models::ToolRunRequest {
            pattern: Some(r".*\.go$".to_string()),
            tools: Some(vec!["rust-unwrap-to-question-mark".to_string()]),
            tool_args: None,
            dry_run: Some(true),
            action: Some("fix".to_string()),
        };

        let response_fix: String = client
            .request("run_tools", jsonrpsee::rpc_params![payload_fix])
            .await?;
        assert_eq!(response_fix, "Tools executed successfully");

        let payload_audit = bridle_sdk::models::ToolRunRequest {
            pattern: Some(r".*\.go$".to_string()),
            tools: Some(vec!["rust-unwrap-to-question-mark".to_string()]),
            tool_args: None,
            dry_run: None,
            action: Some("audit".to_string()),
        };

        let response_audit: String = client
            .request("run_tools", jsonrpsee::rpc_params![payload_audit])
            .await?;
        assert_eq!(response_audit, "Tools executed successfully");

        Ok(())
    }

    #[tokio::test]
    async fn test_team_crud_rpc() -> Result<(), BridleError> {
        let db_url = get_test_db();
        let addr = run_server(db_url.clone()).await?;
        let url = format!("http://{}", addr);
        let client = HttpClientBuilder::default().build(url)?;

        let now = chrono::Utc::now().naive_utc();
        let new_team = bridle_sdk::models::Team {
            id: 11,
            org_id: 1,
            parent_id: None,
            name: "rpcteam".to_string(),
            description: None,
            created_at: now,
            updated_at: now,
        };

        let _: () = client
            .request("create_team", jsonrpsee::rpc_params![new_team])
            .await?;
        let fetched: bridle_sdk::models::Team = client
            .request("get_team", jsonrpsee::rpc_params![11])
            .await?;

        assert_eq!(fetched.name, "rpcteam");

        let _ = std::fs::remove_file(db_url);
        Ok(())
    }

    #[tokio::test]
    async fn test_user_crud_rpc() -> Result<(), BridleError> {
        let db_url = get_test_db();
        let addr = run_server(db_url.clone()).await?;
        let url = format!("http://{}", addr);
        let client = HttpClientBuilder::default().build(url)?;

        let now = chrono::Utc::now().naive_utc();
        let new_user = User {
            id: 11,
            username: "rpctester".to_string(),
            email: "rpc@example.com".to_string(),
            password_hash: "hash".to_string(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };

        let _: () = client
            .request("create_user", jsonrpsee::rpc_params![new_user])
            .await?;
        let fetched: User = client
            .request("get_user", jsonrpsee::rpc_params![11])
            .await?;

        assert_eq!(fetched.username, "rpctester");

        let _ = std::fs::remove_file(db_url);
        Ok(())
    }

    #[tokio::test]
    async fn test_org_crud_rpc() -> Result<(), BridleError> {
        let db_url = get_test_db();
        let addr = run_server(db_url.clone()).await?;
        let url = format!("http://{}", addr);
        let client = HttpClientBuilder::default().build(url)?;

        let now = chrono::Utc::now().naive_utc();
        let new_org = Organisation {
            id: 11,
            name: "rpcorg".to_string(),
            description: None,
            verified_domain: None,
            billing_plan: "pro".to_string(),
            created_at: now,
            updated_at: now,
        };

        let _: () = client
            .request("create_org", jsonrpsee::rpc_params![new_org])
            .await?;
        let fetched: Organisation = client
            .request("get_org", jsonrpsee::rpc_params![11])
            .await?;

        assert_eq!(fetched.name, "rpcorg");

        let _ = std::fs::remove_file(db_url);
        Ok(())
    }

    #[tokio::test]
    async fn test_repo_crud_rpc() -> Result<(), BridleError> {
        let db_url = get_test_db();
        let addr = run_server(db_url.clone()).await?;
        let url = format!("http://{}", addr);
        let client = HttpClientBuilder::default().build(url)?;

        let now = chrono::Utc::now().naive_utc();
        let new_repo = Repository {
            id: 11,
            owner_id: 1,
            owner_type: "org".to_string(),
            name: "rpcrepo".to_string(),
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

        let _: () = client
            .request("create_repo", jsonrpsee::rpc_params![new_repo])
            .await?;
        let fetched: Repository = client
            .request("get_repo", jsonrpsee::rpc_params![11])
            .await?;

        assert_eq!(fetched.name, "rpcrepo");

        let _ = std::fs::remove_file(db_url);
        Ok(())
    }

    #[tokio::test]
    async fn test_batch_and_sync_rpc() -> Result<(), BridleError> {
        let db_url = get_test_db();
        let addr = run_server(db_url.clone()).await?;
        let url = format!("http://{}", addr);
        let client = HttpClientBuilder::default().build(url)?;

        // batch_run
        let req_batch = bridle_sdk::models::BatchRunRequest {
            config_path: "nonexistent.yml".to_string(),
            safety_mode: true,
            max_repos: Some(1),
            max_prs_per_hour: Some(1),
        };
        let res_batch: Result<String, _> = client
            .request("batch_run", jsonrpsee::rpc_params![req_batch])
            .await;
        assert!(res_batch.is_err());

        // batch_fix
        let req_fix = bridle_sdk::models::BatchFixRequest {
            org: "test".to_string(),
            issue: "test".to_string(),
            pattern: None,
            tools: None,
            tool_args: None,
            safety_mode: true,
            max_repos: Some(1),
            max_prs_per_hour: Some(1),
        };
        let res_fix: Result<String, _> = client
            .request("batch_fix", jsonrpsee::rpc_params![req_fix])
            .await;
        assert!(res_fix.is_err());

        // sync_prs
        let req_sync = bridle_sdk::models::SyncPrsRequest {
            org: "test".to_string(),
            max_prs_per_hour: Some(1),
            fork_org: None,
        };
        let res_sync: Result<String, _> = client
            .request("sync_prs", jsonrpsee::rpc_params![req_sync])
            .await;
        assert!(res_sync.is_ok());

        let _ = std::fs::remove_file(db_url);
        Ok(())
    }

    #[tokio::test]
    async fn test_rpc_error_conversions() {
        let err = rpc_reg_err_into("test");
        assert!(err.is_err());

        let err2 = rpc_reg_err("test");
        assert!(matches!(err2, BridleError::Register(_)));
    }

    #[tokio::test]
    async fn test_rpc_err_from() {
        let err = rpc_err_from("test");
        assert!(err.is_err());
    }
}
