# Adding New Tools

## Purpose

The strength of `bridle-ctl` lies in deterministic codebase mutations. This document explains how to write a new tool, compile it, and integrate it into `bridle-ctl`.

With recent additions, `bridle-ctl` now supports **four** primary ways of integrating a tool:
1. Native Rust Implementations
2. Dynamically loaded C/C++/Rust libraries (`dlopen`)
3. Subprocess execution
4. JSON-RPC via HTTP

---

## 🔄 The Tool Integration Process

### Approach 1: Native Rust Implementation

If you are writing the tool natively within the `bridle-ctl` workspace (e.g. in `bridle-cli/src/tools/`), simply implement the `CodeTool` trait:

```rust
use crate::tools::CodeTool;
use crate::error::CliError;
use bridle_sdk::path_scope::PathScope;

pub struct MyNativeTool;

impl CodeTool for MyNativeTool {
    fn name(&self) -> &str { "my-native-tool" }
    fn description(&self) -> &str { "Does a thing" }
    fn match_regex(&self) -> &str { r".*\.txt$" }

    fn audit(&self, args: &[String], scope: Option<&PathScope>) -> Result<String, CliError> {
        // Implementation
        Ok("Found 1 issue".into())
    }

    fn fix(&self, args: &[String], dry_run: bool, scope: Option<&PathScope>) -> Result<String, CliError> {
        // Implementation
        Ok("Fixed 1 issue".into())
    }
}
```
Then add it to the `get_tools()` function in `bridle-cli/src/tools/registry.rs`.

---

### Approach 2: Dynamic / Fallback Tool Configuration (TOML)

You can also add tools without touching the Rust source code using TOML configuration files!

#### The Core Configuration File
Create or edit the `bridle-tools.toml` file in your workspace root. This core configuration file maps tool names to their specific `.toml` definitions and toggles their active state.

```toml
[plugins]
# Map internal tool names to their definition files
my-subprocess-tool = ".bridle-plugins/my-subprocess-tool.toml"
my-jsonrpc-tool    = ".bridle-plugins/my-jsonrpc-tool.toml"
my-dlopen-tool     = ".bridle-plugins/my-dlopen-tool.toml"

[enabled]
# Toggle plugins globally here
my-subprocess-tool = true
my-jsonrpc-tool    = false
my-dlopen-tool     = true
```

#### Individual Plugin Definitions
Each tool specifies its configuration in its own isolated TOML file. Below are examples of the three dynamic types:

**1. Subprocess Plugin (`.bridle-plugins/my-subprocess-tool.toml`)**
```toml
description = "Fixes things using python"
match_regex = ".*\\.py$"
type = "subprocess"
command = "/path/to/my-python-script.py"

[env]
# Optional: Securely pass environment variables or override the PATH!
PATH = "/my/custom/virtualenv/bin:/usr/bin"
MY_ARBITRARY_VAR = "some_value"
```
*The CLI will execute `my-command audit [args...]` and `my-command fix [--dry-run] [args...]`.*

**2. JSON-RPC Plugin (`.bridle-plugins/my-jsonrpc-tool.toml`)**
```toml
description = "Calls an external server"
match_regex = ".*"
type = "jsonrpc"
endpoint = "http://localhost:8080/rpc"
# Optional: Provide a command that developers could use to boot the server
launch_command = "npm start --prefix /path/to/my/server"
```
*The CLI posts JSON-RPC 2.0 standard payloads (`method="audit"` or `method="fix"`) to the specified endpoint.*

**3. Dlopen Plugin (`.bridle-plugins/my-dlopen-tool.toml`)**
```toml
description = "Uses a dynamic C library"
match_regex = ".*\\.c$"
type = "dlopen"
path = "./target/release/libmytool.so"
# Optional: Provide the command needed to build this .so object
build_command = "cargo build --release"
```

*Note: Your shared library (`.so`, `.dylib`, `.dll`) MUST expose two C-compatible functions exactly:*

```c
// Returns 0 on success, non-zero on error.
// out_ptr should point to a newly allocated string (or null).
int tool_audit(const char** args_ptr, size_t args_len, char** out_ptr);

int tool_fix(const char** args_ptr, size_t args_len, bool dry_run, char** out_ptr);
```

**4. Statically-linked FFI Plugin (`.bridle-plugins/my-ffi-tool.toml`)**
```toml
description = "Uses an internal statically-linked SDK FFI wrapper"
match_regex = ".*\\.go$"
type = "ffi"
wrapper = "go_auto_err"
# Optional: Provide a subcommand arg used by complex wrappers
subcommand = "my_subcmd"
```

### Testing Requirements

`bridle-ctl` enforces **100% Test Coverage** and **Strict Quality Standards**. 
1. If writing a native tool, add standard `#[test]` unit tests inside your module.
2. Ensure you handle missing arguments, bad data, and correctly implement `dry_run`.
3. Never use `.unwrap()`, `.expect()`, or `anyhow!`. Only return explicitly typed `CliError` instances utilizing `?`.
4. Typescript frontend tools must avoid `any`, `unknown`, and `never`.
