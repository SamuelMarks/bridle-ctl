//! Logic for running codebase tools (audit and fix modes).

use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::tools::{self, registry};
use crate::tui;
use crate::workspace::git_command;
use bridle_sdk::BridleError;

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

/// Interactive selection
fn interactive_selection() -> Result<Vec<Box<dyn tools::CodeTool>>, BridleError> {
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

    let mut selected_tools = Vec::new();
    for index in selected_indices {
        if let Some(tool) = available_tools.get(index) {
            let name = tool.name().to_string();
            let all = registry::get_tools();
            if let Some(t) = all.into_iter().find(|t| t.name() == name) {
                selected_tools.push(t);
            }
        }
    }

    Ok(selected_tools)
}

/// Append to README
fn append_to_readme(action_name: &str, json_report: &str) -> Result<(), BridleError> {
    if Path::new("README.md").exists() {
        let mut file = OpenOptions::new().append(true).open("README.md")?;

        writeln!(
            file,
            "
## Latest {} Report
```json
{}
```",
            action_name, json_report
        )?;
        println!(
            "
📝 Appended report to README.md"
        );

        println!(
            "
🔍 Running `git diff README.md` to detail changes:"
        );
        let output = git_command().arg("diff").arg("README.md").output()?;

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
pub fn run(action: Action, request: bridle_sdk::models::ToolRunRequest) -> Result<(), BridleError> {
    let action_name = match action {
        Action::Audit => "audit",
        Action::Fix { .. } => "fix",
    };

    println!("🔍 Starting {}...", action_name);

    let selected_tools = if let Some(pattern) = &request.pattern {
        let available_for_pattern = registry::get_tools_for_pattern(pattern);
        if available_for_pattern.is_empty() {
            println!("No tools found matching pattern: {}", pattern);
            return Ok(());
        }

        if let Some(tools_list) = &request.tools {
            let tools_set: HashSet<&String> = tools_list.iter().collect();
            let filtered: Vec<_> = available_for_pattern
                .into_iter()
                .filter(|t| tools_set.contains(&t.name().to_string()))
                .collect();

            if filtered.is_empty() {
                println!("None of the specified tools matched pattern: {}", pattern);
                return Ok(());
            }
            filtered
        } else {
            available_for_pattern
        }
    } else if let Some(tools_list) = &request.tools {
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
        filtered
    } else {
        interactive_selection()?
    };

    if selected_tools.is_empty() {
        println!("No tools selected.");
        return Ok(());
    }

    println!(
        "
🚀 Running selected tools...
"
    );

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

        for _ in 0..2 {
            pb.tick();
            std::thread::sleep(Duration::from_millis(1));
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
            Err(e) => {
                pb.finish_with_message(format!("❌ Error running {}: {}", tool.name(), e));
                report
                    .details
                    .insert(tool.name().to_string(), format!("Error: {}", e));
            }
        }
    }

    let json_report = serde_json::to_string_pretty(&report)?;
    println!(
        "
📊 JSON Format Report:"
    );
    println!("{}", json_report);

    append_to_readme(action_name, &json_report)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bridle_sdk::models::ToolRunRequest;

    #[test]
    #[serial_test::serial]
    fn test_detect_applicable_tools_and_append_readme() -> Result<(), BridleError> {
        let tools = registry::get_tools();
        let tmp_dir = tempfile::tempdir()?;
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir(tmp_dir.path())?;

        // Empty directory: interactive_selection returns empty vec via run()
        let empty_req = ToolRunRequest {
            pattern: None,
            tools: None,
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Audit, empty_req)?;

        let _ = git_command().arg("init").status();
        let _ = git_command()
            .args(["config", "user.name", "Test User"])
            .status();
        let _ = git_command()
            .args(["config", "user.email", "test@example.com"])
            .status();
        std::fs::write("dummy_test_detect.rs", "fn main() {}")?;
        std::fs::write("README.md", "# Test Readme\n")?;
        let applicable = detect_applicable_tools(&tools);
        assert!(applicable.contains(&"rust-unwrap-to-question-mark".to_string()));

        unsafe {
            std::env::set_var("BRIDLE_TEST_MOCK_TUI", "1");
        }
        let req = ToolRunRequest {
            pattern: None,
            tools: None,
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Audit, req.clone())?;
        let _ = git_command().args(["add", "README.md"]).status();
        let _ = git_command()
            .args(["commit", "-m", "track readme"])
            .status();
        run(Action::Audit, req)?;
        unsafe {
            std::env::remove_var("BRIDLE_TEST_MOCK_TUI");
        }

        let readme = std::fs::read_to_string("README.md")?;
        assert!(readme.contains("Latest audit Report"));

        std::env::set_current_dir(original_dir)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_no_tools() -> Result<(), BridleError> {
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
    fn test_run_non_interactive_with_tools() -> Result<(), BridleError> {
        let req = ToolRunRequest {
            pattern: Some(r".*\.rs$".to_string()),
            tools: Some(vec!["rust-unwrap-to-question-mark".to_string()]),
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Fix { dry_run: false }, req)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_with_tools_no_pattern() -> Result<(), BridleError> {
        let req = ToolRunRequest {
            pattern: None,
            tools: Some(vec![
                "rust-unwrap-to-question-mark".to_string(),
                "type-correct".to_string(),
            ]),
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Fix { dry_run: false }, req)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_with_missing_tool_no_pattern() -> Result<(), BridleError> {
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
    fn test_run_non_interactive_with_tools_dry_run() -> Result<(), BridleError> {
        let req = ToolRunRequest {
            pattern: Some(r".*\.rs$".to_string()),
            tools: Some(vec!["rust-unwrap-to-question-mark".to_string()]),
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Fix { dry_run: true }, req)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_all_tools() -> Result<(), BridleError> {
        let req = ToolRunRequest {
            pattern: Some(r".*\.rs$".to_string()),
            tools: None,
            tool_args: None,
            dry_run: None,
            action: None,
        };
        run(Action::Audit, req)?;
        Ok(())
    }

    #[test]
    fn test_run_non_interactive_not_found_tool() -> Result<(), BridleError> {
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
