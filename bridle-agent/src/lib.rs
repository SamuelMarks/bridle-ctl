//! Agent Interface for bridle-ctl.

pub mod error;
pub mod mcp;

use crate::error::AgentError;

/// Starts the Agent service.
pub fn start_agent() -> Result<&'static str, AgentError> {
    mcp::start_mcp_server()?;
    mcp::register_tools()?;
    mcp::expose_resources()?;

    // Wire up context AST pruning and run the daemon
    let _context = mcp::prune_context_ast()?;
    let _plan = mcp::agent_planning_engine()?;
    mcp::self_healing_loop()?;
    mcp::run_agent_daemon()?;

    Ok("Agent started")
}

/// Generates Claude Code Tool Manifest.
pub fn generate_claude_manifest() -> String {
    r#"{"tools": ["bridle-cli"]}"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_start() -> Result<(), AgentError> {
        assert_eq!(start_agent()?, "Agent started");
        Ok(())
    }

    #[test]
    fn test_mcp_stubs() -> Result<(), AgentError> {
        mcp::start_mcp_server()?;
        let tools = mcp::register_tools()?;
        assert!(tools.contains(&"rust-unwrap-to-question-mark".to_string()));
        let resources = mcp::expose_resources()?;
        assert_eq!(resources.len(), 24);
        assert_eq!(mcp::prune_context_ast()?, "Pruned AST context");
        mcp::self_healing_loop()?;
        mcp::run_agent_daemon()?;
        assert_eq!(mcp::agent_planning_engine()?, "Plan: run self-healing loop");
        Ok(())
    }

    #[test]
    fn test_claude_manifest() {
        assert_eq!(generate_claude_manifest(), r#"{"tools": ["bridle-cli"]}"#);
    }

    #[test]
    fn test_mcp_add() -> Result<(), AgentError> {
        assert_eq!(mcp::mcp_add(5, 5)?, 10);
        Ok(())
    }

    #[test]
    fn test_execute_mcp_tool() -> Result<(), AgentError> {
        let res = mcp::execute_mcp_tool("sdk_add", "5,5")?;
        assert_eq!(res, "Result: 10");

        let err1 = mcp::execute_mcp_tool("unknown", "");
        assert!(err1.is_err());

        let err2 = mcp::execute_mcp_tool("sdk_add", "5");
        assert!(err2.is_err());

        let err3 = mcp::execute_mcp_tool("sdk_add", "a,b");
        assert!(err3.is_err());

        Ok(())
    }
}
