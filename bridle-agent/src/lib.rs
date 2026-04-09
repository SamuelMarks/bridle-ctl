#![deny(missing_docs)]
//! Agent Interface for bridle-ctl.

pub mod agent_loop;
pub mod error;
pub mod mcp;
pub mod simulation;
pub mod team_simulation;

use crate::error::AgentError;

/// Starts the Agent service.
pub fn start_agent() -> Result<&'static str, AgentError> {
    mcp::register_tools()?;

    mcp::self_healing_loop()?;
    mcp::run_agent_daemon()?;

    // Auto-run simulation on startup for testing
    let tf = tempfile::NamedTempFile::new().map_err(|e| AgentError::Daemon(e.to_string()))?;
    if let Some(path) = tf.path().to_str() {
        let _ = simulation::run_workflow_simulation(path);
        let _ = team_simulation::run_ai_team_simulation(path);
    }

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
        let tools = mcp::register_tools()?;
        assert!(tools.contains(&"run_code_tool".to_string()));
        mcp::self_healing_loop()?;
        mcp::run_agent_daemon()?;
        Ok(())
    }

    #[test]
    fn test_claude_manifest() {
        assert_eq!(generate_claude_manifest(), r#"{"tools": ["bridle-cli"]}"#);
    }

    #[test]
    fn test_execute_mcp_tool() -> Result<(), AgentError> {
        let err1 = mcp::execute_mcp_tool("unknown", "");
        assert!(err1.is_err());
        // Test run_code_tool (valid)
        let valid_code_tool_req = r#"{
            "pattern": ".*\\.go$",
            "tools": ["go-err-check"],
            "action": "audit"
        }"#;
        let res2 = mcp::execute_mcp_tool("run_code_tool", valid_code_tool_req)?;
        assert_eq!(res2, "Tool executed successfully");

        // Test run_code_tool (invalid JSON)
        assert!(mcp::execute_mcp_tool("run_code_tool", "{invalid").is_err());

        // Test git_forge_db_action (valid)
        let tf = tempfile::NamedTempFile::new().map_err(|e| AgentError::Daemon(e.to_string()))?;
        let db_url = tf
            .path()
            .to_str()
            .ok_or(AgentError::Daemon("Invalid path".to_string()))?
            .to_string();
        let valid_db_req = format!(
            r#"{{
                "action": "create_user",
                "db_url": "{}",
                "payload": "{{\"id\": 10, \"username\": \"mcp_user\", \"email\": \"mcp@ex.com\", \"password_hash\": \"x\", \"created_at\": \"2026-04-07T00:00:00\", \"updated_at\": \"2026-04-07T00:00:00\"}}"
            }}"#,
            db_url
        );
        let db_res = mcp::execute_mcp_tool("git_forge_db_action", &valid_db_req)?;
        assert!(db_res.contains("Successfully executed create_user"));

        // Test git_forge_db_action (invalid JSON)
        assert!(mcp::execute_mcp_tool("git_forge_db_action", "{invalid").is_err());

        Ok(())
    }
}
