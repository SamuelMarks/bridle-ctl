#![deny(missing_docs)]
#![warn(missing_docs)]
//! JSON RPC Interface for bridle-ctl.

use bridle_sdk::BridleError;
use jsonrpsee::RpcModule;
use jsonrpsee::server::ServerBuilder;
use std::net::SocketAddr;
use std::sync::Arc;

/// Helper to create a `BridleError` from a generic displayable error.
fn rpc_reg_err<T: std::fmt::Display>(e: T) -> BridleError {
    BridleError::Register(e.to_string())
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

/// Builds the JSON-RPC module with all registered methods for the given database URL.
pub fn build_rpc_module(db_url: String) -> Result<RpcModule<RpcState>, BridleError> {
    let state = Arc::new(RpcState { db_url });
    let mut module = RpcModule::from_arc(state);

    module
        .register_method("health", |_, _, _| {
            Ok::<&str, jsonrpsee::types::error::ErrorObjectOwned>("Server is healthy")
        })
        .map_err(rpc_reg_err)?;

    module
        .register_method("start_agent", |_, _, _| {
            let msg = bridle_agent::start_agent()?;
            Ok::<String, jsonrpsee::types::error::ErrorObjectOwned>(msg.to_string())
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

            bridle_cli::runner::run(action, req)?;
            Ok::<String, jsonrpsee::types::error::ErrorObjectOwned>(
                "Tools executed successfully".to_string(),
            )
        })
        .map_err(rpc_reg_err)?;

    module
        .register_method("batch_run", |params, state, _| {
            let (req,): (bridle_sdk::models::BatchRunRequest,) = params.parse()?;
            let msg = bridle_cli::batch_pipeline::run_pipeline(
                &req.config_path,
                &state.db_url,
                req.safety_mode,
                req.max_repos,
                req.max_prs_per_hour,
            )?;
            Ok::<String, jsonrpsee::types::error::ErrorObjectOwned>(msg)
        })
        .map_err(rpc_reg_err)?;

    module
        .register_method("batch_fix", |params, state, _| {
            let (req,): (bridle_sdk::models::BatchFixRequest,) = params.parse()?;
            let msg = bridle_cli::batch_fix::batch_fix(
                &req.org,
                &req.issue,
                req.pattern,
                req.tools,
                req.tool_args,
                &state.db_url,
                req.safety_mode,
                req.max_repos,
                req.max_prs_per_hour,
            )?;
            Ok::<String, jsonrpsee::types::error::ErrorObjectOwned>(msg)
        })
        .map_err(rpc_reg_err)?;

    module
        .register_method("sync_prs", |params, state, _| {
            let (req,): (bridle_sdk::models::SyncPrsRequest,) = params.parse()?;
            let msg = bridle_cli::sync_prs::sync_prs(
                &req.org,
                &state.db_url,
                req.max_prs_per_hour,
                req.fork_org,
            )?;
            Ok::<String, jsonrpsee::types::error::ErrorObjectOwned>(msg)
        })
        .map_err(rpc_reg_err)?;

    Ok(module)
}

/// Starts the JSON-RPC server with a given db_url.
pub async fn run_server(db_url: String) -> Result<SocketAddr, BridleError> {
    let server = ServerBuilder::default().build("127.0.0.1:0").await?;
    let addr = server.local_addr()?;
    let module = build_rpc_module(db_url)?;
    let handle = server.start(module);
    tokio::spawn(handle.stopped());
    Ok(addr)
}

/// Main entry point for the JSON RPC server.
#[tokio::main]
async fn main() -> Result<(), BridleError> {
    let _ = bridle_sdk::telemetry::init_telemetry();
    let db_url = bridle_sdk::db::database_url();
    let addr = run_server(db_url).await?;
    println!("JSON-RPC server running at {}", addr);

    #[cfg(not(test))]
    std::future::pending::<()>().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bridle_sdk::models::{Organisation, Repository, User};
    use jsonrpsee::core::client::ClientT;
    use jsonrpsee::http_client::HttpClientBuilder;

    fn get_test_db() -> (tempfile::NamedTempFile, String) {
        let tf = tempfile::NamedTempFile::new().unwrap_or_else(|e| panic!("temp file: {}", e));
        let path = tf.path().to_string_lossy().to_string();
        (tf, path)
    }

    #[tokio::test]
    async fn test_health_and_start_agent_rpc() -> Result<(), BridleError> {
        let (_tf, db_url) = get_test_db();
        let addr = run_server(db_url).await?;
        let url = format!("http://{}", addr);

        let client = HttpClientBuilder::default().build(url)?;
        let response: String = client.request("health", jsonrpsee::rpc_params![]).await?;
        assert_eq!(response, "Server is healthy");

        let agent_resp: String = client
            .request("start_agent", jsonrpsee::rpc_params![])
            .await?;
        assert_eq!(agent_resp, "Agent started");

        let _ = std::thread::spawn(super::main).join();
        Ok(())
    }

    #[tokio::test]
    async fn test_run_tools_rpc() -> Result<(), BridleError> {
        let (_tf, db_url) = get_test_db();
        let addr = run_server(db_url).await?;
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
    async fn test_all_crud_methods_rpc() -> Result<(), BridleError> {
        let (_tf, db_url) = get_test_db();
        let addr = run_server(db_url).await?;
        let url = format!("http://{}", addr);
        let client = HttpClientBuilder::default().build(url)?;

        let now = chrono::Utc::now().naive_utc();

        let new_user = User {
            id: 1,
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
            .request("create_user", jsonrpsee::rpc_params![&new_user])
            .await?;
        let fetched_user: User = client
            .request("get_user", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_user, new_user);

        let user2 = User {
            id: 2,
            username: "rpctester2".to_string(),
            email: "rpc2@example.com".to_string(),
            password_hash: "hash".to_string(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request("create_user", jsonrpsee::rpc_params![&user2])
            .await?;

        let new_org = Organisation {
            id: 1,
            name: "rpcorg".to_string(),
            description: None,
            verified_domain: None,
            billing_plan: "pro".to_string(),
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request("create_org", jsonrpsee::rpc_params![&new_org])
            .await?;
        let fetched_org: Organisation =
            client.request("get_org", jsonrpsee::rpc_params![1]).await?;
        assert_eq!(fetched_org, new_org);

        let new_repo = Repository {
            id: 1,
            owner_id: 1,
            owner_type: "user".to_string(),
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
            .request("create_repo", jsonrpsee::rpc_params![&new_repo])
            .await?;
        let fetched_repo: Repository = client
            .request("get_repo", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_repo, new_repo);

        let new_team = bridle_sdk::models::Team {
            id: 1,
            org_id: 1,
            parent_id: None,
            name: "rpcteam".to_string(),
            description: None,
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request("create_team", jsonrpsee::rpc_params![&new_team])
            .await?;
        let fetched_team: bridle_sdk::models::Team = client
            .request("get_team", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_team, new_team);

        let new_branch = bridle_sdk::models::Branch {
            id: 1,
            repo_id: 1,
            name: "main".to_string(),
            head_sha: "sha1".to_string(),
            is_protected: false,
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request("create_branch", jsonrpsee::rpc_params![&new_branch])
            .await?;
        let fetched_branch: bridle_sdk::models::Branch = client
            .request("get_branch", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_branch, new_branch);

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
        let _: () = client
            .request(
                "create_branch_protection_rule",
                jsonrpsee::rpc_params![&new_bpr],
            )
            .await?;
        let fetched_bpr: bridle_sdk::models::BranchProtectionRule = client
            .request("get_branch_protection_rule", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_bpr, new_bpr);

        let new_key = bridle_sdk::models::Key {
            id: 1,
            user_id: 1,
            key_type: "ssh".to_string(),
            title: "key".to_string(),
            key_data: "ssh-rsa".to_string(),
            fingerprint: "fp1".to_string(),
            last_used_at: None,
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request("create_key", jsonrpsee::rpc_params![&new_key])
            .await?;
        let fetched_key: bridle_sdk::models::Key =
            client.request("get_key", jsonrpsee::rpc_params![1]).await?;
        assert_eq!(fetched_key, new_key);

        let new_follow = bridle_sdk::models::Follow {
            id: 1,
            follower_id: 1,
            following_id: 2,
            created_at: now,
        };
        let _: () = client
            .request("create_follow", jsonrpsee::rpc_params![&new_follow])
            .await?;
        let fetched_follow: bridle_sdk::models::Follow = client
            .request("get_follow", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_follow, new_follow);

        let new_star = bridle_sdk::models::Star {
            id: 1,
            user_id: 1,
            repo_id: 1,
            created_at: now,
        };
        let _: () = client
            .request("create_star", jsonrpsee::rpc_params![&new_star])
            .await?;
        let fetched_star: bridle_sdk::models::Star = client
            .request("get_star", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_star, new_star);

        let new_org_mem = bridle_sdk::models::OrgMembership {
            id: 1,
            org_id: 1,
            user_id: 1,
            role: "admin".to_string(),
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request(
                "create_org_membership",
                jsonrpsee::rpc_params![&new_org_mem],
            )
            .await?;
        let fetched_org_mem: bridle_sdk::models::OrgMembership = client
            .request("get_org_membership", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_org_mem, new_org_mem);

        let new_collab = bridle_sdk::models::RepoCollaborator {
            id: 1,
            repo_id: 1,
            user_id: 1,
            permission_level: "write".to_string(),
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request(
                "create_repo_collaborator",
                jsonrpsee::rpc_params![&new_collab],
            )
            .await?;
        let fetched_collab: bridle_sdk::models::RepoCollaborator = client
            .request("get_repo_collaborator", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_collab, new_collab);

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
        let _: () = client
            .request("create_milestone", jsonrpsee::rpc_params![&new_ms])
            .await?;
        let fetched_ms: bridle_sdk::models::Milestone = client
            .request("get_milestone", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_ms, new_ms);

        let new_lbl = bridle_sdk::models::Label {
            id: 1,
            repo_id: 1,
            name: "bug".to_string(),
            color: "ff0000".to_string(),
            description: None,
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request("create_label", jsonrpsee::rpc_params![&new_lbl])
            .await?;
        let fetched_lbl: bridle_sdk::models::Label = client
            .request("get_label", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_lbl, new_lbl);

        let new_issue = bridle_sdk::models::Issue {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "issue1".to_string(),
            body: None,
            state: "open".to_string(),
            author_id: 1,
            assignee_id: None,
            milestone_id: None,
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request("create_issue", jsonrpsee::rpc_params![&new_issue])
            .await?;
        let fetched_issue: bridle_sdk::models::Issue = client
            .request("get_issue", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_issue, new_issue);

        let new_il = bridle_sdk::models::IssueLabel {
            id: 1,
            issue_id: 1,
            label_id: 1,
        };
        let _: () = client
            .request("create_issue_label", jsonrpsee::rpc_params![&new_il])
            .await?;
        let fetched_il: bridle_sdk::models::IssueLabel = client
            .request("get_issue_label", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_il, new_il);

        let new_pr = bridle_sdk::models::PullRequest {
            id: 1,
            repo_id: 1,
            number: 1,
            title: "pr1".to_string(),
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
        let _: () = client
            .request("create_pull_request", jsonrpsee::rpc_params![&new_pr])
            .await?;
        let fetched_pr: bridle_sdk::models::PullRequest = client
            .request("get_pull_request", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_pr, new_pr);

        let new_rev = bridle_sdk::models::PullRequestReview {
            id: 1,
            pr_id: 1,
            user_id: 1,
            state: "approved".to_string(),
            body: None,
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request(
                "create_pull_request_review",
                jsonrpsee::rpc_params![&new_rev],
            )
            .await?;
        let fetched_rev: bridle_sdk::models::PullRequestReview = client
            .request("get_pull_request_review", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_rev, new_rev);

        let new_rel = bridle_sdk::models::Release {
            id: 1,
            repo_id: 1,
            tag_name: "v1.0".to_string(),
            target_commitish: "main".to_string(),
            name: None,
            body: None,
            is_draft: false,
            is_prerelease: false,
            author_id: 1,
            created_at: now,
            published_at: None,
        };
        let _: () = client
            .request("create_release", jsonrpsee::rpc_params![&new_rel])
            .await?;
        let fetched_rel: bridle_sdk::models::Release = client
            .request("get_release", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_rel, new_rel);

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
        let _: () = client
            .request("create_webhook", jsonrpsee::rpc_params![&new_wh])
            .await?;
        let fetched_wh: bridle_sdk::models::Webhook = client
            .request("get_webhook", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_wh, new_wh);

        let new_commit = bridle_sdk::models::Commit {
            id: 1,
            repo_id: 1,
            sha: "c1".to_string(),
            tree_sha: "t1".to_string(),
            parent_shas: "".to_string(),
            message: "msg".to_string(),
            author_name: "a".to_string(),
            author_email: "a@b.c".to_string(),
            author_date: now,
            committer_name: "a".to_string(),
            committer_email: "a@b.c".to_string(),
            committer_date: now,
            created_at: now,
        };
        let _: () = client
            .request("create_commit", jsonrpsee::rpc_params![&new_commit])
            .await?;
        let fetched_commit: bridle_sdk::models::Commit = client
            .request("get_commit", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_commit, new_commit);

        let new_tree = bridle_sdk::models::Tree {
            id: 1,
            repo_id: 1,
            sha: "t1".to_string(),
            entries: "[]".to_string(),
            created_at: now,
        };
        let _: () = client
            .request("create_tree", jsonrpsee::rpc_params![&new_tree])
            .await?;
        let fetched_tree: bridle_sdk::models::Tree = client
            .request("get_tree", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_tree, new_tree);

        let new_blob = bridle_sdk::models::Blob {
            id: 1,
            repo_id: 1,
            sha: "b1".to_string(),
            size: 5,
            content: Some(b"hello".to_vec()),
            created_at: now,
        };
        let _: () = client
            .request("create_blob", jsonrpsee::rpc_params![&new_blob])
            .await?;
        let fetched_blob: bridle_sdk::models::Blob = client
            .request("get_blob", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_blob, new_blob);

        let new_job = bridle_sdk::models::BatchJob {
            id: 1,
            pipeline_name: "pipe".to_string(),
            status: "Pending".to_string(),
            started_at: now,
            completed_at: None,
        };
        let _: () = client
            .request("create_batch_job", jsonrpsee::rpc_params![&new_job])
            .await?;
        let fetched_job: bridle_sdk::models::BatchJob = client
            .request("get_batch_job", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_job, new_job);

        let new_task = bridle_sdk::models::BatchTask {
            id: 1,
            job_id: 1,
            repo_id: 1,
            status: "Pending".to_string(),
            error_reason: None,
            pr_url: None,
            created_at: now,
            updated_at: now,
        };
        let _: () = client
            .request("create_batch_task", jsonrpsee::rpc_params![&new_task])
            .await?;
        let fetched_task: bridle_sdk::models::BatchTask = client
            .request("get_batch_task", jsonrpsee::rpc_params![1])
            .await?;
        assert_eq!(fetched_task, new_task);

        Ok(())
    }

    #[tokio::test]
    async fn test_batch_and_sync_rpc() -> Result<(), BridleError> {
        let (_tf, db_url) = get_test_db();
        let addr = run_server(db_url).await?;
        let url = format!("http://{}", addr);
        let client = HttpClientBuilder::default().build(url)?;

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

        let req_batch_ok = bridle_sdk::models::BatchRunRequest {
            config_path: "config.yml".to_string(),
            safety_mode: true,
            max_repos: Some(1),
            max_prs_per_hour: Some(1),
        };
        let res_batch_ok: String = client
            .request("batch_run", jsonrpsee::rpc_params![req_batch_ok])
            .await?;
        assert_eq!(res_batch_ok, "Batch pipeline run from config.yml");

        let tmp_ws = tempfile::tempdir()?;
        let org_dir = tmp_ws
            .path()
            .join(".bridle")
            .join("workspace")
            .join("ok_org_rpc");
        std::fs::create_dir_all(&org_dir)?;
        unsafe {
            std::env::set_var("HOME", tmp_ws.path());
        }
        let req_fix_ok = bridle_sdk::models::BatchFixRequest {
            org: "ok_org_rpc".to_string(),
            issue: "test".to_string(),
            pattern: None,
            tools: None,
            tool_args: None,
            safety_mode: true,
            max_repos: Some(1),
            max_prs_per_hour: Some(1),
        };
        let res_fix_ok: String = client
            .request("batch_fix", jsonrpsee::rpc_params![req_fix_ok])
            .await?;
        assert!(res_fix_ok.contains("Batch fix applied"));

        let req_fix = bridle_sdk::models::BatchFixRequest {
            org: "test_nonexistent_org".to_string(),
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

        let req_sync = bridle_sdk::models::SyncPrsRequest {
            org: "test".to_string(),
            max_prs_per_hour: Some(1),
            fork_org: None,
        };
        let res_sync: String = client
            .request("sync_prs", jsonrpsee::rpc_params![req_sync])
            .await?;
        assert_eq!(res_sync, "Successfully synced 1 PR(s).");

        Ok(())
    }

    #[tokio::test]
    async fn test_rpc_error_paths() -> Result<(), BridleError> {
        let (_tf, db_url) = get_test_db();
        let addr = run_server(db_url).await?;
        let url = format!("http://{}", addr);
        let client = HttpClientBuilder::default().build(url)?;

        let endpoints = [
            "get_user",
            "get_org",
            "get_repo",
            "get_team",
            "get_branch",
            "get_branch_protection_rule",
            "get_key",
            "get_follow",
            "get_star",
            "get_org_membership",
            "get_repo_collaborator",
            "get_milestone",
            "get_label",
            "get_issue",
            "get_issue_label",
            "get_pull_request",
            "get_pull_request_review",
            "get_release",
            "get_webhook",
            "get_commit",
            "get_tree",
            "get_blob",
            "get_batch_job",
            "get_batch_task",
        ];

        for ep in endpoints {
            let res: Result<jsonrpsee::core::JsonValue, _> =
                client.request(ep, jsonrpsee::rpc_params![-1]).await;
            assert!(res.is_err());

            let create_ep = ep.replace("get_", "create_");
            let res_create: Result<jsonrpsee::core::JsonValue, _> =
                client.request(&create_ep, jsonrpsee::rpc_params![()]).await;
            assert!(res_create.is_err());
        }

        let err2 = rpc_reg_err("test");
        assert!(matches!(err2, BridleError::Register(_)));

        Ok(())
    }
}
