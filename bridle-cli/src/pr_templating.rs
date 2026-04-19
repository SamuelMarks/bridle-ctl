use crate::error::CliError;
use std::path::Path;
use tera::{Context, Tera};

/// PR Template engine.
pub struct PrTemplateEngine {
    /// The tera template engine.
    tera: Tera,
}

impl PrTemplateEngine {
    /// Creates a new template engine.
    pub fn new() -> Result<Self, CliError> {
        let tera = Tera::default();
        Ok(Self { tera })
    }

    /// Resolves the PR template from the repository or uses fallback.
    pub fn resolve_template(dir: &Path, fallback: &str) -> String {
        let candidates = [
            ".github/PULL_REQUEST_TEMPLATE.md",
            ".github/PULL_REQUEST_TEMPLATE",
            "PULL_REQUEST_TEMPLATE.md",
            ".gitlab/merge_request_templates/default.md",
        ];

        for candidate in candidates {
            let path = dir.join(candidate);
            if path.exists()
                && let Ok(content) = std::fs::read_to_string(path) {
                    return content;
                }
        }

        fallback.to_string()
    }

    /// Renders the PR body.
    pub fn render_pr_body(
        &mut self,
        template_content: &str,
        repo_name: &str,
        repo_owner: &str,
        branch_name: &str,
        diff_stats: &str,
    ) -> Result<String, CliError> {
        self.tera
            .add_raw_template("pr_body", template_content)
            .map_err(|e| CliError::Execution(e.to_string()))?;

        let mut context = Context::new();
        context.insert(
            "repo",
            &serde_json::json!({
                "name": repo_name,
                "owner": repo_owner,
            }),
        );
        context.insert("branch_name", branch_name);
        context.insert("diff_stats", diff_stats);

        // Expose steps output could be added here

        self.tera
            .render("pr_body", &context)
            .map_err(|e| CliError::Execution(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_resolve_template_fallback() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let fallback = "Fallback template";
        let resolved = PrTemplateEngine::resolve_template(dir.path(), fallback);
        assert_eq!(resolved, fallback);
        Ok(())
    }

    #[test]
    fn test_resolve_template_exists() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let file_path = dir.path().join("PULL_REQUEST_TEMPLATE.md");
        std::fs::write(&file_path, "Real template")?;

        let resolved = PrTemplateEngine::resolve_template(dir.path(), "Fallback template");
        assert_eq!(resolved, "Real template");
        Ok(())
    }

    #[test]
    fn test_render_pr_body_success() -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = PrTemplateEngine::new()?;
        let template = "Repo: {{ repo.name }}, Owner: {{ repo.owner }}, Branch: {{ branch_name }}, Stats: {{ diff_stats }}";
        let body = engine.render_pr_body(template, "my_repo", "my_owner", "my_branch", "+5 -2")?;

        assert_eq!(
            body,
            "Repo: my_repo, Owner: my_owner, Branch: my_branch, Stats: +5 -2"
        );
        Ok(())
    }

    #[test]
    fn test_render_pr_body_invalid_template() -> Result<(), Box<dyn std::error::Error>> {
        let mut engine = PrTemplateEngine::new()?;
        let template = "Repo: {{ repo.name "; // Missing closing braces
        let err = engine.render_pr_body(template, "my_repo", "my_owner", "my_branch", "+5 -2");

        assert!(err.is_err());
        Ok(())
    }
}
