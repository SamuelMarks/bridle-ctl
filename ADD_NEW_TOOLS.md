# Adding New Tools to Bridle

This guide explains how to build and integrate new code processing tools—written in any language (C, Go, TypeScript, Java, etc.)—into the `bridle-ctl` system.

*Note: While `bridle-ctl` also includes a full embedded Git Forge backend (`bridle-sdk/db`), this document focuses specifically on integrating codebase orchestration tools.*

## The `CodeTool` Interface

To be natively usable inside `bridle-ctl` and not just as a standalone utility, every tool must ultimately conform to the `CodeTool` Rust trait defined in `bridle-cli/src/tools/mod.rs`. 

When integrating a tool, you will create a Rust struct in `bridle-cli/src/tools/registry.rs` that implements this trait:

```rust
pub trait CodeTool {
    /// Returns the unique name of the tool, e.g., "my-go-tool"
    fn name(&self) -> &'static str;
    
    /// A short description of what the tool does
    fn description(&self) -> &'static str;
    
    /// The regex pattern of files this tool targets, e.g., r".*\.go$"
    fn match_regex(&self) -> &'static str;

    /// Runs the logic for identifying issues.
    fn audit(&self, args: &[String]) -> Result<String, CliError>;

    /// Runs the logic for automatically fixing issues.
    fn fix(&self, args: &[String], dry_run: bool) -> Result<String, CliError>;
}
```

**Why Regex Matching?**
Tools in `bridle-ctl` target files using exact regex patterns (e.g., `\.github/workflows/.*\.ya?ml$`) rather than generic file extensions. This prevents large, uncontrolled dumps of arbitrary files from entering the agent's context and guarantees that a tool exclusively interacts with files it was strictly built to handle.

---

## Integration Strategies

Because `bridle-ctl` is a Rust binary, tools written in other languages must cross the language boundary. There are two primary methods to do this:

### 1. FFI (Foreign Function Interface) - Recommended for Determinism

`bridle-ctl` heavily utilizes `unsafe extern "C"` to call directly into compiled logic from specialized libraries. This provides deterministic, high-performance, and hallucination-free edits.

If your tool is written in **C, C++, Go, or Java (via GraalVM Native Image)**, you should compile it to a static or shared C-ABI library (`.a`, `.so`, `.dylib`) and call it via FFI.

#### Requirements for the External Tool:
You must expose your `audit` and `fix` functionality as C-compatible functions.

**C / C++ Example:**
```c
// include/my_tool.h
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

    int my_tool_audit(const char* target_path);
    int my_tool_fix(const char* target_path, bool dry_run);

#ifdef __cplusplus
}
#endif
```

**Go Example:**
```go
package main

import "C"
import "fmt"

//export MyToolAudit
func MyToolAudit(targetPath *C.char) C.int {
    path := C.GoString(targetPath)
    fmt.Printf("Auditing %s\n", path)
    return 0 // 0 for success
}

func main() {} // Required for buildmode=c-shared or c-archive
```
*(Compile with: `go build -buildmode=c-archive -o libmytool.a`)*

#### Wiring it into Bridle:
1. Update `bridle-sdk/src/ffi.rs` (or `registry.rs`) to declare the `extern "C"` functions.
2. Update `bridle-sdk/build.rs` to link the compiled library using `cc` or by setting `cargo:rustc-link-search`.
3. Implement `CodeTool` in `bridle-cli/src/tools/registry.rs` that wraps your `unsafe` FFI calls and returns the resulting `Result<String, CliError>`.

---

### 2. Subprocess Execution (CLI Wrapper)

If your tool is written in a language that requires a runtime like **TypeScript (Node.js)** or **Python**, or if setting up FFI is too complex, you can write your tool as a standalone CLI executable and wrap it via a subprocess inside `bridle-ctl`. 

#### Requirements for the External Tool:
Your tool must be invocable via the command line and accept standard arguments for auditing and fixing files.

Example CLI usage:
```bash
my-ts-tool --audit /path/to/target.ts
my-ts-tool --fix /path/to/target.ts --dry-run
```

#### Wiring it into Bridle:
In `bridle-cli/src/tools/registry.rs`, implement `CodeTool` using `std::process::Command` to invoke your CLI:

```rust
use std::process::Command;
use crate::error::CliError;

struct MyTsTool;

impl CodeTool for MyTsTool {
    fn name(&self) -> &'static str { "my-ts-tool" }
    fn description(&self) -> &'static str { "Typescript formatting/fixing tool" }
    fn match_regex(&self) -> &'static str { r".*\.ts$" }

    fn audit(&self, args: &[String]) -> Result<String, CliError> {
        let output = Command::new("node")
            .arg("path/to/my-ts-tool.js")
            .arg("--audit")
            .args(args)
            .output()
            .map_err(|e| CliError::ExecutionError(e.to_string()))?;
            
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn fix(&self, args: &[String], dry_run: bool) -> Result<String, CliError> {
        let mut cmd = Command::new("node");
        cmd.arg("path/to/my-ts-tool.js").arg("--fix").args(args);
        
        if dry_run {
            cmd.arg("--dry-run");
        }
        
        let output = cmd.output().map_err(|e| CliError::ExecutionError(e.to_string()))?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
```

## Step-by-Step Integration Checklist

1. **Write the Tool:** Develop the tool's core logic in its native language. Ensure it deterministically audits and fixes the files it processes.
2. **Expose the Interface:** 
   - Compile as a C-archive/shared object exposing standard C symbols (`extern "C"`) if taking the FFI route.
   - *OR* ensure it behaves as a predictable command-line utility taking standard flags (`--audit`, `--fix`, `--dry-run`).
3. **Register in Bridle:** Open `bridle-cli/src/tools/registry.rs` and create a struct that implements the `CodeTool` trait.
4. **Append to `get_tools()`:** Add your initialized struct to the `get_tools()` vector function at the bottom of `registry.rs` so that the CLI and Agent can dynamically discover and execute it.