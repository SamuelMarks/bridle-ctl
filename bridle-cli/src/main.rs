#![deny(missing_docs)]
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
    /// Ingests all repositories for an organization from an upstream provider.
    IngestOrg {
        /// The name of the organization.
        #[arg(long)]
        org: String,
        /// The upstream provider (e.g., github).
        #[arg(long, default_value = "github")]
        provider: String,
        /// Optional DB URL to sync the org.
        #[arg(long, default_value = "bridle.db")]
        db_url: String,
    },
    /// Executes a batch fix across all repositories in an organization.
    BatchFix {
        /// The name of the organization.
        #[arg(long)]
        org: String,
        /// The issue title or description.
        #[arg(long)]
        issue: String,
        /// Target a specific regex pattern (passed to tools).
        #[arg(long)]
        pattern: Option<String>,
        /// Comma-separated list of tools to run.
        #[arg(long, value_delimiter = ',')]
        tools: Option<Vec<String>>,
        /// Arguments for specific tools.
        #[arg(long, value_delimiter = ',')]
        tool_args: Option<Vec<String>>,
        /// Database URL.
        #[arg(long, default_value = "bridle.db")]
        db_url: String,
        /// If true, will not fork and submit PRs automatically.
        #[arg(long)]
        safety_mode: bool,
        /// Limit the maximum number of repositories processed.
        #[arg(long)]
        max_repos: Option<usize>,
        /// Global limit of number of PRs to send per hour.
        #[arg(long)]
        max_prs_per_hour: Option<usize>,
    },
    /// Syncs local PRs to the upstream provider.
    SyncPrs {
        /// The name of the organization.
        #[arg(long)]
        org: String,
        /// Database URL.
        #[arg(long, default_value = "bridle.db")]
        db_url: String,
        /// Global limit of number of PRs to send per hour.
        #[arg(long)]
        max_prs_per_hour: Option<usize>,
        /// Specific organization to fork to (if not personal account).
        #[arg(long)]
        fork_org: Option<String>,
    },
    /// Runs a batch pipeline configuration.
    BatchRun {
        /// Path to the YAML/TOML config file.
        #[arg(long)]
        config: String,
        /// If true, will not fork and submit PRs automatically.
        #[arg(long)]
        safety_mode: bool,
        /// Limit the maximum number of repositories processed.
        #[arg(long)]
        max_repos: Option<usize>,
        /// Global limit of number of PRs to send per hour.
        #[arg(long)]
        max_prs_per_hour: Option<usize>,
    },
    /// Resumes an interrupted batch run.
    BatchResume {
        /// ID of the batch job to resume.
        #[arg(long)]
        job_id: i32,
    },
    /// Displays the status of a batch run via TUI.
    BatchStatus {
        /// ID of the batch job to track.
        #[arg(long)]
        job_id: i32,
    },
}

/// Parses a list of strings like `tool:arg1` into a mapping of tool to its arguments.
#[cfg(not(tarpaulin_include))]
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

/// Executes the provided command.
#[cfg(not(tarpaulin_include))]
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

        Commands::Db {
            db_url,
            action,
            payload,
            id,
        } => bridle_cli::db::execute_db_command(db_url, action, payload.clone(), *id),
        Commands::IngestOrg {
            org,
            provider,
            db_url,
        } => bridle_cli::ingest::ingest_org(org, provider, db_url),
        Commands::BatchFix {
            org,
            issue,
            pattern,
            tools,
            tool_args,
            db_url,
            safety_mode,
            max_repos,
            max_prs_per_hour,
        } => {
            let parsed_args = tool_args.as_ref().map(|args| parse_tool_args(Some(args.clone())));
            bridle_cli::batch_fix::batch_fix(
                org,
                issue,
                pattern.clone(),
                tools.clone(),
                parsed_args,
                db_url,
                *safety_mode,
                *max_repos,
                *max_prs_per_hour,
            )
        }
        Commands::SyncPrs {
            org,
            db_url,
            max_prs_per_hour,
            fork_org,
        } => bridle_cli::sync_prs::sync_prs(org, db_url, *max_prs_per_hour, fork_org.clone()),
        Commands::BatchRun {
            config,
            safety_mode,
            max_repos,
            max_prs_per_hour,
        } => bridle_cli::batch_pipeline::run_pipeline(
            config,
            "bridle.db",
            *safety_mode,
            *max_repos,
            *max_prs_per_hour,
        ),
        Commands::BatchResume { job_id } => {
            bridle_cli::batch_pipeline::resume_pipeline(*job_id, "bridle.db")
        }
        Commands::BatchStatus { job_id } => {
            bridle_cli::batch_pipeline::status_pipeline(*job_id, "bridle.db")
        }
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
    fn test_execute_commands() -> Result<(), Box<dyn std::error::Error>> {
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

        let tf = tempfile::NamedTempFile::new()?;
        let db_url = tf.path().to_str().ok_or("invalid path")?.to_string();

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

        assert_eq!(
            execute(&Commands::BatchRun {
                config: "config.yml".to_string(),
                safety_mode: false,
                max_repos: None,
                max_prs_per_hour: None
            })?,
            "Batch pipeline run from config.yml"
        );
        assert_eq!(
            execute(&Commands::BatchResume { job_id: 123 })?,
            "Resumed batch job 123"
        );
        assert_eq!(
            execute(&Commands::BatchStatus { job_id: 123 })?,
            "Status of batch job 123"
        );

        Ok(())
    }
}
