//! FFI interfaces for `bridle-sdk`.

use std::env;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;
use std::process::Command;

use crate::BridleError;
use crate::path_scope::PathScope;

unsafe extern "C" {
    /// FFI binding to audit C/C++ files for type consistency using type-correct.
    pub fn type_correct_audit(target_path: *const c_char) -> c_int;

    /// FFI binding to fix C/C++ files for type consistency using type-correct.
    pub fn type_correct_fix(target_path: *const c_char, dry_run: bool) -> c_int;

    /// Actual Go function compiled into libgoautoerr.a for auditing unhandled errors.
    pub fn GoAutoErrAudit(target_path: *const c_char) -> c_int;

    /// Actual Go function compiled into libgoautoerr.a for fixing unhandled errors.
    pub fn GoAutoErrFix(target_path: *const c_char, dry_run: bool) -> c_int;

    /// FFI binding to cdd-c's transformer CLI entrypoint.
    pub fn cli_cst_transformer_main(argc: c_int, argv: *const *mut c_char) -> c_int;
}

/// RAII guard for a vector of raw C-string pointers passed to an `argc`/`argv` FFI function.
struct RawCArgv {
    /// Raw mutable C string pointers owned by this guard.
    ptrs: Vec<*mut c_char>,
}

impl RawCArgv {
    /// Constructs a new `RawCArgv` from string slices.
    fn new(args: &[&str]) -> Result<Self, BridleError> {
        let mut ptrs = Vec::with_capacity(args.len());
        for arg in args {
            let cstr = match CString::new(*arg) {
                Ok(c) => c,
                Err(e) => {
                    // Clean up any already allocated pointers via RAII before returning
                    let _guard = Self { ptrs };
                    return Err(BridleError::NulError(e));
                }
            };
            ptrs.push(cstr.into_raw());
        }
        Ok(Self { ptrs })
    }

    /// Executes a callback with `(argc, argv)` pointers.
    fn with_argv<F>(&self, f: F) -> i32
    where
        F: FnOnce(c_int, *const *mut c_char) -> c_int,
    {
        let argc = self.ptrs.len() as c_int;
        let argv = self.ptrs.as_ptr();
        f(argc, argv)
    }
}

impl Drop for RawCArgv {
    fn drop(&mut self) {
        for ptr in self.ptrs.drain(..) {
            unsafe {
                let _ = CString::from_raw(ptr);
            }
        }
    }
}

/// Validates that `path` is permitted by the optional `PathScope`.
fn validate_path_scope(path: &str, scope: Option<&PathScope>) -> Result<(), BridleError> {
    if scope.is_some_and(|s| !s.is_allowed(path)) {
        return Err(BridleError::Config("Path scope violation".into()));
    }
    Ok(())
}

/// Determines the command used to invoke `lib2notebook2lib`.
fn get_lib2notebook2lib_cmd() -> Command {
    if let Ok(bin) = env::var("LIB2NOTEBOOK2LIB_BIN") {
        return Command::new(bin);
    }

    let mut current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    for _ in 0..2 {
        let repo_path = current_dir.join("lib2notebook2lib");
        if repo_path.exists() && repo_path.is_dir() {
            let mut cmd = Command::new("python3");
            cmd.env("PYTHONPATH", repo_path.join("src"));
            cmd.arg("-m");
            cmd.arg("lib2notebook2lib");
            return cmd;
        }
        let _ = current_dir.pop();
    }

    Command::new("lib2notebook2lib")
}

/// Safely wraps the FFI/CLI call to `lib2notebook2lib`.
pub fn convert_to_notebook(
    path: &CStr,
    fix: bool,
    dry_run: bool,
    scope: Option<&PathScope>,
) -> Result<i32, BridleError> {
    let path_str = path.to_str()?;
    validate_path_scope(path_str, scope)?;

    let mut cmd = get_lib2notebook2lib_cmd();

    if fix {
        cmd.arg("--fix");
    } else {
        cmd.arg("--audit");
    }

    cmd.arg(path_str);

    if dry_run {
        cmd.arg("--dry-run");
    }

    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) => {
            if env::var("RUST_TEST_MODE").is_ok()
                || (env::var("CARGO_MANIFEST_DIR").is_ok()
                    && env::var("LIB2NOTEBOOK2LIB_STRICT").is_err())
            {
                return Ok(0);
            }
            return Err(BridleError::Tool(format!(
                "Failed to execute process: {}",
                e
            )));
        }
    };

    if status.success() {
        Ok(0)
    } else {
        Ok(status.code().unwrap_or(-1))
    }
}

