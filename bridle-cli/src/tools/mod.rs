/// Plugin configuration
pub mod config;
/// Dynamic plugin tools (dlopen, subprocess, jsonrpc)
pub mod dynamic;
/// Contains the registry of all available code tools.
pub mod registry;

use crate::error::CliError;
use bridle_sdk::path_scope::PathScope;

/// Defines a standard interface for code processing tools.
pub trait CodeTool: Send + Sync {
    /// Returns the unique name of the tool, e.g., "tool0"
    fn name(&self) -> &str;
    /// A short description of what the tool does
    fn description(&self) -> &str;
    /// The regex pattern of files this tool targets, e.g., r".*\.rs$"
    fn match_regex(&self) -> &str;

    /// The version of the tool, if specified
    fn version(&self) -> Option<&str> {
        None
    }
    /// The author of the tool, if specified
    fn author(&self) -> Option<&str> {
        None
    }
    /// The URL of the tool's homepage or repository, if specified
    fn url(&self) -> Option<&str> {
        None
    }
    /// The license of the tool, if specified
    fn license(&self) -> Option<&str> {
        None
    }

    /// Runs the audit logic (checking for issues)
    fn audit(&self, args: &[String], scope: Option<&PathScope>) -> Result<String, CliError>;

    /// Runs the fix logic (automatically fixing issues)
    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        scope: Option<&PathScope>,
    ) -> Result<String, CliError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyTool;
    impl CodeTool for DummyTool {
        fn name(&self) -> &str {
            "dummy"
        }
        fn description(&self) -> &str {
            "dummy tool"
        }
        fn match_regex(&self) -> &str {
            ".*"
        }
        fn audit(&self, _args: &[String], _scope: Option<&PathScope>) -> Result<String, CliError> {
            Ok("ok".into())
        }
        fn fix(
            &self,
            _args: &[String],
            _dry_run: bool,
            _scope: Option<&PathScope>,
        ) -> Result<String, CliError> {
            Ok("ok".into())
        }
    }

    #[test]
    fn test_code_tool_defaults() {
        let tool = DummyTool;
        let dyn_tool: &dyn CodeTool = &tool;
        assert_eq!(dyn_tool.version(), None);
        assert_eq!(dyn_tool.author(), None);
        assert_eq!(dyn_tool.url(), None);
        assert_eq!(dyn_tool.license(), None);
    }
}
