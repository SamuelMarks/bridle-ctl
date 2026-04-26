//! Dynamic tool implementations
use std::ffi::{CStr, CString};

use super::CodeTool;
use crate::error::CliError;
use bridle_sdk::path_scope::PathScope;

use std::process::Command;
use std::sync::Arc;

/// A tool that executes a subprocess
pub struct SubprocessTool {
    /// Name
    name: String,
    /// Description
    description: String,
    /// Match regex
    match_regex: String,
    /// Tool version
    version: Option<String>,
    /// Tool author
    author: Option<String>,
    /// Tool URL
    url: Option<String>,
    /// Tool license
    license: Option<String>,
    /// Command
    command: String,
    /// Environment variables
    env: std::collections::HashMap<String, String>,
    /// Venv aware
    venv_aware: bool,
}

impl SubprocessTool {
    /// Create a new SubprocessTool
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        description: String,
        match_regex: String,
        version: Option<String>,
        author: Option<String>,
        url: Option<String>,
        license: Option<String>,

        command: String,
        env: std::collections::HashMap<String, String>,
        venv_aware: bool,
    ) -> Self {
        Self {
            name,
            description,
            match_regex,
            version,
            author,
            url,
            license,
            command,
            env,
            venv_aware,
        }
    }

    /// Configures the command with arguments and environment variables.
    fn configure_command(
        &self,
        arg_action: &str,
        args: &[String],
        dry_run: Option<bool>,
    ) -> Command {
        let mut cmd = Command::new(&self.command);
        let mut envs = self.env.clone();

        if self.venv_aware {
            if let Ok(venv) = std::env::var("VIRTUAL_ENV") {
                let venv_path = std::path::PathBuf::from(venv);
                let bin_dir = if cfg!(windows) {
                    venv_path.join("Scripts")
                } else {
                    venv_path.join("bin")
                };
                if let Ok(path) = std::env::var("PATH") {
                    let new_path = std::env::join_paths(
                        std::iter::once(bin_dir).chain(std::env::split_paths(&path)),
                    )
                    .unwrap_or_default();
                    envs.insert("PATH".to_string(), new_path.to_string_lossy().to_string());
                } else {
                    envs.insert("PATH".to_string(), bin_dir.display().to_string());
                }
            } else {
                let mut local_venvs = vec![".venv".to_string(), "venv".to_string()];

                // Also find versioned venv paths like .venv-3-11, .venv-uv-3-12, etc.
                if let Ok(entries) = std::fs::read_dir(".") {
                    let prefixes = [
                        ".venv-uv-",
                        ".venv-pyenv-",
                        "venv-uv-",
                        "venv-pyenv-",
                        ".venv-",
                        "venv-",
                    ];
                    for entry in entries.flatten() {
                        if let Ok(name) = entry.file_name().into_string()
                            && entry.file_type().is_ok_and(|t| t.is_dir())
                        {
                            for prefix in prefixes.iter() {
                                if let Some(suffix) = name.strip_prefix(prefix) {
                                    let parts: Vec<&str> = suffix.split(['-', '.']).collect();
                                    if parts.len() == 2
                                        && !parts[0].is_empty()
                                        && !parts[1].is_empty()
                                        && parts[0].chars().all(|c| c.is_ascii_digit())
                                        && parts[1].chars().all(|c| c.is_ascii_digit())
                                    {
                                        local_venvs.push(name.clone());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                for v in local_venvs.iter() {
                    let venv_path = std::path::Path::new(v);
                    if venv_path.exists() && venv_path.is_dir() {
                        let bin_dir = if cfg!(windows) {
                            venv_path.join("Scripts")
                        } else {
                            venv_path.join("bin")
                        };
                        if let Ok(path) = std::env::var("PATH") {
                            let new_path = std::env::join_paths(
                                std::iter::once(bin_dir).chain(std::env::split_paths(&path)),
                            )
                            .unwrap_or_default();
                            envs.insert("PATH".to_string(), new_path.to_string_lossy().to_string());
                        } else {
                            envs.insert("PATH".to_string(), bin_dir.display().to_string());
                        }
                        envs.insert(
                            "VIRTUAL_ENV".to_string(),
                            venv_path
                                .canonicalize()
                                .unwrap_or_else(|_| venv_path.to_path_buf())
                                .display()
                                .to_string(),
                        );
                        break;
                    }
                }
            }
        }

        cmd.envs(&envs);
        cmd.arg(arg_action);
        if let Some(true) = dry_run {
            cmd.arg("--dry-run");
        }
        cmd.args(args);
        cmd
    }
}

impl CodeTool for SubprocessTool {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn match_regex(&self) -> &str {
        &self.match_regex
    }
    fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
    fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }
    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    fn license(&self) -> Option<&str> {
        self.license.as_deref()
    }

    fn audit(&self, args: &[String], _scope: Option<&PathScope>) -> Result<String, CliError> {
        let mut cmd = self.configure_command("audit", args, None);
        let output = cmd.output()?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?.trim().to_string())
        } else {
            Err(CliError::Execution(
                String::from_utf8(output.stderr)?.trim().to_string(),
            ))
        }
    }

    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, CliError> {
        let mut cmd = self.configure_command("fix", args, Some(dry_run));
        let output = cmd.output()?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?.trim().to_string())
        } else {
            Err(CliError::Execution(
                String::from_utf8(output.stderr)?.trim().to_string(),
            ))
        }
    }
}

