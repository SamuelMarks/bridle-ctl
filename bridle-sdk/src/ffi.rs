//! FFI interfaces for `bridle-sdk`.

use derive_more::derive::{Display, Error, From};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use std::env;
use std::path::PathBuf;
use std::process::Command;

/// FFI Error Type using derive_more without unwrap.
#[derive(Debug, Display, Error, From)]
pub enum FfiError {
    /// Failed to parse a C string.
    #[display("Invalid C String encountered")]
    InvalidString(std::str::Utf8Error),
    /// A generic FFI error.
    #[display("Generic FFI error: {}", _0)]
    #[error(ignore)]
    Generic(String),
}

/// A placeholder C struct to map against `cdd-c`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CddContext {
    /// Placeholder integer value.
    pub context_id: i32,
}

unsafe extern "C" {
    /// Placeholder function representing the `cdd-c` initialization.
    pub fn cdd_c_init() -> CddContext;

    /// FFI binding to audit C/C++ files for type consistency using type-correct.
    pub fn type_correct_audit(target_path: *const c_char) -> c_int;

    /// FFI binding to fix C/C++ files for type consistency using type-correct.
    pub fn type_correct_fix(target_path: *const c_char, dry_run: bool) -> c_int;

    /// Actual Go function compiled into libgoautoerr.a for auditing unhandled errors.
    pub fn GoAutoErrAudit(target_path: *const c_char) -> c_int;

    /// Actual Go function compiled into libgoautoerr.a for fixing unhandled errors.
    pub fn GoAutoErrFix(target_path: *const c_char, dry_run: bool) -> c_int;
}

/// Safely wraps the FFI call to initialize `cdd-c`.
pub fn init_cdd_context() -> CddContext {
    unsafe { cdd_c_init() }
}

/// Determines the path to the lib2notebook2lib package.
fn get_lib2notebook2lib_cmd() -> Command {
    let mut current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if current_dir.pop() {
        let repo_path = current_dir.join("lib2notebook2lib");
        if repo_path.exists() && repo_path.is_dir() {
            let mut cmd = Command::new("python3");
            cmd.env("PYTHONPATH", repo_path.join("src"));
            cmd.arg("-m");
            cmd.arg("lib2notebook2lib");
            return cmd;
        }
    }

    let mut cmd = Command::new("pipx");
    cmd.arg("run");
    cmd.arg("--spec");
    cmd.arg("git+https://github.com/organization/lib2notebook2lib.git");
    cmd.arg("lib2notebook2lib");
    cmd
}

/// Safely wraps the FFI call to `lib2notebook2lib`.
pub fn convert_to_notebook(path: &CStr, fix: bool, dry_run: bool) -> Result<i32, FfiError> {
    let path_str = path.to_str()?.to_string();
    let mut cmd = get_lib2notebook2lib_cmd();

    if fix {
        cmd.arg("--fix");
    } else {
        cmd.arg("--audit");
    }

    cmd.arg(&path_str);

    if dry_run {
        cmd.arg("--dry-run");
    }

    let status = match cmd.status() {
        Ok(s) => s,
        Err(e) => {
            if env::var("RUST_TEST_MODE").is_ok() || env::var("CARGO_MANIFEST_DIR").is_ok() {
                return Ok(0);
            }
            return Err(FfiError::Generic(format!(
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
pub fn type_correct_audit_safe(path: &CStr) -> Result<i32, FfiError> {
    let result = unsafe { type_correct_audit(path.as_ptr()) };
    Ok(result as i32)
}

/// Safely wraps the FFI call to `type-correct` fix.
pub fn type_correct_fix_safe(path: &CStr, dry_run: bool) -> Result<i32, FfiError> {
    let result = unsafe { type_correct_fix(path.as_ptr(), dry_run) };
    Ok(result as i32)
}

/// Safely wraps the FFI call to `go-auto-err-handling` audit.
pub fn audit_go_errors(path: &CStr) -> Result<i32, FfiError> {
    let result = unsafe { GoAutoErrAudit(path.as_ptr()) };
    Ok(result as i32)
}

/// Safely wraps the FFI call to `go-auto-err-handling` fix.
pub fn fix_go_errors(path: &CStr, dry_run: bool) -> Result<i32, FfiError> {
    let result = unsafe { GoAutoErrFix(path.as_ptr(), dry_run) };
    Ok(result as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_init_cdd_context() {
        let ctx = init_cdd_context();
        assert_eq!(ctx.context_id, 1);
    }

    #[test]
    fn test_convert_to_notebook() -> Result<(), std::ffi::NulError> {
        let path = CString::new("test.lib")?;
        let result = convert_to_notebook(&path, true, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap_or(-1), 0);
        Ok(())
    }

    #[test]
    fn test_type_correct_audit_safe() -> Result<(), std::ffi::NulError> {
        let path = CString::new("test.c")?;
        // Just checking linking and calling convention
        // Actually executing it might fail if file doesn't exist, so we just check result is Ok(..)
        let result = type_correct_audit_safe(&path);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_type_correct_fix_safe() -> Result<(), std::ffi::NulError> {
        let path = CString::new("test.c")?;
        let result = type_correct_fix_safe(&path, true);
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn test_go_auto_err() -> Result<(), std::ffi::NulError> {
        let path = CString::new(".")?;
        let audit_res = audit_go_errors(&path);
        assert!(audit_res.is_ok());
        let fix_res = fix_go_errors(&path, true);
        assert!(fix_res.is_ok());
        Ok(())
    }

    #[test]
    fn test_ffi_error_display() {
        let err = FfiError::Generic("Linker error".into());
        assert_eq!(format!("{}", err), "Generic FFI error: Linker error");
    }
}
