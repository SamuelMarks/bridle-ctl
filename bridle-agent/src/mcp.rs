//! Model Context Protocol (MCP) Server for `bridle-ctl`.

use crate::error::AgentError;
use bridle_sdk::models::ToolRunRequest;

/// Starts the MCP server over STDIO.
pub fn start_mcp_server() -> Result<(), AgentError> {
    println!("MCP Server started over STDIO.");
    Ok(())
}

/// Registers AI tools such as `rust-unwrap-to-question-mark`, `fix_go_errors`.
pub fn register_tools() -> Result<Vec<String>, AgentError> {
    let tools = bridle_cli::tools::registry::get_tools();
    let mut tool_names: Vec<String> = tools.iter().map(|t| t.name().to_string()).collect();

    if !tool_names
        .iter()
        .any(|n| n == "rust-unwrap-to-question-mark")
    {
        return Err(AgentError::Daemon(
            "Required tool 'rust-unwrap-to-question-mark' not found in registry".to_string(),
        ));
    }

    tool_names.push("sdk_add".to_string());

    Ok(tool_names)
}

/// Exposes resources like project context and AST dumps.
pub fn expose_resources() -> Result<Vec<String>, AgentError> {
    let mut resources = vec!["workspace_ast".to_string(), "git_diff".to_string()];

    let models = vec![
        "User",
        "Organisation",
        "Team",
        "Repository",
        "Branch",
        "BranchProtectionRule",
        "Key",
        "Follow",
        "Star",
        "OrgMembership",
        "RepoCollaborator",
        "Milestone",
        "Label",
        "Issue",
        "IssueLabel",
        "PullRequest",
        "PullRequestReview",
        "Release",
        "Webhook",
        "Commit",
        "Tree",
        "Blob",
    ];

    for model in models {
        resources.push(format!("db_schema_{}", model));
    }

    Ok(resources)
}

/// Helper for AST-aware context pruning.
pub fn prune_context_ast() -> Result<String, AgentError> {
    Ok("Pruned AST context".to_string())
}

/// Self-healing loop mechanism for autonomous agent mode.
pub fn self_healing_loop() -> Result<(), AgentError> {
    println!("Running self-healing loop...");
    let req = ToolRunRequest {
        pattern: None,
        tools: Some(vec![
            "rust-unwrap-to-question-mark".to_string(),
            "go-err-check".to_string(),
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

/// Agent natural language planning engine.
pub fn agent_planning_engine() -> Result<String, AgentError> {
    Ok("Plan: run self-healing loop".to_string())
}

/// Add two numbers by delegating to the Bridle SDK.
pub fn mcp_add(left: usize, right: usize) -> Result<usize, AgentError> {
    Ok(bridle_sdk::add(left, right))
}

/// Executes a registered MCP tool by name.
pub fn execute_mcp_tool(name: &str, args: &str) -> Result<String, AgentError> {
    if name == "sdk_add" {
        let parts: Vec<&str> = args.split(',').collect();
        if parts.len() != 2 {
            return Err(AgentError::Daemon(
                "Invalid arguments for sdk_add".to_string(),
            ));
        }
        let left = parts[0]
            .trim()
            .parse::<usize>()
            .map_err(|e| AgentError::Daemon(e.to_string()))?;
        let right = parts[1]
            .trim()
            .parse::<usize>()
            .map_err(|e| AgentError::Daemon(e.to_string()))?;
        let result = mcp_add(left, right)?;
        return Ok(format!("Result: {}", result));
    }
    Err(AgentError::Daemon(format!(
        "Tool not implemented or unknown: {}",
        name
    )))
}

/// Agent daemon mode.
pub fn run_agent_daemon() -> Result<(), AgentError> {
    start_mcp_server()?;
    let tools = register_tools()?;
    expose_resources()?;
    let _ = prune_context_ast()?;

    let plan = agent_planning_engine()?;
    println!("Agent plan: {}", plan);

    self_healing_loop()?;

    if tools.contains(&"sdk_add".to_string()) {
        let add_res = execute_mcp_tool("sdk_add", "5,7")?;
        println!("Agent automated test of sdk_add: {}", add_res);
    }

    Ok(())
}
