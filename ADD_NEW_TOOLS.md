# Adding New Tools

## Purpose

The strength of `bridle-ctl` lies in deterministic codebase mutations. Instead of allowing AI agents to generate error-prone string replacements, agents invoke compiled FFI (Foreign Function Interface) tools. 

This document explains how to write a new tool, compile it, and integrate it into the `bridle-sdk`.

---

## 🔄 The Tool Integration Process

Adding a new tool involves four main steps:
1. Write the logic in your preferred systems language (C, C++, Rust, Go).
2. Expose the required `extern "C"` functions (`audit` and `fix`).
3. Compile it as a shared library (`.so`, `.dylib`, `.dll`).
4. Bind and register it in `bridle-sdk/src/ffi.rs`.

### 1. The FFI Contract

Any tool integrating with `bridle-ctl` MUST expose two C-compatible functions:

```c
// Checks if the tool can/should operate on the given file.
// Returns a 1 (true) or 0 (false). You can also return a JSON string
// if more complex metadata needs to be passed back to the SDK.
int audit(const char* file_path);

// Executes the actual codebase mutation.
// Returns 0 on success, or a non-zero error code on failure.
int fix(const char* file_path, const char* json_args);
```

### 2. Example: Writing a Tool in Rust

Here is how you might write a simple tool in Rust that replaces `.unwrap()` with `?`.

Create a new library crate, set `crate-type = ["cdylib"]` in `Cargo.toml`, and write:

```rust
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::fs;

#[no_mangle]
pub extern "C" fn audit(file_path: *const c_char) -> c_int {
    let path = unsafe { CStr::from_ptr(file_path) }.to_str().unwrap_or("");
    if !path.ends_with(".rs") { return 0; }
    
    if let Ok(content) = fs::read_to_string(path) {
        if content.contains(".unwrap()") { return 1; }
    }
    0
}

#[no_mangle]
pub extern "C" fn fix(file_path: *const c_char, _args: *const c_char) -> c_int {
    let path = unsafe { CStr::from_ptr(file_path) }.to_str().unwrap_or("");
    
    // In a real tool, use the `syn` crate for AST-aware replacement!
    // This is a naive example.
    if let Ok(content) = fs::read_to_string(path) {
        let fixed = content.replace(".unwrap()", "?");
        if fs::write(path, fixed).is_ok() {
            return 0; // Success
        }
    }
    1 // Error
}
```

### 3. Binding the Tool in `bridle-sdk`

Once compiled to a shared library (e.g., `librust_unwrap_fixer.so`), you must register it in `bridle-sdk`.

Open `bridle-sdk/src/ffi.rs` and utilize the `libloading` crate (or our internal wrapper) to dynamically load the library and bind the functions to our internal `CodeTool` enum/struct.

```rust
// Example addition in bridle-sdk/src/ffi.rs

pub struct RustUnwrapFixer {
    library: libloading::Library,
}

impl CodeTool for RustUnwrapFixer {
    fn name(&self) -> &str { "rust-unwrap-to-question-mark" }
    
    fn audit(&self, path: &Path) -> Result<bool, Error> {
        // ... unsafe call to the loaded audit() symbol ...
    }
    
    fn fix(&self, path: &Path, args: Option<&str>) -> Result<(), Error> {
        // ... unsafe call to the loaded fix() symbol ...
    }
}
```

### 4. Testing Requirements

`bridle-ctl` enforces **100% Test Coverage**. When you add a new FFI binding:
1. Write unit tests in `bridle-sdk/src/ffi.rs` that load the shared library (or a mock version).
2. Ensure both success and failure states of the C ABI are handled correctly without panicking.
3. Verify memory safety (ensure strings passed across the FFI boundary are not leaked or prematurely freed).
4. Run `cargo tarpaulin` to guarantee your new integration lines are fully covered.