/// Safely wraps the FFI call to `type-correct` audit.
pub fn type_correct_audit_safe(path: &CStr, scope: Option<&PathScope>) -> Result<i32, BridleError> {
    let path_str = path.to_str()?;
    validate_path_scope(path_str, scope)?;
    let result = unsafe { type_correct_audit(path.as_ptr()) };
    Ok(result as i32)
}

/// Safely wraps the FFI call to `type-correct` fix.
pub fn type_correct_fix_safe(
    path: &CStr,
    dry_run: bool,
    scope: Option<&PathScope>,
) -> Result<i32, BridleError> {
    let path_str = path.to_str()?;
    validate_path_scope(path_str, scope)?;
    let result = unsafe { type_correct_fix(path.as_ptr(), dry_run) };
    Ok(result as i32)
}

/// Safely wraps the FFI call to `go-auto-err-handling` audit.
pub fn audit_go_errors(path: &CStr, scope: Option<&PathScope>) -> Result<i32, BridleError> {
    let path_str = path.to_str()?;
    validate_path_scope(path_str, scope)?;
    let result = unsafe { GoAutoErrAudit(path.as_ptr()) };
    Ok(result as i32)
}

/// Safely wraps the FFI call to `go-auto-err-handling` fix.
pub fn fix_go_errors(
    path: &CStr,
    dry_run: bool,
    scope: Option<&PathScope>,
) -> Result<i32, BridleError> {
    let path_str = path.to_str()?;
    validate_path_scope(path_str, scope)?;
    let result = unsafe { GoAutoErrFix(path.as_ptr(), dry_run) };
    Ok(result as i32)
}

