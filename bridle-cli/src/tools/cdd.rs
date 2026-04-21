//! The `cdd-c` tools for various refactoring and generation tasks.

use super::CodeTool;
use crate::error::CliError;
use bridle_sdk::path_scope::PathScope;
use derive_more::derive::{Display, Error, From};

/// Error type for Cdd tools.
#[derive(Debug, Display, Error, From)]
pub enum CddError {
    /// Subprocess execution error
    #[display("Execution Error: {}", _0)]
    #[error(ignore)]
    Execution(String),
    /// Missing target argument
    #[display("Missing target file argument")]
    MissingArgument,
    /// Path scope violation
    #[display("Path scope violation")]
    ScopeViolation,
}

impl From<CddError> for CliError {
    fn from(err: CddError) -> Self {
        CliError::Tool(err.to_string())
    }
}

/// Generates a cdd-c tool implementation
macro_rules! cdd_tool {
    ($tool_name:ident, $name:expr, $desc:expr, $subcommand:expr, $regex:expr) => {
        /// $tool_name
        #[derive(Debug, Clone)]
        pub struct $tool_name;

        impl CodeTool for $tool_name {
            fn name(&self) -> &'static str {
                $name
            }
            fn description(&self) -> &'static str {
                $desc
            }
            fn match_regex(&self) -> &'static str {
                $regex
            }
            fn audit(
                &self,
                args: &[String],
                scope: Option<&PathScope>,
            ) -> Result<String, CliError> {
                if args.is_empty() {
                    return Err(CddError::MissingArgument.into());
                }
                let target = &args[0];
                if let Some(s) = scope {
                    if !s.is_allowed(target) {
                        return Err(CddError::ScopeViolation.into());
                    }
                }

                let result =
                    bridle_sdk::ffi::cdd_transformer_safe($subcommand, target, true, false, scope)
                        .map_err(|e| CddError::Execution(e.to_string()))?;

                if result != 0 {
                    return Err(CddError::Execution(format!("{} failed", $name)).into());
                }
                Ok(format!("{} audit executed", $name))
            }
            fn fix(
                &self,
                args: &[String],
                dry_run: bool,
                scope: Option<&PathScope>,
            ) -> Result<String, CliError> {
                if args.is_empty() {
                    return Err(CddError::MissingArgument.into());
                }
                let target = &args[0];
                if let Some(s) = scope {
                    if !s.is_allowed(target) {
                        return Err(CddError::ScopeViolation.into());
                    }
                }

                let result = bridle_sdk::ffi::cdd_transformer_safe(
                    $subcommand,
                    target,
                    false,
                    dry_run,
                    scope,
                )
                .map_err(|e| CddError::Execution(e.to_string()))?;
                if result != 0 {
                    return Err(CddError::Execution(format!("{} failed", $name)).into());
                }
                if dry_run {
                    Ok(format!("[DRY RUN] {} fix planned", $name))
                } else {
                    Ok(format!("{} fix applied", $name))
                }
            }
        }
    };
}

cdd_tool!(
    CddExternCTool,
    "cdd-extern-c",
    "Adds extern \"C\" wrapping",
    "extern_c",
    r".*\.(c|cpp|h|hpp)$"
);
cdd_tool!(
    CddMsvcPortTool,
    "cdd-msvc-port",
    "Ports POSIX to MSVC",
    "msvc_port",
    r".*\.(c|cpp|h|hpp)$"
);
cdd_tool!(
    CddGnuStandardizerTool,
    "cdd-gnu-standardizer",
    "Standardizes GNU extensions",
    "gnu_standardizer",
    r".*\.(c|cpp|h|hpp)$"
);
cdd_tool!(
    CddErrorPercolatorTool,
    "cdd-error-percolator",
    "Percolates errors",
    "error_percolator",
    r".*\.(c|cpp|h|hpp)$"
);
cdd_tool!(
    CddSafeCrtTool,
    "cdd-safe-crt",
    "Migrates to Safe CRT",
    "safe_crt",
    r".*\.(c|cpp|h|hpp)$"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdd_error_display() {
        let err = CddError::MissingArgument;
        assert_eq!(format!("{}", err), "Missing target file argument");

        let err = CddError::ScopeViolation;
        assert_eq!(format!("{}", err), "Path scope violation");

        let err = CddError::Execution("failed".into());
        assert_eq!(format!("{}", err), "Execution Error: failed");
    }

    #[test]
    fn test_cdd_error_into_cli_error() {
        let err = CddError::MissingArgument;
        let cli_err: CliError = err.into();
        assert_eq!(
            format!("{}", cli_err),
            "Tool Execution Error: Missing target file argument"
        );
    }

    macro_rules! test_cdd_tool {
        ($test_name:ident, $tool:ident, $name:expr) => {
            #[test]
            fn $test_name() -> Result<(), CliError> {
                let tool = $tool;
                assert_eq!(tool.name(), $name);
                assert!(!tool.description().is_empty());
                assert_eq!(tool.match_regex(), r".*\.(c|cpp|h|hpp)$");

                unsafe { std::env::set_var("RUST_TEST_MODE", "1") };

                // Missing args
                assert!(tool.audit(&[], None).is_err());
                assert!(tool.fix(&[], false, None).is_err());

                // Success cases
                let args = vec!["dummy.c".to_string()];
                assert_eq!(
                    tool.audit(&args, None)?,
                    format!("{} audit executed", $name)
                );
                assert_eq!(
                    tool.fix(&args, false, None)?,
                    format!("{} fix applied", $name)
                );
                assert_eq!(
                    tool.fix(&args, true, None)?,
                    format!("[DRY RUN] {} fix planned", $name)
                );

                // Scope violation
                let allowed = vec!["allowed.c".to_string()];
                let ignored: Vec<String> = vec![];
                let scope = PathScope::new(&allowed, &ignored)
                    .map_err(|e| CliError::Execution(e.to_string()))?;
                let scope_args = vec!["disallowed.c".to_string()];
                assert!(tool.audit(&scope_args, Some(&scope)).is_err());
                assert!(tool.fix(&scope_args, false, Some(&scope)).is_err());

                // FFI return 1
                let fail_args = vec!["fail.c".to_string()];
                assert!(tool.audit(&fail_args, None).is_err());
                assert!(tool.fix(&fail_args, false, None).is_err());

                // FFI error
                let error_args = vec!["error.c".to_string()];
                assert!(tool.audit(&error_args, None).is_err());
                assert!(tool.fix(&error_args, false, None).is_err());

                unsafe { std::env::remove_var("RUST_TEST_MODE") };
                Ok(())
            }
        };
    }

    test_cdd_tool!(test_cdd_extern_c_tool, CddExternCTool, "cdd-extern-c");
    test_cdd_tool!(test_cdd_msvc_port_tool, CddMsvcPortTool, "cdd-msvc-port");
    test_cdd_tool!(
        test_cdd_gnu_standardizer_tool,
        CddGnuStandardizerTool,
        "cdd-gnu-standardizer"
    );
    test_cdd_tool!(
        test_cdd_error_percolator_tool,
        CddErrorPercolatorTool,
        "cdd-error-percolator"
    );
    test_cdd_tool!(test_cdd_safe_crt_tool, CddSafeCrtTool, "cdd-safe-crt");
}