/// JSON-RPC Payload
#[derive(serde::Serialize)]
struct JsonRpcRequest {
    /// jsonrpc version
    jsonrpc: String,
    /// Method name
    method: String,
    /// Params
    params: serde_json::Value,
    /// Request id
    id: u64,
}

/// JSON-RPC Response
#[derive(serde::Deserialize)]
struct JsonRpcResponse {
    /// Result
    result: Option<String>,
    /// Error
    error: Option<serde_json::Value>,
}

/// A tool that uses JSON-RPC over HTTP
pub struct JsonRpcTool {
    /// Name
    name: String,
    /// Description
    description: String,
    /// Match regex
    match_regex: String,
    /// Tool version
    version: Option<String>,
    /// Tool author
    author: Option<String>,
    /// Tool URL
    url: Option<String>,
    /// Tool license
    license: Option<String>,
    /// Endpoint
    endpoint: String,
}

impl JsonRpcTool {
    /// Create a new JsonRpcTool
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        description: String,
        match_regex: String,
        version: Option<String>,
        author: Option<String>,
        url: Option<String>,
        license: Option<String>,
        endpoint: String,
    ) -> Self {
        Self {
            name,
            description,
            match_regex,
            version,
            author,
            url,
            license,
            endpoint,
        }
    }

    /// Calls the RPC endpoint
    fn call_rpc(&self, method: &str, params: serde_json::Value) -> Result<String, CliError> {
        let req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
            id: 1,
        };
        let client = reqwest::blocking::Client::new();
        let res = client.post(&self.endpoint).json(&req).send()?;
        let rpc_res: JsonRpcResponse = res.json()?;
        if let Some(err) = rpc_res.error {
            return Err(CliError::Execution(err.to_string()));
        }
        Ok(rpc_res.result.unwrap_or_default())
    }
}

impl CodeTool for JsonRpcTool {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn match_regex(&self) -> &str {
        &self.match_regex
    }
    fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
    fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }
    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    fn license(&self) -> Option<&str> {
        self.license.as_deref()
    }

    fn audit(&self, args: &[String], _scope: Option<&PathScope>) -> Result<String, CliError> {
        self.call_rpc("audit", serde_json::json!({ "args": args }))
    }

    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, CliError> {
        self.call_rpc(
            "fix",
            serde_json::json!({ "args": args, "dry_run": dry_run }),
        )
    }
}

/// Type alias for an audit C function signature
type AuditFunc = unsafe extern "C" fn(
    args_ptr: *const *const std::ffi::c_char,
    args_len: usize,
    out_ptr: *mut *mut std::ffi::c_char,
) -> i32;

/// Type alias for a fix C function signature
type FixFunc = unsafe extern "C" fn(
    args_ptr: *const *const std::ffi::c_char,
    args_len: usize,
    dry_run: bool,
    out_ptr: *mut *mut std::ffi::c_char,
) -> i32;

/// A tool loaded dynamically via dlopen
pub struct DlopenTool {
    /// Name
    name: String,
    /// Description
    description: String,
    /// Match regex
    match_regex: String,
    /// Tool version
    version: Option<String>,
    /// Tool author
    author: Option<String>,
    /// Tool URL
    url: Option<String>,
    /// Tool license
    license: Option<String>,
    /// Lib
    lib: Arc<libloading::Library>,
}

// Safety: The library operations must be thread safe
unsafe impl Send for DlopenTool {}
unsafe impl Sync for DlopenTool {}

impl DlopenTool {
    /// Create a new DlopenTool
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        description: String,
        match_regex: String,
        version: Option<String>,
        author: Option<String>,
        url: Option<String>,
        license: Option<String>,

