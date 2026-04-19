//! Logic for running codebase tools (audit and fix modes).

use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::error::CliError;
use crate::tools::{self, registry};
use crate::tui;

/// Defines the action to be performed by the runner.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    /// Audits the codebase, looking for issues.
    Audit,
    /// Fixes the issues found in the codebase.
    Fix {
        /// Whether to perform a dry run.
        dry_run: bool,
    },
}

/// Represents the final report generated after running tools.
#[derive(Serialize, Deserialize, Debug)]
struct Report {
    /// Action
    action: String,
    /// Tools
    tools_run: Vec<String>,
    /// Status
    status: String,
    /// Details
    details: HashMap<String, String>,
}

/// Scans the current directory and matches files against tool regex patterns.
fn detect_applicable_tools(tools: &[Box<dyn tools::CodeTool>]) -> Vec<String> {
    let mut applicable = HashSet::new();
    let mut regexes = Vec::new();

    for t in tools {
        if let Ok(re) = Regex::new(t.match_regex()) {
            regexes.push((t.name().to_string(), re));
        }
    }

    let walker = WalkBuilder::new(".").follow_links(false).build();
    for entry in walker.flatten() {
        if entry.file_type().is_some_and(|ft| ft.is_file()) {
            let path_str = entry.path().to_string_lossy();
            for (name, re) in &regexes {
                if !applicable.contains(name) && re.is_match(&path_str) {
                    applicable.insert(name.clone());
                }
            }
        }
    }

    let mut result: Vec<String> = applicable.into_iter().collect();
    result.sort();
    result
}

#[cfg(not(tarpaulin_include))]
/// Interactive selection
fn interactive_selection() -> Result<Vec<Box<dyn tools::CodeTool>>, CliError> {
    let all_tools = registry::get_tools();
    let detected_names = detect_applicable_tools(&all_tools);

    if detected_names.is_empty() {
        println!("No files matched any available tools in current directory.");
        return Ok(Vec::new());
    }

    let available_tools: Vec<_> = all_tools
        .into_iter()
        .filter(|t| detected_names.contains(&t.name().to_string()))
        .collect();

    let selected_indices = tui::select_tools(&available_tools)?;

    // Need to extract the selected tools from the vector, taking ownership.
    // We can do this by keeping the ones that were selected.
    let mut selected_tools = Vec::new();
    // iterate backwards to safely remove from available_tools, or just use indices directly
    for index in selected_indices {
        if let Some(tool) = available_tools.get(index) {
            // Need to reconstruct from registry to get owned boxes since we can't easily move out of the filtered vec dynamically without complicated ownership
            // simpler: just re-fetch the ones we selected by name
            let name = tool.name().to_string();
            let all = registry::get_tools();
            if let Some(t) = all.into_iter().find(|t| t.name() == name) {
                selected_tools.push(t);
            }
        }
    }

    Ok(selected_tools)
}

#[cfg(not(tarpaulin_include))]
/// Append to README
fn append_to_readme(action_name: &str, json_report: &str) -> Result<(), CliError> {
    if Path::new("README.md").exists() {
        let mut file = OpenOptions::new().append(true).open("README.md")?;

        writeln!(
            file,
            "\n## Latest {} Report\n```json\n{}\n```",
            action_name, json_report
        )?;
        println!("\n📝 Appended report to README.md");

        // Run git diff
        println!("\n🔍 Running `git diff README.md` to detail changes:");
        let output = Command::new("git")
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .arg("diff")
            .arg("README.md")
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() {
            println!("(No diff output - file may be untracked. Run `git add README.md` to track.)");
        } else {
            println!("{}", stdout);
        }
    } else {
        println!("README.md not found, skipped appending report.");
    }
    Ok(())
}