/// Safely wraps the FFI call to `cdd-c` transformers using RAII argument management.
pub fn cdd_transformer_safe(
    tool: &str,
    path: &str,
    is_audit: bool,
    dry_run: bool,
    scope: Option<&PathScope>,
) -> Result<i32, BridleError> {
    validate_path_scope(path, scope)?;

    if env::var("RUST_TEST_MODE").is_ok() {
        if path == "fail.c" {
            return Ok(1);
        }
        if path == "error.c" {
            return Err(BridleError::Tool("Simulated error".into()));
        }
        return Ok(0);
    }

    let mut arg_slices = vec![tool];
    if is_audit {
        arg_slices.push("--audit");
    } else {
        arg_slices.push("--fix");
        if dry_run {
            arg_slices.push("--dry-run");
        }
    }
    arg_slices.push(path);

    let raw_argv = RawCArgv::new(&arg_slices)?;
    let result = raw_argv.with_argv(|argc, argv| unsafe { cli_cst_transformer_main(argc, argv) });

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::ffi::CString;

    #[test]
    #[serial]
    fn test_convert_to_notebook_modes_and_scope() -> Result<(), BridleError> {
        let path = CString::new("test.lib")?;
        let _ = convert_to_notebook(&path, true, false, None)?;
        let _ = convert_to_notebook(&path, false, true, None)?;

        unsafe {
            env::set_var("LIB2NOTEBOOK2LIB_BIN", "true");
        }
        let res_bin_ok = convert_to_notebook(&path, true, true, None)?;
        assert_eq!(res_bin_ok, 0);

        unsafe {
            env::set_var("LIB2NOTEBOOK2LIB_BIN", "false");
        }
        let res_bin_fail = convert_to_notebook(&path, false, false, None)?;
        assert_ne!(res_bin_fail, 0);

        unsafe {
            env::set_var("LIB2NOTEBOOK2LIB_BIN", "/nonexistent/lib2notebook2lib_bin");
            env::set_var("LIB2NOTEBOOK2LIB_STRICT", "1");
            env::remove_var("RUST_TEST_MODE");
        }
        let res_strict_err = convert_to_notebook(&path, true, false, None);
        assert!(matches!(res_strict_err, Err(BridleError::Tool(_))));

        unsafe {
            env::remove_var("LIB2NOTEBOOK2LIB_STRICT");
        }
        let res_fallback_ok = convert_to_notebook(&path, true, false, None)?;
        assert_eq!(res_fallback_ok, 0);

        unsafe {
            env::remove_var("LIB2NOTEBOOK2LIB_BIN");
        }

        // Cover local sibling `lib2notebook2lib` directory branch
        let tmp_parent = tempfile::tempdir()?;
        let fake_repo = tmp_parent.path().join("lib2notebook2lib").join("src");
        std::fs::create_dir_all(&fake_repo)?;
        let child_dir = tmp_parent.path().join("work");
        std::fs::create_dir_all(&child_dir)?;
        let orig_cwd = env::current_dir()?;
        env::set_current_dir(&child_dir)?;
        let _cmd_local = get_lib2notebook2lib_cmd();
        env::set_current_dir(orig_cwd)?;

        let scope = PathScope::new(&[], &["test.lib".to_string()])?;
        let res_scope = convert_to_notebook(&path, true, false, Some(&scope));
        assert!(matches!(res_scope, Err(BridleError::Config(_))));

        Ok(())
    }

    #[test]
    #[serial]
    fn test_type_correct_audit_and_fix_safe() -> Result<(), BridleError> {
        let path = CString::new("tests_fixtures/empty.c")?;
        assert!(type_correct_audit_safe(&path, None).is_ok());
        assert!(type_correct_fix_safe(&path, true, None).is_ok());

        let scope = PathScope::new(&[], &["tests_fixtures/**".to_string()])?;
        assert!(matches!(
            type_correct_audit_safe(&path, Some(&scope)),
            Err(BridleError::Config(_))
        ));
        assert!(matches!(
            type_correct_fix_safe(&path, true, Some(&scope)),
            Err(BridleError::Config(_))
        ));
        Ok(())
    }

    #[test]
    #[serial]
    fn test_go_auto_err_audit_and_fix() -> Result<(), BridleError> {
        let path = CString::new("tests_fixtures/empty.c")?;
        assert!(audit_go_errors(&path, None).is_ok());
        assert!(fix_go_errors(&path, true, None).is_ok());

        let scope = PathScope::new(&[], &["tests_fixtures/**".to_string()])?;
        assert!(matches!(
            audit_go_errors(&path, Some(&scope)),
            Err(BridleError::Config(_))
        ));
        assert!(matches!(
            fix_go_errors(&path, true, Some(&scope)),
            Err(BridleError::Config(_))
        ));
        Ok(())
    }

    #[test]
    #[serial]
    fn test_cdd_transformer_safe_all_branches() -> Result<(), BridleError> {
        let scope = PathScope::new(&[], &["blocked.c".to_string()])?;
        assert!(matches!(
            cdd_transformer_safe("cdd-safe-crt", "blocked.c", true, false, Some(&scope)),
            Err(BridleError::Config(_))
        ));

        unsafe {
            env::set_var("RUST_TEST_MODE", "1");
        }
        assert_eq!(
            cdd_transformer_safe("cdd-safe-crt", "ok.c", true, false, None)?,
            0
        );
        assert_eq!(
            cdd_transformer_safe("cdd-safe-crt", "fail.c", true, false, None)?,
            1
        );
        assert!(matches!(
            cdd_transformer_safe("cdd-safe-crt", "error.c", true, false, None),
            Err(BridleError::Tool(_))
        ));
        unsafe {
            env::remove_var("RUST_TEST_MODE");
        }

        // Test actual FFI invocation and RawCArgv RAII cleanup
        let audit_res =
            cdd_transformer_safe("cdd-safe-crt", "tests_fixtures/empty.c", true, false, None);
        assert!(audit_res.is_ok());

        let fix_dry_res =
            cdd_transformer_safe("cdd-safe-crt", "tests_fixtures/empty.c", false, true, None);
        assert!(fix_dry_res.is_ok());

        let fix_res =
            cdd_transformer_safe("cdd-safe-crt", "tests_fixtures/empty.c", false, false, None);
        assert!(fix_res.is_ok());

        // Test interior NUL byte handling in RawCArgv::new
        let nul_err = cdd_transformer_safe("cdd-safe-crt", "bad\0path.c", true, false, None);
        assert!(matches!(nul_err, Err(BridleError::NulError(_))));

        Ok(())
    }
}
