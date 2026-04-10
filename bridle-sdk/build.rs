//! Build script for `bridle-sdk`.

use std::env;
use std::path::Path;
use std::process::Command;

/// Sets up the FFI bindings and paths for external dependencies.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let out_path = Path::new(&out_dir);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    // --- go-auto-err-handling ---
    let local_repo = Path::new(&manifest_dir).join("../../go-auto-err-handling");

    let repo_dir = if local_repo.exists() {
        local_repo
    } else {
        let git_dir = out_path.join("go-auto-err-handling");
        if !git_dir.exists() {
            let status = Command::new("git")
                .env_remove("GIT_DIR")
                .env_remove("GIT_WORK_TREE")
                .env_remove("GIT_INDEX_FILE")
                .arg("clone")
                .arg("--depth=1")
                .arg("https://github.com/SamuelMarks/go-auto-err-handling.git")
                .arg(&git_dir)
                .status()
                .expect("Failed to run git clone");
            if !status.success() {
                panic!("Failed to clone github.com/SamuelMarks/go-auto-err-handling");
            }
        }
        git_dir
    };

    println!("cargo:rerun-if-changed={}", repo_dir.display());

    let status = Command::new("go")
        .current_dir(&repo_dir)
        .env("CGO_ENABLED", "1")
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(out_path.join("libgoautoerr.a"))
        .arg("./cmd/ffi/main.go")
        .status()
        .expect("Failed to execute go build");

    assert!(status.success(), "Go build failed");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=goautoerr");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }

    // --- type-correct ---
    let type_correct_local = Path::new(&manifest_dir).join("../../type-correct");
    let type_correct_repo = if type_correct_local.exists() {
        type_correct_local
    } else {
        let git_dir = out_path.join("type-correct");
        if !git_dir.exists() {
            let status = Command::new("git")
                .env_remove("GIT_DIR")
                .env_remove("GIT_WORK_TREE")
                .env_remove("GIT_INDEX_FILE")
                .arg("clone")
                .arg("--depth=1")
                .arg("https://github.com/SamuelMarks/type-correct.git")
                .arg(&git_dir)
                .status()
                .expect("Failed to run git clone type-correct");
            assert!(status.success(), "Failed to clone type-correct");
        }
        git_dir
    };

    println!("cargo:rerun-if-changed={}", type_correct_repo.display());

    let llvm_config_bin = if Path::new("/opt/homebrew/opt/llvm/bin/llvm-config").exists() {
        "/opt/homebrew/opt/llvm/bin/llvm-config"
    } else {
        "llvm-config"
    };

    let llvm_config_out = Command::new(llvm_config_bin)
        .arg("--prefix")
        .output()
        .expect("Failed to run llvm-config");
    let llvm_prefix = String::from_utf8(llvm_config_out.stdout)
        .expect("llvm-config output was not valid UTF-8")
        .trim()
        .to_string();

    let dst = cmake::Config::new(&type_correct_repo)
        .define("CT_Clang_INSTALL_DIR", &llvm_prefix)
        .define("BUILD_TESTING", "OFF")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=type_correct");

    // Add rpath so the binary can find the dylib at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}/lib", dst.display());
    // --- cdd-c stub ---

    println!("cargo:rerun-if-changed=src/cdd_stub.c");
    cc::Build::new().file("src/cdd_stub.c").compile("cdd_stub");
}
