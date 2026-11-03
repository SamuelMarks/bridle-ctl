//! Dynamic tool implementations
use std::env;
use std::ffi::{CStr, CString};
use std::process::Command;
use std::sync::Arc;

use super::CodeTool;
use bridle_sdk::BridleError;
use bridle_sdk::path_scope::PathScope;

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

    /// Prepends `bin_dir` to `PATH` inside `envs`.
    fn prepend_bin_to_path(
        envs: &mut std::collections::HashMap<String, String>,
        bin_dir: std::path::PathBuf,
    ) {
        let current_path = std::env::var("PATH").unwrap_or_default();
        let new_path = std::env::join_paths(
            std::iter::once(bin_dir).chain(std::env::split_paths(&current_path)),
        )
        .unwrap_or_default();
        envs.insert("PATH".to_string(), new_path.to_string_lossy().to_string());
    }

    /// Configures the command with arguments and environment variables.
    fn configure_command(
        &self,
        arg_action: &str,
        args: &[String],
        dry_run: Option<bool>,
    ) -> Command {
        #[cfg(windows)]
        const VENV_BIN_SUBDIR: &str = "Scripts";
        #[cfg(not(windows))]
        const VENV_BIN_SUBDIR: &str = "bin";

        let mut cmd = Command::new(&self.command);
        let mut envs = self.env.clone();

        if self.venv_aware {
            if let Ok(venv) = std::env::var("VIRTUAL_ENV") {
                let venv_path = std::path::PathBuf::from(venv);
                Self::prepend_bin_to_path(&mut envs, venv_path.join(VENV_BIN_SUBDIR));
            } else {
                let mut local_venvs = vec![".venv".to_string(), "venv".to_string()];

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
                        Self::prepend_bin_to_path(&mut envs, venv_path.join(VENV_BIN_SUBDIR));
                        let canonical = venv_path
                            .canonicalize()
                            .unwrap_or_else(|_| venv_path.to_path_buf());
                        envs.insert("VIRTUAL_ENV".to_string(), canonical.display().to_string());
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

    fn audit(&self, args: &[String], _scope: Option<&PathScope>) -> Result<String, BridleError> {
        let mut cmd = self.configure_command("audit", args, None);
        let output = cmd.output()?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?.trim().to_string())
        } else {
            Err(BridleError::Tool(
                String::from_utf8(output.stderr)?.trim().to_string(),
            ))
        }
    }

    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, BridleError> {
        let mut cmd = self.configure_command("fix", args, Some(dry_run));
        let output = cmd.output()?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?.trim().to_string())
        } else {
            Err(BridleError::Tool(
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
    fn call_rpc(&self, method: &str, params: serde_json::Value) -> Result<String, BridleError> {
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
            return Err(BridleError::Tool(err.to_string()));
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

    fn audit(&self, args: &[String], _scope: Option<&PathScope>) -> Result<String, BridleError> {
        self.call_rpc("audit", serde_json::json!({ "args": args }))
    }

    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, BridleError> {
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
    ) -> Result<Self, BridleError> {
        let lib = unsafe { libloading::Library::new(path)? };
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
    ) -> Result<String, BridleError> {
        let mut c_args = Vec::with_capacity(args.len());
        for s in args {
            c_args.push(CString::new(s.as_str())?);
        }

        let c_args_ptrs: Vec<*const std::ffi::c_char> = c_args.iter().map(|c| c.as_ptr()).collect();
        let mut out_ptr: *mut std::ffi::c_char = std::ptr::null_mut();

        let res = unsafe {
            if let Some(dr) = dry_run {
                let func: libloading::Symbol<FixFunc> = self.lib.get(func_name)?;
                func(c_args_ptrs.as_ptr(), c_args_ptrs.len(), dr, &mut out_ptr)
            } else {
                let func: libloading::Symbol<AuditFunc> = self.lib.get(func_name)?;
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
            Err(BridleError::Tool(format!(
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

    fn audit(&self, args: &[String], _scope: Option<&PathScope>) -> Result<String, BridleError> {
        self.call_c_func(b"tool_audit\0", args, None)
    }

    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, BridleError> {
        self.call_c_func(b"tool_fix\0", args, Some(dry_run))
    }
}

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

    fn audit(&self, args: &[String], scope: Option<&PathScope>) -> Result<String, BridleError> {
        let path_str = args.first().map(|s| s.as_str()).unwrap_or(".");
        let c_path = CString::new(path_str)?;

        match self.wrapper.as_str() {
            "type_correct" => {
                if args.is_empty() {
                    return Err(BridleError::Tool(
                        "Missing target file argument".to_string(),
                    ));
                }
                let result = bridle_sdk::ffi::type_correct_audit_safe(&c_path, scope)?;
                if result != 0 {
                    return Err(BridleError::Tool(format!(
                        "Audit returned error code: {}",
                        result
                    )));
                }
                Ok("type-correct audit executed successfully".into())
            }
            "go_auto_err" => {
                let result = bridle_sdk::ffi::audit_go_errors(&c_path, scope)?;
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
                let res = bridle_sdk::ffi::convert_to_notebook(&c_path, false, false, scope)?;
                if res == 0 {
                    Ok(format!(
                        "lib2notebook2lib audit executed successfully for {}",
                        args[0]
                    ))
                } else {
                    Err(BridleError::Tool(format!(
                        "Audit failed with exit code: {}",
                        res
                    )))
                }
            }
            "cdd" => {
                if args.is_empty() {
                    return Err(BridleError::Tool(
                        "Missing target file argument".to_string(),
                    ));
                }
                let subcmd = self.subcommand.as_deref().unwrap_or("");
                let result =
                    bridle_sdk::ffi::cdd_transformer_safe(subcmd, path_str, true, false, scope)?;
                if result != 0 {
                    return Err(BridleError::Tool(format!("{} failed", self.name)));
                }
                Ok(format!("{} audit executed", self.name))
            }
            _ => Err(BridleError::Config(format!(
                "Unknown wrapper: {}",
                self.wrapper
            ))),
        }
    }

    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        scope: Option<&PathScope>,
    ) -> Result<String, BridleError> {
        let path_str = args.first().map(|s| s.as_str()).unwrap_or(".");
        let c_path = CString::new(path_str)?;

        match self.wrapper.as_str() {
            "type_correct" => {
                if args.is_empty() {
                    return Err(BridleError::Tool(
                        "Missing target file argument".to_string(),
                    ));
                }
                let result = bridle_sdk::ffi::type_correct_fix_safe(&c_path, dry_run, scope)?;
                if result != 0 {
                    return Err(BridleError::Tool(format!(
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
                let result = bridle_sdk::ffi::fix_go_errors(&c_path, dry_run, scope)?;
                let dry_prefix = if dry_run { "[DRY RUN] " } else { "" };
                if result == 0 {
                    Ok(format!(
                        "{}go-auto-err-handling fix applied successfully to {}",
                        dry_prefix, path_str
                    ))
                } else {
                    Err(BridleError::Tool(format!(
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
                let res = bridle_sdk::ffi::convert_to_notebook(&c_path, true, dry_run, scope)?;
                if res == 0 {
                    let dry_prefix = if dry_run { "[DRY RUN] " } else { "" };
                    Ok(format!(
                        "{}lib2notebook2lib fix applied successfully to {}",
                        dry_prefix, args[0]
                    ))
                } else {
                    Err(BridleError::Tool(format!(
                        "Fix failed with exit code: {}",
                        res
                    )))
                }
            }
            "cdd" => {
                if args.is_empty() {
                    return Err(BridleError::Tool(
                        "Missing target file argument".to_string(),
                    ));
                }
                let subcmd = self.subcommand.as_deref().unwrap_or("");
                let result =
                    bridle_sdk::ffi::cdd_transformer_safe(subcmd, path_str, false, dry_run, scope)?;
                if result != 0 {
                    return Err(BridleError::Tool(format!("{} failed", self.name)));
                }
                if dry_run {
                    Ok(format!("[DRY RUN] {} fix planned", self.name))
                } else {
                    Ok(format!("{} fix applied", self.name))
                }
            }
            _ => Err(BridleError::Config(format!(
                "Unknown wrapper: {}",
                self.wrapper
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::CodeTool;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_subprocess_tool_metadata_and_venv() -> Result<(), BridleError> {
        let tool = SubprocessTool::new(
            "test".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            Some("1.0".to_string()),
            Some("author".to_string()),
            Some("http://url".to_string()),
            Some("MIT".to_string()),
            "echo".to_string(),
            std::collections::HashMap::new(),
            true,
        );
        assert_eq!(tool.name(), "test");
        assert_eq!(tool.description(), "desc");
        assert_eq!(tool.match_regex(), ".*");
        assert_eq!(tool.version(), Some("1.0"));
        assert_eq!(tool.author(), Some("author"));
        assert_eq!(tool.url(), Some("http://url"));
        assert_eq!(tool.license(), Some("MIT"));

        assert_eq!(tool.audit(&[], None)?, "audit");
        assert_eq!(tool.fix(&[], false, None)?, "fix");
        assert_eq!(tool.fix(&[], true, None)?, "fix --dry-run");

        unsafe {
            env::set_var("VIRTUAL_ENV", "/tmp/fake_venv");
        }
        assert_eq!(tool.audit(&[], None)?, "audit");
        unsafe {
            env::remove_var("VIRTUAL_ENV");
        }

        let tmp_dir = tempfile::tempdir()?;
        let orig_dir = env::current_dir()?;
        env::set_current_dir(tmp_dir.path())?;
        std::fs::create_dir_all(".venv-3-12/bin")?;
        std::fs::create_dir_all(".venv/bin")?;
        let _ = tool.audit(&[], None)?;
        env::set_current_dir(orig_dir)?;

        let fail_tool = SubprocessTool::new(
            "fail".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            None,
            None,
            None,
            None,
            "false".to_string(),
            std::collections::HashMap::new(),
            false,
        );
        assert!(matches!(
            fail_tool.audit(&[], None),
            Err(BridleError::Tool(_))
        ));
        assert!(matches!(
            fail_tool.fix(&[], false, None),
            Err(BridleError::Tool(_))
        ));

        Ok(())
    }

    #[test]
    fn test_jsonrpc_tool_success_and_error() -> Result<(), BridleError> {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();

        let handle = std::thread::spawn(move || {
            for i in 0..3 {
                if let Ok((mut stream, _)) = listener.accept() {
                    let mut buf = [0u8; 1024];
                    let _ = stream.read(&mut buf);
                    let body = if i < 2 {
                        r#"{"jsonrpc":"2.0","result":"rpc_ok","id":1}"#
                    } else {
                        r#"{"jsonrpc":"2.0","error":{"code":-32000,"message":"fail"},"id":1}"#
                    };
                    let resp = format!(
                        "HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: {}

{}",
                        body.len(),
                        body
                    );
                    let _ = stream.write_all(resp.as_bytes());
                }
            }
        });

        let tool = JsonRpcTool::new(
            "rpc".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            Some("1.0".to_string()),
            Some("author".to_string()),
            Some("http://url".to_string()),
            Some("MIT".to_string()),
            format!("http://127.0.0.1:{}", port),
        );
        assert_eq!(tool.name(), "rpc");
        assert_eq!(tool.description(), "desc");
        assert_eq!(tool.match_regex(), ".*");
        assert_eq!(tool.version(), Some("1.0"));
        assert_eq!(tool.author(), Some("author"));
        assert_eq!(tool.url(), Some("http://url"));
        assert_eq!(tool.license(), Some("MIT"));

        assert_eq!(tool.audit(&[], None)?, "rpc_ok");
        assert_eq!(tool.fix(&[], true, None)?, "rpc_ok");
        assert!(matches!(tool.audit(&[], None), Err(BridleError::Tool(_))));

        let _ = handle.join();
        Ok(())
    }

    #[test]
    fn test_dlopen_tool() -> Result<(), BridleError> {
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
        assert!(matches!(tool_err, Err(BridleError::LibLoading(_))));

        // Compile a tiny C shared library in a temp directory to test DlopenTool end-to-end
        let tmp_dir = tempfile::tempdir()?;
        let c_file = tmp_dir.path().join("plugin.c");
        let lib_file = tmp_dir
            .path()
            .join(format!("libplugin.{}", std::env::consts::DLL_EXTENSION));
        std::fs::write(
            &c_file,
            r#"
            int tool_audit(const char** args, unsigned long len, char** out) {
                if (len == 0) { *out = "audit_ok"; return 0; }
                *out = 0;
                return 1;
            }
            int tool_fix(const char** args, unsigned long len, _Bool dry, char** out) {
                *out = dry ? "fix_dry" : "fix_ok";
                return 0;
            }
            "#,
        )?;
        let status = Command::new("cc")
            .args([
                "-shared",
                "-fPIC",
                c_file.to_str().unwrap_or(""),
                "-o",
                lib_file.to_str().unwrap_or(""),
            ])
            .status()?;
        assert!(status.success());

        let dl_tool = DlopenTool::new(
            "dl".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            Some("1.0".to_string()),
            Some("author".to_string()),
            Some("http://url".to_string()),
            Some("MIT".to_string()),
            lib_file.to_str().unwrap_or(""),
        )?;
        assert_eq!(dl_tool.name(), "dl");
        assert_eq!(dl_tool.description(), "desc");
        assert_eq!(dl_tool.match_regex(), ".*");
        assert_eq!(dl_tool.version(), Some("1.0"));
        assert_eq!(dl_tool.author(), Some("author"));
        assert_eq!(dl_tool.url(), Some("http://url"));
        assert_eq!(dl_tool.license(), Some("MIT"));

        assert_eq!(dl_tool.audit(&[], None)?, "audit_ok");
        assert!(matches!(
            dl_tool.audit(&["arg".to_string()], None),
            Err(BridleError::Tool(_))
        ));
        assert_eq!(dl_tool.fix(&[], false, None)?, "fix_ok");
        assert_eq!(dl_tool.fix(&[], true, None)?, "fix_dry");

        Ok(())
    }

    #[test]
    #[serial]
    fn test_ffi_tool_all_wrappers() -> Result<(), BridleError> {
        let ffi_meta = FfiTool::new(
            "meta".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            Some("1.0".to_string()),
            Some("author".to_string()),
            Some("http://url".to_string()),
            Some("MIT".to_string()),
            "unknown".to_string(),
            None,
        );
        assert_eq!(ffi_meta.name(), "meta");
        assert_eq!(ffi_meta.description(), "desc");
        assert_eq!(ffi_meta.match_regex(), ".*");
        assert_eq!(ffi_meta.version(), Some("1.0"));
        assert_eq!(ffi_meta.author(), Some("author"));
        assert_eq!(ffi_meta.url(), Some("http://url"));
        assert_eq!(ffi_meta.license(), Some("MIT"));
        assert!(matches!(
            ffi_meta.audit(&[], None),
            Err(BridleError::Config(_))
        ));
        assert!(matches!(
            ffi_meta.fix(&[], false, None),
            Err(BridleError::Config(_))
        ));

        let cdd_tool = FfiTool::new(
            "cdd-test".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            None,
            None,
            None,
            None,
            "cdd".to_string(),
            Some("cdd-safe-crt".to_string()),
        );
        assert!(cdd_tool.audit(&[], None).is_err());
        assert!(cdd_tool.fix(&[], false, None).is_err());

        unsafe {
            env::set_var("RUST_TEST_MODE", "1");
        }
        assert_eq!(
            cdd_tool.audit(&["ok.c".to_string()], None)?,
            "cdd-test audit executed"
        );
        assert!(cdd_tool.audit(&["fail.c".to_string()], None).is_err());
        assert_eq!(
            cdd_tool.fix(&["ok.c".to_string()], false, None)?,
            "cdd-test fix applied"
        );
        assert_eq!(
            cdd_tool.fix(&["ok.c".to_string()], true, None)?,
            "[DRY RUN] cdd-test fix planned"
        );
        assert!(cdd_tool.fix(&["fail.c".to_string()], false, None).is_err());
        unsafe {
            env::remove_var("RUST_TEST_MODE");
        }

        let l2n_tool = FfiTool::new(
            "l2n".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            None,
            None,
            None,
            None,
            "lib2notebook".to_string(),
            None,
        );
        assert_eq!(l2n_tool.audit(&[], None)?, "No file provided");
        assert_eq!(l2n_tool.fix(&[], false, None)?, "No file provided");

        unsafe {
            env::set_var("LIB2NOTEBOOK2LIB_BIN", "true");
        }
        assert!(
            l2n_tool
                .audit(&["test.py".to_string()], None)?
                .contains("executed successfully")
        );
        assert!(
            l2n_tool
                .fix(&["test.py".to_string()], false, None)?
                .contains("applied successfully")
        );
        assert!(
            l2n_tool
                .fix(&["test.py".to_string()], true, None)?
                .contains("[DRY RUN]")
        );

        unsafe {
            env::set_var("LIB2NOTEBOOK2LIB_BIN", "false");
        }
        assert!(l2n_tool.audit(&["test.py".to_string()], None).is_err());
        assert!(l2n_tool.fix(&["test.py".to_string()], false, None).is_err());
        unsafe {
            env::remove_var("LIB2NOTEBOOK2LIB_BIN");
        }

        let tc_tool = FfiTool::new(
            "tc".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            None,
            None,
            None,
            None,
            "type_correct".to_string(),
            None,
        );
        let tmp_dir = tempfile::tempdir()?;
        let valid_c = tmp_dir.path().join("valid.c");
        std::fs::write(&valid_c, "int main() { return 0; }\n")?;
        let valid_c_str = valid_c.to_string_lossy().to_string();
        assert!(
            tc_tool
                .audit(std::slice::from_ref(&valid_c_str), None)
                .is_ok()
        );
        assert!(
            tc_tool
                .fix(std::slice::from_ref(&valid_c_str), true, None)
                .is_ok()
        );
        assert!(
            tc_tool
                .fix(std::slice::from_ref(&valid_c_str), false, None)
                .is_ok()
        );
        assert!(
            tc_tool
                .fix(&["/nonexistent/bad.c".to_string()], false, None)
                .is_err()
        );

        let go_tool = FfiTool::new(
            "go".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            None,
            None,
            None,
            None,
            "go_auto_err".to_string(),
            None,
        );
        let bad_go = tmp_dir.path().join("bad.go");
        std::fs::write(
            &bad_go,
            "package main\nfunc fail() error { return nil }\nfunc main() { fail() }\n",
        )?;
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&bad_go)?.permissions();
        perms.set_mode(0o400);
        std::fs::set_permissions(&bad_go, perms)?;
        let bad_go_str = bad_go.to_string_lossy().to_string();
        assert!(
            go_tool
                .fix(std::slice::from_ref(&bad_go_str), false, None)
                .is_err()
        );

        Ok(())
    }
}