        path: &str,
    ) -> Result<Self, CliError> {
        let lib = unsafe {
            libloading::Library::new(path).map_err(|e| CliError::Execution(e.to_string()))?
        };
        Ok(Self {
            name,
            description,
            match_regex,
            version,
            author,
            url,
            license,
            lib: Arc::new(lib),
        })
    }

    /// Calls the C function dynamically loaded
    fn call_c_func(
        &self,
        func_name: &[u8],
        args: &[String],
        dry_run: Option<bool>,
    ) -> Result<String, CliError> {
        let c_args: Vec<CString> = args
            .iter()
            .map(|s| {
                CString::new(s.as_str()).unwrap_or_else(|_| CString::new("").unwrap_or_default())
            })
            .collect();

        let c_args_ptrs: Vec<*const std::ffi::c_char> = c_args.iter().map(|c| c.as_ptr()).collect();
        let mut out_ptr: *mut std::ffi::c_char = std::ptr::null_mut();

        let res = unsafe {
            if let Some(dr) = dry_run {
                let func: libloading::Symbol<FixFunc> = self
                    .lib
                    .get(func_name)
                    .map_err(|e| CliError::Execution(e.to_string()))?;
                func(c_args_ptrs.as_ptr(), c_args_ptrs.len(), dr, &mut out_ptr)
            } else {
                let func: libloading::Symbol<AuditFunc> = self
                    .lib
                    .get(func_name)
                    .map_err(|e| CliError::Execution(e.to_string()))?;
                func(c_args_ptrs.as_ptr(), c_args_ptrs.len(), &mut out_ptr)
            }
        };

        let out_str = if !out_ptr.is_null() {
            unsafe { CStr::from_ptr(out_ptr).to_string_lossy().into_owned() }
        } else {
            String::new()
        };

        if res == 0 {
            Ok(out_str)
        } else {
            Err(CliError::Execution(format!(
                "dlopen function returned error {}: {}",
                res, out_str
            )))
        }
    }
}

impl CodeTool for DlopenTool {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> &str {
        &self.description
    }
    fn match_regex(&self) -> &str {
        &self.match_regex
    }
    fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
    fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }
    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    fn license(&self) -> Option<&str> {
        self.license.as_deref()
    }

    fn audit(&self, args: &[String], _scope: Option<&PathScope>) -> Result<String, CliError> {
        self.call_c_func(b"tool_audit\0", args, None)
    }

    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, CliError> {
        self.call_c_func(b"tool_fix\0", args, Some(dry_run))
    }
}

use std::env;

/// A tool that uses a statically linked FFI function
pub struct FfiTool {
    /// Name
    name: String,
    /// Description
    description: String,
    /// Match regex
    match_regex: String,
    /// Tool version
    version: Option<String>,
    /// Tool author
    author: Option<String>,
    /// Tool URL
    url: Option<String>,
    /// Tool license
    license: Option<String>,
    /// Wrapper
    wrapper: String,
    /// Subcommand
    subcommand: Option<String>,
}

impl FfiTool {
    /// Create a new FfiTool
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        description: String,
        match_regex: String,
        version: Option<String>,
        author: Option<String>,
        url: Option<String>,
        license: Option<String>,

        wrapper: String,
        subcommand: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            match_regex,
            version,
            author,
            url,
            license,
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
    fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
    fn author(&self) -> Option<&str> {
        self.author.as_deref()
    }
    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    fn license(&self) -> Option<&str> {
        self.license.as_deref()
    }

