/// Contains the registry of all available code tools.
pub mod registry;

use crate::error::CliError;
use bridle_sdk::path_scope::PathScope;

/// Defines a standard interface for code processing tools.
pub trait CodeTool {
    /// Returns the unique name of the tool, e.g., "tool0"
    fn name(&self) -> &'static str;
    /// A short description of what the tool does
    fn description(&self) -> &'static str;
    /// The regex pattern of files this tool targets, e.g., r".*\.rs$"
    fn match_regex(&self) -> &'static str;

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
