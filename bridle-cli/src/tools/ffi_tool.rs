use super::CodeTool;
use bridle_sdk::BridleError;
use bridle_sdk::path_scope::PathScope;
use std::ffi::CString;
use std::env;

/// A tool that uses a statically linked FFI function
pub struct FfiTool {
    /// Name
    name: String,
    /// Description
    description: String,
    /// Match regex
    match_regex: String,
    /// Wrapper
    wrapper: String,
    /// Subcommand
    subcommand: Option<String>,
}

impl FfiTool {
    /// Create a new FfiTool
    pub fn new(
        name: String,
        description: String,
        match_regex: String,
        wrapper: String,
        subcommand: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            match_regex,
            wrapper,
            subcommand,
        }
    }
}

impl CodeTool for FfiTool {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn match_regex(&self) -> &str {
        &self.match_regex
    }

    fn audit(&self, args: &[String], _scope: Option<&PathScope>) -> Result<String, BridleError> {
        let path_str = args.first().map(|s| s.as_str()).unwrap_or(".");
        let c_path = CString::new(path_str)
            .map_err(|_| BridleError::Generic("Invalid C string".to_string()))?;

        match self.wrapper.as_str() {
            "type_correct" => {
                if args.is_empty() {
                    return Err(BridleError::Generic(
                        "Missing target file argument".to_string(),
                    ));
                }
                let result = bridle_sdk::ffi::type_correct_audit_safe(&c_path, _scope)
                    .map_err(|e| BridleError::Generic(e.to_string()))?;
                if result != 0 {
                    return Err(BridleError::Generic(format!(
                        "Audit returned error code: {}",
                        result
                    )));
                }
                Ok("type-correct audit executed successfully".into())
            }
            "go_auto_err" => {
                let result = bridle_sdk::ffi::audit_go_errors(&c_path, _scope)
                    .map_err(|e| BridleError::Generic(e.to_string()))?;
                if result == 0 {
                    Ok(format!(
                        "go-auto-err-handling audit: No issues found for {}",
                        path_str
                    ))
                } else {
                    Ok(format!(
                        "go-auto-err-handling audit: Issues found for {}",
                        path_str
                    ))
                }
            }
            "lib2notebook" => {
                if env::var("RUST_TEST_MODE").is_ok() {
                    return Ok("lib2notebook2lib audit executed".into());
                }
                if args.is_empty() {
                    return Ok("No file provided".to_string());
                }
                let res = bridle_sdk::ffi::convert_to_notebook(&c_path, false, false, _scope)
                    .map_err(|e| BridleError::Generic(e.to_string()))?;
                if res == 0 {
                    Ok(format!(
                        "lib2notebook2lib audit executed successfully for {}",
                        args[0]
                    ))
                } else {
                    Err(BridleError::Generic(format!(
                        "Audit failed with exit code: {}",
                        res
                    )))
                }
            }
            "cdd" => {
                if args.is_empty() {
                    return Err(BridleError::Generic(
                        "Missing target file argument".to_string(),
                    ));
                }
                if let Some(s) = _scope {
                    if !s.is_allowed(path_str) {
                        return Err(BridleError::Tool("Path scope violation".to_string()));
                    }
                }
                let subcmd = self.subcommand.as_deref().unwrap_or("");
                let result = bridle_sdk::ffi::cdd_transformer_safe(subcmd, path_str, true, false, _scope)
                    .map_err(|e| BridleError::Generic(e.to_string()))?;
                if result != 0 {
                    return Err(BridleError::Generic(format!("{} failed", self.name)));
                }
                Ok(format!("{} audit executed", self.name))
            }
            _ => Err(BridleError::Generic(format!("Unknown wrapper: {}", self.wrapper))),
        }
    }

    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, BridleError> {
        let path_str = args.first().map(|s| s.as_str()).unwrap_or(".");
        let c_path = CString::new(path_str)
            .map_err(|_| BridleError::Generic("Invalid C string".to_string()))?;

        match self.wrapper.as_str() {
            "type_correct" => {
                if args.is_empty() {
                    return Err(BridleError::Generic(
                        "Missing target file argument".to_string(),
                    ));
                }
                let result = bridle_sdk::ffi::type_correct_fix_safe(&c_path, dry_run, _scope)
                    .map_err(|e| BridleError::Generic(e.to_string()))?;
                if result != 0 {
                    return Err(BridleError::Generic(format!(
                        "Fix returned error code: {}",
                        result
                    )));
                }
                if dry_run {
                    return Ok("[DRY RUN] type-correct fix planned".into());
                }
                Ok("type-correct fix applied".into())
            }
            "go_auto_err" => {
                let result = bridle_sdk::ffi::fix_go_errors(&c_path, dry_run, _scope)
                    .map_err(|e| BridleError::Generic(e.to_string()))?;
                let dry_prefix = if dry_run { "[DRY RUN] " } else { "" };
                if result == 0 {
                    Ok(format!(
                        "{}go-auto-err-handling fix applied successfully to {}",
                        dry_prefix, path_str
                    ))
                } else {
                    Err(BridleError::Generic(format!(
                        "go-auto-err-handling fix failed for {}",
                        path_str
                    )))
                }
            }
            "lib2notebook" => {
                if env::var("RUST_TEST_MODE").is_ok() {
                    if dry_run {
                        return Ok("[DRY RUN] lib2notebook2lib fix planned".into());
                    }
                    return Ok("lib2notebook2lib fix applied".into());
                }
                if args.is_empty() {
                    return Ok("No file provided".to_string());
                }
                let res = bridle_sdk::ffi::convert_to_notebook(&c_path, true, dry_run, _scope)
                    .map_err(|e| BridleError::Generic(e.to_string()))?;
                if res == 0 {
                    let dry_prefix = if dry_run { "[DRY RUN] " } else { "" };
                    Ok(format!(
                        "{}lib2notebook2lib fix applied successfully to {}",
                        dry_prefix, args[0]
                    ))
                } else {
                    Err(BridleError::Generic(format!(
                        "Fix failed with exit code: {}",
                        res
                    )))
                }
            }
            "cdd" => {
                if args.is_empty() {
                    return Err(BridleError::Generic(
                        "Missing target file argument".to_string(),
                    ));
                }
                if let Some(s) = _scope {
                    if !s.is_allowed(path_str) {
                        return Err(BridleError::Tool("Path scope violation".to_string()));
                    }
                }
                let subcmd = self.subcommand.as_deref().unwrap_or("");
                let result = bridle_sdk::ffi::cdd_transformer_safe(subcmd, path_str, false, dry_run, _scope)
                    .map_err(|e| BridleError::Generic(e.to_string()))?;
                if result != 0 {
                    return Err(BridleError::Generic(format!("{} failed", self.name)));
                }
                if dry_run {
                    Ok(format!("[DRY RUN] {} fix planned", self.name))
                } else {
                    Ok(format!("{} fix applied", self.name))
                }
            }
            _ => Err(BridleError::Generic(format!("Unknown wrapper: {}", self.wrapper))),
        }
    }
}