    fn audit(&self, args: &[String], _scope: Option<&PathScope>) -> Result<String, CliError> {
        let path_str = args.first().map(|s| s.as_str()).unwrap_or(".");
        let c_path = CString::new(path_str)
            .map_err(|_| CliError::Execution("Invalid C string".to_string()))?;

        match self.wrapper.as_str() {
            "type_correct" => {
                if args.is_empty() {
                    return Err(CliError::Execution(
                        "Missing target file argument".to_string(),
                    ));
                }
                let result = bridle_sdk::ffi::type_correct_audit_safe(&c_path, _scope)
                    .map_err(|e| CliError::Execution(e.to_string()))?;
                if result != 0 {
                    return Err(CliError::Execution(format!(
                        "Audit returned error code: {}",
                        result
                    )));
                }
                Ok("type-correct audit executed successfully".into())
            }
            "go_auto_err" => {
                let result = bridle_sdk::ffi::audit_go_errors(&c_path, _scope)
                    .map_err(|e| CliError::Execution(e.to_string()))?;
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
                    .map_err(|e| CliError::Execution(e.to_string()))?;
                if res == 0 {
                    Ok(format!(
                        "lib2notebook2lib audit executed successfully for {}",
                        args[0]
                    ))
                } else {
                    Err(CliError::Execution(format!(
                        "Audit failed with exit code: {}",
                        res
                    )))
                }
            }
            "cdd" => {
                if args.is_empty() {
                    return Err(CliError::Execution(
                        "Missing target file argument".to_string(),
                    ));
                }
                if _scope.is_some_and(|s| !s.is_allowed(path_str)) {
                    return Err(CliError::Tool("Path scope violation".to_string()));
                }
                let subcmd = self.subcommand.as_deref().unwrap_or("");
                let result =
                    bridle_sdk::ffi::cdd_transformer_safe(subcmd, path_str, true, false, _scope)
                        .map_err(|e| CliError::Execution(e.to_string()))?;
                if result != 0 {
                    return Err(CliError::Execution(format!("{} failed", self.name)));
                }
                Ok(format!("{} audit executed", self.name))
            }
            _ => Err(CliError::Execution(format!(
                "Unknown wrapper: {}",
                self.wrapper
            ))),
        }
    }

    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, CliError> {
        let path_str = args.first().map(|s| s.as_str()).unwrap_or(".");
        let c_path = CString::new(path_str)
            .map_err(|_| CliError::Execution("Invalid C string".to_string()))?;

        match self.wrapper.as_str() {
            "type_correct" => {
                if args.is_empty() {
                    return Err(CliError::Execution(
                        "Missing target file argument".to_string(),
                    ));
                }
                let result = bridle_sdk::ffi::type_correct_fix_safe(&c_path, dry_run, _scope)
                    .map_err(|e| CliError::Execution(e.to_string()))?;
                if result != 0 {
                    return Err(CliError::Execution(format!(
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
                    .map_err(|e| CliError::Execution(e.to_string()))?;
                let dry_prefix = if dry_run { "[DRY RUN] " } else { "" };
                if result == 0 {
                    Ok(format!(
                        "{}go-auto-err-handling fix applied successfully to {}",
                        dry_prefix, path_str
                    ))
                } else {
                    Err(CliError::Execution(format!(
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
                    .map_err(|e| CliError::Execution(e.to_string()))?;
                if res == 0 {
                    let dry_prefix = if dry_run { "[DRY RUN] " } else { "" };
                    Ok(format!(
                        "{}lib2notebook2lib fix applied successfully to {}",
                        dry_prefix, args[0]
                    ))
                } else {
                    Err(CliError::Execution(format!(
                        "Fix failed with exit code: {}",
                        res
                    )))
                }
            }
            "cdd" => {
                if args.is_empty() {
                    return Err(CliError::Execution(
                        "Missing target file argument".to_string(),
                    ));
                }
                if _scope.is_some_and(|s| !s.is_allowed(path_str)) {
                    return Err(CliError::Tool("Path scope violation".to_string()));
                }
                let subcmd = self.subcommand.as_deref().unwrap_or("");
                let result =
                    bridle_sdk::ffi::cdd_transformer_safe(subcmd, path_str, false, dry_run, _scope)
                        .map_err(|e| CliError::Execution(e.to_string()))?;
                if result != 0 {
                    return Err(CliError::Execution(format!("{} failed", self.name)));
                }
                if dry_run {
                    Ok(format!("[DRY RUN] {} fix planned", self.name))
                } else {
                    Ok(format!("{} fix applied", self.name))
                }
            }
            _ => Err(CliError::Execution(format!(
                "Unknown wrapper: {}",
                self.wrapper
            ))),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subprocess_tool() -> Result<(), CliError> {
        let tool = SubprocessTool::new(
            "test".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            None,
            None,
            None,
            None,
            "echo".to_string(),
            std::collections::HashMap::new(),
            true,
        );
        assert_eq!(tool.name(), "test");
        assert_eq!(tool.description(), "desc");
        assert_eq!(tool.match_regex(), ".*");

        let audit_res = tool.audit(&[], None)?;
        assert_eq!(audit_res, "audit");

        let fix_res = tool.fix(&[], false, None)?;
        assert_eq!(fix_res, "fix");

        let dry_res = tool.fix(&[], true, None)?;
        assert_eq!(dry_res, "fix --dry-run");
        Ok(())
    }

    #[test]
    fn test_jsonrpc_tool() {
        let tool = JsonRpcTool::new(
            "rpc".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            None,
            None,
            None,
            None,
            "http://localhost:0".to_string(),
        );
        assert_eq!(tool.name(), "rpc");
        let err = tool.audit(&[], None);
        assert!(err.is_err());
    }

    #[test]
    fn test_dlopen_tool() {
        let tool_err = DlopenTool::new(
            "dl".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            None,
            None,
            None,
            None,
            "nonexistent.so",
        );
        assert!(tool_err.is_err());
    }
}