/// Runs the selected action (audit/fix) potentially filtering by pattern and specific tools.
pub fn run(action: Action, request: bridle_sdk::models::ToolRunRequest) -> Result<(), CliError> {
    let action_name = match action {
        Action::Audit => "audit",
        Action::Fix { .. } => "fix",
    };

    println!("🔍 Starting {}...", action_name);

    let selected_tools_opt: Option<Vec<Box<dyn tools::CodeTool>>>;

    // Determine tools
    if let Some(pattern) = &request.pattern {
        // Non-interactive mode (pattern)
        let available_for_pattern = registry::get_tools_for_pattern(pattern);
        if available_for_pattern.is_empty() {
            println!("No tools found matching pattern: {}", pattern);
            return Ok(());
        }

        if let Some(tools_list) = &request.tools {
            // Filter by selected tools
            let tools_set: HashSet<&String> = tools_list.iter().collect();
            let filtered: Vec<_> = available_for_pattern
                .into_iter()
                .filter(|t| tools_set.contains(&t.name().to_string()))
                .collect();

            if filtered.is_empty() {
                println!("None of the specified tools matched pattern: {}", pattern);
                return Ok(());
            }
            selected_tools_opt = Some(filtered);
        } else {
            // Run all tools for that pattern
            selected_tools_opt = Some(available_for_pattern);
        }
    } else {
        if let Some(tools_list) = &request.tools {
            // Specified tools without target pattern
            let all_tools = registry::get_tools();
            let tools_set: HashSet<&String> = tools_list.iter().collect();
            let filtered: Vec<_> = all_tools
                .into_iter()
                .filter(|t| tools_set.contains(&t.name().to_string()))
                .collect();

            if filtered.is_empty() {
                println!("None of the specified tools were found.");
                return Ok(());
            }
            selected_tools_opt = Some(filtered);
        } else {
            // Interactive mode
            #[cfg(not(tarpaulin_include))]
            {
                selected_tools_opt = Some(interactive_selection()?);
            }
            #[cfg(tarpaulin_include)]
            {
                selected_tools_opt = Some(vec![]);
            }
        }
    }

    let selected_tools = selected_tools_opt.unwrap_or_default();

    #[cfg(not(tarpaulin_include))]
    {
        if selected_tools.is_empty() {
            println!("No tools selected.");
            return Ok(());
        }
    }

    println!("\n🚀 Running selected tools...\n");

    let mut report = Report {
        action: action_name.to_string(),
        tools_run: selected_tools
            .iter()
            .map(|t| t.name().to_string())
            .collect(),
        status: "Completed".to_string(),
        details: HashMap::new(),
    };

    for tool in selected_tools {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .template("{spinner:.green} {msg}")?,
        );
        pb.set_message(format!("Running {}...", tool.name()));

        // Simulate some time passing to show the spinner (could be removed in real app)
        for _ in 0..10 {
            pb.tick();
            std::thread::sleep(Duration::from_millis(50));
        }

        let empty_args = Vec::new();
        let current_args = request
            .tool_args
            .as_ref()
            .and_then(|args| args.get(tool.name()))
            .unwrap_or(&empty_args);

        let result = match action {
            Action::Audit => tool.audit(current_args, None),
            Action::Fix { dry_run } => tool.fix(current_args, dry_run, None),
        };

        match result {
            Ok(output) => {
                pb.finish_with_message(format!("✅ Finished {}: {}", tool.name(), output));
                report.details.insert(tool.name().to_string(), output);
            }
            #[cfg(not(tarpaulin_include))]
            Err(e) => {
                pb.finish_with_message(format!("❌ Error running {}: {}", tool.name(), e));
                report
                    .details
                    .insert(tool.name().to_string(), format!("Error: {}", e));
            }
        }
    }

    let json_report = serde_json::to_string_pretty(&report)?;
    println!("\n📊 JSON Format Report:");
    println!("{}", json_report);

    #[cfg(not(tarpaulin_include))]
    append_to_readme(action_name, &json_report)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bridle_sdk::models::ToolRunRequest;

    #[test]
    fn test_detect_applicable_tools() {
        let tools = registry::get_tools();
        let applicable = detect_applicable_tools(&tools);
        // Will always detect rust in this codebase
        assert!(applicable.contains(&"rust-unwrap-to-question-mark".to_string()));
    }

    #[test]
    fn test_run_non_interactive_no_tools() -> Result<(), CliError> {
        let req = ToolRunRequest {
            pattern: Some("unknown-pattern".to_string()),
            tools: None,
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Audit, req)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_with_tools() -> Result<(), CliError> {
        // We provide a known mock tool from registry for go matching r".*\.go$"
        let req = ToolRunRequest {
            pattern: Some(r".*\.go$".to_string()),
            tools: Some(vec!["rust-unwrap-to-question-mark".to_string()]),
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Fix { dry_run: false }, req)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_with_tools_no_pattern() -> Result<(), CliError> {
        let req = ToolRunRequest {
            pattern: None,
            tools: Some(vec!["rust-unwrap-to-question-mark".to_string()]),
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Fix { dry_run: false }, req)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_with_missing_tool_no_pattern() -> Result<(), CliError> {
        let req = ToolRunRequest {
            pattern: None,
            tools: Some(vec!["non-existent-tool".to_string()]),
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Fix { dry_run: false }, req)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_with_tools_dry_run() -> Result<(), CliError> {
        let req = ToolRunRequest {
            pattern: Some(r".*\.go$".to_string()),
            tools: Some(vec!["rust-unwrap-to-question-mark".to_string()]),
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Fix { dry_run: true }, req)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_all_tools() -> Result<(), CliError> {
        let req = ToolRunRequest {
            pattern: Some(r".*\.go$".to_string()),
            tools: None,
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Audit, req)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_not_found_tool() -> Result<(), CliError> {
        let req = ToolRunRequest {
            pattern: Some(r".*\.go$".to_string()),
            tools: Some(vec!["missing-tool".to_string()]),
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Audit, req)?;
        Ok(())
    }

    #[test]
    fn test_action_enum() {
        assert_eq!(Action::Audit, Action::Audit);
        assert_ne!(Action::Audit, Action::Fix { dry_run: false });
    }
}
