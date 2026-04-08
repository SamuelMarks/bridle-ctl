#![warn(missing_docs)]
//! CLI Interface for bridle-ctl.

use bridle_cli::{error, runner};
use clap::{Parser, Subcommand};

/// Bridle CLI tool for agentic and manual codebase operations.
#[derive(Parser)]
#[command(name = "bridle", version = "0.1.0", author, about = "CLI for bridle-ctl", long_about = None)]
pub struct Cli {
    /// The subcommand to run
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI subcommands.
#[derive(Subcommand, PartialEq, Debug)]
pub enum Commands {
    /// Starts the REST API server.
    Rest,
    /// Starts the JSON-RPC server.
    Rpc,
    /// Starts the Agent daemon.
    Agent,
    /// Performs a health check.
    Health,
    /// Run an audit to analyze code issues.
    Audit {
        /// Target a specific regex pattern
        #[arg(long)]
        pattern: Option<String>,
        /// Comma-separated list of tools to run
        #[arg(long, value_delimiter = ',')]
        tools: Option<Vec<String>>,
        /// Arguments for specific tools, e.g., tool1:--arg=val
        #[arg(long, value_delimiter = ',')]
        tool_args: Option<Vec<String>>,
    },
    /// Fix code issues automatically.
    Fix {
        /// Target a specific regex pattern
        #[arg(long)]
        pattern: Option<String>,
        /// Comma-separated list of tools to run
        #[arg(long, value_delimiter = ',')]
        tools: Option<Vec<String>>,
        /// Arguments for specific tools, e.g., tool1:--arg=val
        #[arg(long, value_delimiter = ',')]
        tool_args: Option<Vec<String>>,
        /// Perform a dry-run without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Add two numbers together using the SDK (placeholder command).
    Add {
        /// The left number
        #[arg(long)]
        left: usize,
        /// The right number
        #[arg(long)]
        right: usize,
    },
    /// Execute a database operation via JSON.
    Db {
        /// The path to the SQLite database.
        #[arg(long, default_value = "bridle.db")]
        db_url: String,
        /// The action to perform (e.g. "create_user", "get_user", "create_team", "get_team", etc).
        #[arg(long)]
        action: String,
        /// The JSON payload to insert (only needed for create_* actions).
        #[arg(long)]
        payload: Option<String>,
        /// The ID to fetch (only needed for get_* actions).
        #[arg(long)]
        id: Option<i32>,
    },
}

/// Parses a list of strings like `tool:arg1` into a mapping of tool to its arguments.
fn parse_tool_args(args: Option<Vec<String>>) -> std::collections::HashMap<String, Vec<String>> {
    let mut map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    if let Some(list) = args {
        for item in list {
            if let Some((tool, arg)) = item.split_once(':') {
                map.entry(tool.to_string())
                    .or_default()
                    .push(arg.to_string());
            }
        }
    }
    map
}

/// Macro to simplify generating CRUD CLI commands that serialize and deserialize JSON logic.
macro_rules! handle_cli_crud {
    ($action:expr, $db_url:expr, $payload:expr, $id:expr, $( ($create_name:expr, $get_name:expr, $sdk_insert:path, $sdk_get:path, $model:ty) ),+ $(,)?) => {
        match $action {
            $(
                $create_name => {
                    let data = $payload.ok_or_else(|| error::CliError::Execution(format!("Missing payload for {}", $create_name)))?;
                    let parsed: $model = serde_json::from_str(&data).map_err(|e| error::CliError::Execution(e.to_string()))?;
                    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations($db_url)
                        .map_err(|e| error::CliError::Execution(e.to_string()))?;
                    $sdk_insert(&mut conn, &parsed)
                        .map_err(|e| error::CliError::Execution(e.to_string()))?;
                    Ok(format!("Successfully executed {}", $create_name))
                }
                $get_name => {
                    let target_id = $id.ok_or_else(|| error::CliError::Execution(format!("Missing id for {}", $get_name)))?;
                    let mut conn = bridle_sdk::db::establish_connection_and_run_migrations($db_url)
                        .map_err(|e| error::CliError::Execution(e.to_string()))?;
                    let fetched = $sdk_get(&mut conn, target_id)
                        .map_err(|e| error::CliError::Execution(e.to_string()))?;
                    let json = serde_json::to_string_pretty(&fetched).map_err(|e| error::CliError::Execution(e.to_string()))?;
                    Ok(json)
                }
            )+
            _ => Err(error::CliError::Execution(format!("Unknown action: {}", $action)))
        }
    };
}

#[cfg(not(tarpaulin_include))]
fn execute_db_command(
    db_url: &str,
    action: &str,
    payload: Option<String>,
    id: Option<i32>,
) -> Result<String, error::CliError> {
    handle_cli_crud!(
        action,
        db_url,
        payload,
        id,
        (
            "create_user",
            "get_user",
            bridle_sdk::db::insert_user,
            bridle_sdk::db::get_user,
            bridle_sdk::models::User
        ),
        (
            "create_org",
            "get_org",
            bridle_sdk::db::insert_organisation,
            bridle_sdk::db::get_organisation,
            bridle_sdk::models::Organisation
        ),
        (
            "create_repo",
            "get_repo",
            bridle_sdk::db::insert_repository,
            bridle_sdk::db::get_repository,
            bridle_sdk::models::Repository
        ),
        (
            "create_team",
            "get_team",
            bridle_sdk::db::insert_team,
            bridle_sdk::db::get_team,
            bridle_sdk::models::Team
        ),
        (
            "create_branch",
            "get_branch",
            bridle_sdk::db::insert_branch,
            bridle_sdk::db::get_branch,
            bridle_sdk::models::Branch
        ),
        (
            "create_branch_protection_rule",
            "get_branch_protection_rule",
            bridle_sdk::db::insert_branch_protection_rule,
            bridle_sdk::db::get_branch_protection_rule,
            bridle_sdk::models::BranchProtectionRule
        ),
        (
            "create_key",
            "get_key",
            bridle_sdk::db::insert_key,
            bridle_sdk::db::get_key,
            bridle_sdk::models::Key
        ),
        (
            "create_follow",
            "get_follow",
            bridle_sdk::db::insert_follow,
            bridle_sdk::db::get_follow,
            bridle_sdk::models::Follow
        ),
        (
            "create_star",
            "get_star",
            bridle_sdk::db::insert_star,
            bridle_sdk::db::get_star,
            bridle_sdk::models::Star
        ),
        (
            "create_org_membership",
            "get_org_membership",
            bridle_sdk::db::insert_org_membership,
            bridle_sdk::db::get_org_membership,
            bridle_sdk::models::OrgMembership
        ),
        (
            "create_repo_collaborator",
            "get_repo_collaborator",
            bridle_sdk::db::insert_repo_collaborator,
            bridle_sdk::db::get_repo_collaborator,
            bridle_sdk::models::RepoCollaborator
        ),
        (
            "create_milestone",
            "get_milestone",
            bridle_sdk::db::insert_milestone,
            bridle_sdk::db::get_milestone,
            bridle_sdk::models::Milestone
        ),
        (
            "create_label",
            "get_label",
            bridle_sdk::db::insert_label,
            bridle_sdk::db::get_label,
            bridle_sdk::models::Label
        ),
        (
            "create_issue",
            "get_issue",
            bridle_sdk::db::insert_issue,
            bridle_sdk::db::get_issue,
            bridle_sdk::models::Issue
        ),
        (
            "create_issue_label",
            "get_issue_label",
            bridle_sdk::db::insert_issue_label,
            bridle_sdk::db::get_issue_label,
            bridle_sdk::models::IssueLabel
        ),
        (
            "create_pull_request",
            "get_pull_request",
            bridle_sdk::db::insert_pull_request,
            bridle_sdk::db::get_pull_request,
            bridle_sdk::models::PullRequest
        ),
        (
            "create_pull_request_review",
            "get_pull_request_review",
            bridle_sdk::db::insert_pull_request_review,
            bridle_sdk::db::get_pull_request_review,
            bridle_sdk::models::PullRequestReview
        ),
        (
            "create_release",
            "get_release",
            bridle_sdk::db::insert_release,
            bridle_sdk::db::get_release,
            bridle_sdk::models::Release
        ),
        (
            "create_webhook",
            "get_webhook",
            bridle_sdk::db::insert_webhook,
            bridle_sdk::db::get_webhook,
            bridle_sdk::models::Webhook
        ),
        (
            "create_commit",
            "get_commit",
            bridle_sdk::db::insert_commit,
            bridle_sdk::db::get_commit,
            bridle_sdk::models::Commit
        ),
        (
            "create_tree",
            "get_tree",
            bridle_sdk::db::insert_tree,
            bridle_sdk::db::get_tree,
            bridle_sdk::models::Tree
        ),
        (
            "create_blob",
            "get_blob",
            bridle_sdk::db::insert_blob,
            bridle_sdk::db::get_blob,
            bridle_sdk::models::Blob
        )
    )
}

/// Executes the provided command.
pub fn execute(command: &Commands) -> Result<String, error::CliError> {
    match command {
        Commands::Rest => {
            #[cfg(not(test))]
            {
                let mut child = std::process::Command::new("cargo")
                    .args(["run", "-p", "bridle-rest"])
                    .spawn()?;
                let _ = child.wait()?;
            }
            Ok("REST API stopped.".to_string())
        }
        Commands::Rpc => {
            #[cfg(not(test))]
            {
                let mut child = std::process::Command::new("cargo")
                    .args(["run", "-p", "bridle-rpc"])
                    .spawn()?;
                let _ = child.wait()?;
            }
            Ok("JSON-RPC stopped.".to_string())
        }
        Commands::Agent => {
            #[cfg(not(test))]
            {
                let mut child = std::process::Command::new("cargo")
                    .args(["run", "-p", "bridle-agent"])
                    .spawn()?;
                let _ = child.wait()?;
            }
            Ok("Agent stopped.".to_string())
        }
        Commands::Health => Ok("Health check OK".to_string()),
        Commands::Audit {
            pattern,
            tools,
            tool_args,
        } => {
            let parsed_args = parse_tool_args(tool_args.clone());
            let req = bridle_sdk::models::ToolRunRequest {
                pattern: pattern.clone(),
                tools: tools.clone(),
                tool_args: if parsed_args.is_empty() {
                    None
                } else {
                    Some(parsed_args)
                },
                dry_run: None,
                action: Some("audit".to_string()),
            };
            runner::run(runner::Action::Audit, req)?;
            Ok("Audit completed.".to_string())
        }
        Commands::Fix {
            pattern,
            tools,
            tool_args,
            dry_run,
        } => {
            let parsed_args = parse_tool_args(tool_args.clone());
            let req = bridle_sdk::models::ToolRunRequest {
                pattern: pattern.clone(),
                tools: tools.clone(),
                tool_args: if parsed_args.is_empty() {
                    None
                } else {
                    Some(parsed_args)
                },
                dry_run: Some(*dry_run),
                action: Some("fix".to_string()),
            };
            runner::run(runner::Action::Fix { dry_run: *dry_run }, req)?;
            Ok("Fix completed.".to_string())
        }
        Commands::Add { left, right } => {
            let result = bridle_sdk::add(*left, *right);
            Ok(format!("{} + {} = {}", left, right, result))
        }
        Commands::Db {
            db_url,
            action,
            payload,
            id,
        } => execute_db_command(db_url, action, payload.clone(), *id),
    }
}

/// Main entry point for the CLI.
#[cfg(not(tarpaulin_include))]
fn main() {
    let cli = Cli::parse();
    match execute(&cli.command) {
        Ok(msg) => println!("{}", msg),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn test_parse_tool_args() {
        let args = vec![
            "tool1:arg1".to_string(),
            "tool1:arg2".to_string(),
            "tool2:arg3".to_string(),
        ];
        let map = parse_tool_args(Some(args));
        let expected_tool1 = vec!["arg1".to_string(), "arg2".to_string()];
        let expected_tool2 = vec!["arg3".to_string()];
        assert_eq!(map.get("tool1").unwrap_or(&vec![]), &expected_tool1);
        assert_eq!(map.get("tool2").unwrap_or(&vec![]), &expected_tool2);

        let empty_map = parse_tool_args(None);
        assert!(empty_map.is_empty());
    }

    #[test]
    fn test_execute_commands() -> Result<(), error::CliError> {
        assert_eq!(execute(&Commands::Rest)?, "REST API stopped.");
        assert_eq!(execute(&Commands::Rpc)?, "JSON-RPC stopped.");
        assert_eq!(execute(&Commands::Agent)?, "Agent stopped.");
        assert_eq!(execute(&Commands::Health)?, "Health check OK");

        assert_eq!(
            execute(&Commands::Audit {
                pattern: Some("unknown".to_string()),
                tools: None,
                tool_args: None,
            })?,
            "Audit completed."
        );

        assert_eq!(
            execute(&Commands::Fix {
                pattern: Some("unknown".to_string()),
                tools: None,
                tool_args: None,
                dry_run: true,
            })?,
            "Fix completed."
        );

        assert_eq!(execute(&Commands::Add { left: 2, right: 3 })?, "2 + 3 = 5");

        let tf = tempfile::NamedTempFile::new().unwrap();
        let db_url = tf.path().to_str().unwrap().to_string();

        let new_user = bridle_sdk::models::User {
            id: 11,
            username: "clitester".to_string(),
            email: "cli@example.com".to_string(),
            password_hash: "hash".to_string(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };
        let payload = serde_json::to_string(&new_user)
            .map_err(|e| error::CliError::Execution(e.to_string()))?;

        assert_eq!(
            execute(&Commands::Db {
                db_url: db_url.clone(),
                action: "create_user".to_string(),
                payload: Some(payload),
                id: None,
            })?,
            "Successfully executed create_user"
        );

        let fetched = execute(&Commands::Db {
            db_url: db_url.clone(),
            action: "get_user".to_string(),
            payload: None,
            id: Some(11),
        })?;
        assert!(fetched.contains("clitester"));

        // test missing payload/id errors
        assert!(
            execute(&Commands::Db {
                db_url: db_url.clone(),
                action: "create_user".to_string(),
                payload: None,
                id: None
            })
            .is_err()
        );
        assert!(
            execute(&Commands::Db {
                db_url: db_url.clone(),
                action: "get_user".to_string(),
                payload: None,
                id: None
            })
            .is_err()
        );
        assert!(
            execute(&Commands::Db {
                db_url: db_url.clone(),
                action: "unknown_action".to_string(),
                payload: None,
                id: None
            })
            .is_err()
        );

        Ok(())
    }
}
