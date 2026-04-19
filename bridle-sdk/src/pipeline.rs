use serde::{Deserialize, Serialize};

/// Represents the top-level configuration for a remediation pipeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PipelineConfig {
    /// Name of the pipeline.
    pub name: String,
    /// Description of the pipeline.
    pub description: Option<String>,
    /// Author of the pipeline.
    pub author: Option<String>,
    /// Paths allowed to be modified (globs). If empty, all are allowed.
    pub allowed_paths: Option<Vec<String>>,
    /// Paths ignored from modification (globs).
    pub ignored_paths: Option<Vec<String>>,
    /// Selectors to filter which repositories to run this pipeline on.
    pub selectors: Selectors,
    /// The steps to execute in this pipeline.
    pub steps: Vec<Step>,
    /// PR templating configuration.
    pub pr_template: Option<PrTemplate>,
}

/// Target selectors for a pipeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Selectors {
    /// Files that must be present in the repository.
    pub require_files: Option<Vec<String>>,
    /// Repository topics/tags that must match.
    pub topics: Option<Vec<String>>,
    /// Primary languages of the repository.
    pub languages: Option<Vec<String>>,
}

/// The type of a step.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    /// A step to detect an issue.
    Detect,
    /// A step to fix an issue.
    Fix,
    /// A step to validate the fix.
    Validate,
    /// A step to generate dockerfiles using mkconf and build.
    MkconfBuild,
}

/// A single step in the pipeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Step {
    /// Step name.
    pub name: String,
    /// Type of the step (detect, fix, validate).
    pub step_type: StepType,
    /// Command to execute.
    pub command: String,
    /// Arguments for the command.
    pub args: Option<Vec<String>>,
    /// Timeout in seconds.
    pub timeout_seconds: Option<u64>,
    /// Expected exit codes to consider successful.
    pub expected_exit_codes: Option<Vec<i32>>,
}

/// PR template fallback and variable mappings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrTemplate {
    /// Fallback template content.
    pub fallback: String,
    /// Optional variable mappings.
    pub variables: Option<std::collections::HashMap<String, String>>,
}
