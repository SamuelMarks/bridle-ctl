//! Model Context Protocol (MCP) Server for `bridle-ctl`.

use crate::error::AgentError;
use bridle_sdk::models::ToolRunRequest;
use serde::{Deserialize, Serialize};

/// Registers AI tools such as `run_code_tool`, `git_forge_db_action`.
pub fn register_tools() -> Result<Vec<String>, AgentError> {
    let tool_names = vec![
        "run_code_tool".to_string(),
        "git_forge_db_action".to_string(),
    ];

    Ok(tool_names)
}

/// Self-healing loop mechanism for autonomous agent mode.
pub fn self_healing_loop() -> Result<(), AgentError> {
    println!("Running self-healing loop...");
    let req = ToolRunRequest {
        pattern: None,
        tools: Some(vec![
            "rust-unwrap-to-question-mark".to_string(),
            "rust-unwrap-to-question-mark".to_string(),
        ]),
        tool_args: None,
        dry_run: Some(false),
        action: Some("fix".to_string()),
    };

    // Convert cli error to AgentError via string message if it fails
    bridle_cli::runner::run(bridle_cli::runner::Action::Fix { dry_run: false }, req)
        .map_err(|e| AgentError::Daemon(format!("Self-healing failed: {}", e)))?;

    Ok(())
}

/// Argument payload for the git_forge_db_action tool.
#[derive(Serialize, Deserialize)]
pub struct DbActionArgs {
    /// Action to perform, e.g., "create_user" or "get_user"
    pub action: String,
    /// Database URL, e.g. "bridle.db"
    #[serde(default = "default_db_url")]
    pub db_url: String,
    /// JSON string of the payload
    pub payload: Option<String>,
    /// ID for get_* actions
    pub id: Option<i32>,
}

/// Provides the default database URL.
fn default_db_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| "bridle.db".to_string())
}

/// Executes a registered MCP tool by name.
#[cfg(not(tarpaulin_include))]
pub fn execute_mcp_tool(name: &str, args: &str) -> Result<String, AgentError> {
    if name == "run_code_tool" {
        let req: ToolRunRequest = serde_json::from_str(args)
            .map_err(|e| AgentError::Daemon(format!("Invalid ToolRunRequest JSON: {}", e)))?;

        let action = match req.action.as_deref() {
            Some("audit") => bridle_cli::runner::Action::Audit,
            _ => bridle_cli::runner::Action::Fix {
                dry_run: req.dry_run.unwrap_or(false),
            },
        };

        bridle_cli::runner::run(action, req)
            .map_err(|e| AgentError::Daemon(format!("CodeTool execution failed: {}", e)))?;

        return Ok("Tool executed successfully".to_string());
    }

    if name == "git_forge_db_action" {
        let req: DbActionArgs = serde_json::from_str(args)
            .map_err(|e| AgentError::Daemon(format!("Invalid DbActionArgs JSON: {}", e)))?;

        let result =
            bridle_cli::db::execute_db_command(&req.db_url, &req.action, req.payload, req.id)
                .map_err(|e| AgentError::Daemon(format!("DB Action failed: {}", e)))?;

        return Ok(result);
    }

    Err(AgentError::Daemon(format!(
        "Tool not implemented or unknown: {}",
        name
    )))
}

/// Agent daemon mode.
pub fn run_agent_daemon() -> Result<(), AgentError> {
    let _tools = register_tools()?;

    self_healing_loop()?;

    let db_url = default_db_url();
    let loop_res = crate::agent_loop::start_agent_loop(&db_url)?;
    println!("Agent loop: {}", loop_res);

    Ok(())
}
