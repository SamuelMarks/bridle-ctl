#![cfg(not(tarpaulin_include))]
#![cfg(not(tarpaulin_include))]
use super::CodeTool;
use bridle_sdk::BridleError;
use bridle_sdk::path_scope::PathScope;

/// A mock tool for testing Rust `unwrap` replacements.
struct MockRustTool;

impl CodeTool for MockRustTool {
    fn name(&self) -> &str {
        "rust-unwrap-to-question-mark"
    }
    fn description(&self) -> &str {
        "Replaces unwrap with ? in Rust code"
    }
    fn match_regex(&self) -> &str {
        r".*\.rs$"
    }
    fn audit(&self, _args: &[String], _scope: Option<&PathScope>) -> Result<String, BridleError> {
        Ok("Found 3 instances".to_string())
    }
    fn fix(
        &self,
        _args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, BridleError> {
        if dry_run {
            Ok("[DRY RUN] Would replace 3 instances".to_string())
        } else {
            Ok("Replaced 3 instances".to_string())
        }
    }
}

/// A mock tool for testing GitHub Actions workflows.
struct GithubActionsTool;

impl CodeTool for GithubActionsTool {
    fn name(&self) -> &str {
        "gha-improver"
    }
    fn description(&self) -> &str {
        "Improves GitHub Actions workflows"
    }
    fn match_regex(&self) -> &str {
        r"\.github/workflows/.*\.ya?ml$"
    }
    fn audit(&self, _args: &[String], _scope: Option<&PathScope>) -> Result<String, BridleError> {
        Ok("Found 2 workflow improvements".to_string())
    }
    fn fix(
        &self,
        _args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, BridleError> {
        if dry_run {
            Ok("[DRY RUN] Would apply 2 workflow improvements".to_string())
        } else {
            Ok("Applied 2 workflow improvements".to_string())
        }
    }
}

/// Tool for exclusive file modification testing file_lock
struct FileLockTesterTool;

impl CodeTool for FileLockTesterTool {
    fn name(&self) -> &str {
        "file-lock-tester"
    }
    fn description(&self) -> &str {
        "Tests exclusive file mutations"
    }
    fn match_regex(&self) -> &str {
        r".*\.txt$"
    }
    fn audit(&self, _args: &[String], _scope: Option<&PathScope>) -> Result<String, BridleError> {
        Ok("file-lock-tester audit executed".into())
    }
    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, BridleError> {
        if args.is_empty() {
            return Ok("No file provided".to_string());
        }
        let path = &args[0];
        if dry_run {
            return Ok(format!("[DRY RUN] Would mutate {}", path));
        }
        let modified =
            bridle_sdk::file_lock::mutate_file_exclusively(path, _scope, |contents: &str| {
                if contents.contains("LOCK_ME") {
                    Some(contents.replace("LOCK_ME", "LOCKED"))
                } else {
                    None
                }
            })
            .map_err(|e| BridleError::Generic(e.to_string()))?;
        if modified {
            Ok(format!("Exclusively mutated {}", path))
        } else {
            Ok(format!("No mutation needed for {}", path))
        }
    }
}

/// Tool for normalizing file encodings and line endings
struct EncodingNormalizerTool;

impl CodeTool for EncodingNormalizerTool {
    fn name(&self) -> &str {
        "encoding-normalizer"
    }
    fn description(&self) -> &str {
        "Normalizes file encodings and line endings"
    }
    fn match_regex(&self) -> &str {
        r".*\.txt$"
    }
    fn audit(&self, _args: &[String], _scope: Option<&PathScope>) -> Result<String, BridleError> {
        Ok("encoding-normalizer audit executed".into())
    }
    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, BridleError> {
        if args.is_empty() {
            return Ok("No file provided".to_string());
        }
        let path = &args[0];
        if dry_run {
            return Ok(format!("[DRY RUN] Would normalize {}", path));
        }
        let mut doc = bridle_sdk::encoding::read_file_with_encoding(path)
            .map_err(|e| BridleError::Generic(e.to_string()))?;
        doc.normalize_line_endings();
        bridle_sdk::encoding::write_file_with_encoding(path, &doc)
            .map_err(|e| BridleError::Generic(e.to_string()))?;
        Ok(format!("Normalized encoding for {}", path))
    }
}

/// Tool for migrating user structures in SQLite using the SDK
struct DBMigratorTool;

impl CodeTool for DBMigratorTool {
    fn name(&self) -> &str {
        "db-migrator-tester"
    }
    fn description(&self) -> &str {
        "Tests establishing DB connections and fetching a mock user via SDK"
    }
    fn match_regex(&self) -> &str {
        r".*\.sqlite3$"
    }
    fn audit(&self, _args: &[String], _scope: Option<&PathScope>) -> Result<String, BridleError> {
        Ok("db-migrator-tester audit executed".into())
    }
    fn fix(
        &self,
        args: &[String],
        dry_run: bool,
        _scope: Option<&PathScope>,
    ) -> Result<String, BridleError> {
        if args.is_empty() {
            return Ok("No database path provided".to_string());
        }
        let db_url = &args[0];
        if dry_run {
            return Ok(format!("[DRY RUN] Would connect to and migrate {}", db_url));
        }
        let mut _conn = bridle_sdk::db::establish_connection_and_run_migrations(db_url)
            .map_err(|e| BridleError::Generic(e.to_string()))?;
        if let Ok(user) = bridle_sdk::db::get_user(&mut _conn, 999) {
            return Ok(format!(
                "Connected, migrated, and fetched user: {}",
                user.username
            ));
        }
        if let Ok(user) = bridle_sdk::db::get_user(&mut _conn, 1) {
            return Ok(format!(
                "Connected, migrated, and fetched user: {}",
                user.username
            ));
        }
        Ok(format!("Connected and migrated {}", db_url))
    }
}

use crate::tools::config::{CoreConfig, DynamicToolConfig, PluginDef};
use crate::tools::dynamic::{DlopenTool, FfiTool, JsonRpcTool, SubprocessTool};

/// Gets all registered tools available in the codebase.
pub fn get_tools() -> Vec<Box<dyn CodeTool>> {
    let mut tools: Vec<Box<dyn CodeTool>> = vec![
        Box::new(MockRustTool),
        Box::new(GithubActionsTool),
        Box::new(FileLockTesterTool),
        Box::new(EncodingNormalizerTool),
        Box::new(DBMigratorTool),
    ];

    let config_path = if std::path::Path::new("bridle-tools.toml").exists() {
        "bridle-tools.toml"
    } else {
        "../bridle-tools.toml"
    };

    let config_res = CoreConfig::load(config_path);
    if let Ok(config) = config_res {
        tools.retain(|t| config.enabled.get(t.name()).copied().unwrap_or(true));

        for (name, path) in config.plugins {
            let actual_path = if std::path::Path::new(&path).exists() {
                path.clone()
            } else {
                format!("../{}", path)
            };
            if config.enabled.get(&name).copied().unwrap_or(true) {
                let load_res = PluginDef::load(&actual_path);
                if let Ok(def) = load_res {
                    match def.dynamic {
                        DynamicToolConfig::Subprocess {
                            command,
                            env,
                            venv_aware,
                        } => {
                            tools.push(Box::new(SubprocessTool::new(
                                name,
                                def.description,
                                def.match_regex,
                                def.version,
                                def.author,
                                def.url,
                                def.license,
                                command,
                                env,
                                venv_aware,
                            )));
                        }
                        DynamicToolConfig::JsonRpc {
                            endpoint,
                            launch_command: _,
                        } => {
                            tools.push(Box::new(JsonRpcTool::new(
                                name,
                                def.description,
                                def.match_regex,
                                def.version,
                                def.author,
                                def.url,
                                def.license,
                                endpoint,
                            )));
                        }
                        DynamicToolConfig::Dlopen {
                            path,
                            build_command: _,
                        } => {
                            if let Ok(dt) = DlopenTool::new(
                                name,
                                def.description,
                                def.match_regex,
                                def.version,
                                def.author,
                                def.url,
                                def.license,
                                &path,
                            ) {
                                tools.push(Box::new(dt));
                            }
                        }
                        DynamicToolConfig::Ffi {
                            wrapper,
                            subcommand,
                        } => {
                            tools.push(Box::new(FfiTool::new(
                                name,
                                def.description,
                                def.match_regex,
                                def.version,
                                def.author,
                                def.url,
                                def.license,
                                wrapper,
                                subcommand,
                            )));
                        }
                    }
                }
            }
        }
    }

    tools
}

/// Gets tools filtered by matching their target pattern exact string.
pub fn get_tools_for_pattern(pattern: &str) -> Vec<Box<dyn CodeTool>> {
    let resolved_pattern = match pattern.to_lowercase().as_str() {
        "rust" => r".*\.rs$",
        "python" => r".*\.py$",
        "go" => r".*\.go$",
        "c" => r".*\.[ch]$",
        "cpp" | "cxx" | "c++" => r".*\.(cpp|hpp|cxx|hxx|cc|h)$",
        "yaml" | "yml" | "github-actions" | "gha" => r"\.github/workflows/.*\.ya?ml$",
        _ => pattern,
    };
    get_tools()
        .into_iter()
        .filter(|t| t.match_regex() == resolved_pattern || t.match_regex() == pattern)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_rust_tool() -> Result<(), BridleError> {
        let tool = MockRustTool;
        assert_eq!(tool.name(), "rust-unwrap-to-question-mark");
        assert_eq!(tool.match_regex(), r".*\.rs$");
        assert!(tool.audit(&[], None)?.contains("3 instances"));
        assert!(tool.fix(&[], false, None)?.contains("Replaced 3 instances"));
        assert!(tool.fix(&[], true, None)?.contains("[DRY RUN]"));
        Ok(())
    }

    #[test]
    fn test_mock_gha_tool() -> Result<(), BridleError> {
        let tool = GithubActionsTool;
        assert_eq!(tool.name(), "gha-improver");
        assert!(tool.audit(&[], None)?.contains("2 workflow"));
        assert!(tool.fix(&[], false, None)?.contains("Applied 2 workflow"));
        assert!(tool.fix(&[], true, None)?.contains("[DRY RUN]"));
        Ok(())
    }

    #[test]
    fn test_ffi_tools() -> Result<(), BridleError> {
        let tools = get_tools();

        let tc = tools
            .iter()
            .find(|t| t.name() == "type-correct")
            .ok_or("err")?;
        assert_eq!(
            tc.description(),
            "Resolves standard C/C++ type inconsistencies via AST parsing."
        );
        assert_eq!(tc.match_regex(), r".*\.(c|cpp|h|hpp)$");
        // Test with empty args (should fail)
        assert!(tc.audit(&[], None).is_err());
        assert!(tc.fix(&[], false, None).is_err());

        let args = vec!["nonexistent_file.cpp".to_string()];
        assert!(tc.audit(&args, None).is_err());

        let ge = tools
            .iter()
            .find(|t| t.name() == "go-auto-err-handling")
            .ok_or("err")?;
        assert_eq!(
            ge.description(),
            "Automatically injects `if err != nil { return err }` blocks."
        );
        assert_eq!(ge.match_regex(), r".*\.go$");

        let tmp_dir = tempfile::tempdir()?;
        let original_dir = std::env::current_dir().unwrap_or_default();
        std::env::set_current_dir(tmp_dir.path())?;
        let _ = std::process::Command::new("git").arg("init").status();

        let res_audit = ge.audit(&[".".to_string()], None);
        let res_audit_no_arg = ge.audit(&[], None);
        let res_fix = ge.fix(&[".".to_string()], false, None);
        let res_fix_dry = ge.fix(&[".".to_string()], true, None);

        let invalid_c_string = String::from_utf8(vec![0x61, 0x00, 0x62])?; // "a\0b"
        let bad_audit_res = ge.audit(std::slice::from_ref(&invalid_c_string), None);
        let bad_fix_res = ge.fix(std::slice::from_ref(&invalid_c_string), false, None);

        let tmp_file = format!("test_{}.go", uuid::Uuid::new_v4());
        let tmp_file_str = tmp_file.as_str();
        std::fs::write(
            tmp_file_str,
            "package main\nfunc fail() error { return nil }\nfunc main() { fail() }\n",
        )?;
        let audit_fail_res = ge.audit(std::slice::from_ref(&tmp_file), None);

        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(tmp_file_str)?.permissions();
        perms.set_mode(0o400);
        std::fs::set_permissions(tmp_file_str, perms)?;

        std::fs::remove_file(tmp_file_str).unwrap_or_default();

        let _ = std::env::set_current_dir(original_dir);

        assert!(res_audit?.contains("No issues found for ."));
        assert!(res_audit_no_arg?.contains("No issues found for ."));
        assert!(res_fix?.contains("fix applied successfully"));
        assert!(res_fix_dry?.contains("[DRY RUN]"));
        assert!(bad_audit_res.is_err());
        assert!(bad_fix_res.is_err());
        assert!(audit_fail_res?.contains("Issues found for"));

        let l2n = tools
            .iter()
            .find(|t| t.name() == "lib2notebook2lib")
            .ok_or("err")?;
        assert_eq!(
            l2n.description(),
            "Bi-directional sync between Python source and Jupyter notebooks."
        );
        assert_eq!(l2n.match_regex(), r".*\.(py|ipynb)$");

        unsafe {
            std::env::set_var("RUST_TEST_MODE", "1");
        }
        assert_eq!(l2n.audit(&[], None)?, "lib2notebook2lib audit executed");
        assert_eq!(l2n.fix(&[], false, None)?, "lib2notebook2lib fix applied");
        assert_eq!(
            l2n.fix(&[], true, None)?,
            "[DRY RUN] lib2notebook2lib fix planned"
        );
        unsafe {
            std::env::remove_var("RUST_TEST_MODE");
        }

        Ok(())
    }

    #[test]
    fn test_file_lock_tester_tool() -> Result<(), BridleError> {
        let tool = FileLockTesterTool;
        assert_eq!(tool.name(), "file-lock-tester");
        assert_eq!(tool.description(), "Tests exclusive file mutations");
        assert_eq!(tool.match_regex(), r".*\.txt$");
        assert_eq!(tool.audit(&[], None)?, "file-lock-tester audit executed");
        assert_eq!(tool.fix(&[], false, None)?, "No file provided");
        assert_eq!(
            tool.fix(&["test.txt".to_string()], true, None)?,
            "[DRY RUN] Would mutate test.txt"
        );

        let tmp_dir = tempfile::tempdir()?;
        let tmp_file_path = tmp_dir.path().join("test_lock.txt");
        let tmp = tmp_file_path.to_string_lossy().to_string();

        std::fs::write(&tmp_file_path, "hello LOCK_ME world")?;
        let res = tool.fix(std::slice::from_ref(&tmp), false, None)?;
        assert!(res.contains("Exclusively mutated"));
        let res2 = tool.fix(std::slice::from_ref(&tmp), false, None)?;
        assert!(res2.contains("No mutation needed"));
        std::fs::remove_file(&tmp_file_path)?;

        Ok(())
    }

    #[test]
    fn test_encoding_normalizer_tool() -> Result<(), BridleError> {
        let tool = EncodingNormalizerTool;
        assert_eq!(tool.name(), "encoding-normalizer");
        assert_eq!(
            tool.description(),
            "Normalizes file encodings and line endings"
        );
        assert_eq!(tool.match_regex(), r".*\.txt$");
        assert_eq!(tool.audit(&[], None)?, "encoding-normalizer audit executed");
        assert_eq!(tool.fix(&[], false, None)?, "No file provided");
        assert_eq!(
            tool.fix(&["test.txt".to_string()], true, None)?,
            "[DRY RUN] Would normalize test.txt"
        );

        let tmp_dir = tempfile::tempdir()?;
        let tmp_file_path = tmp_dir.path().join("test_enc.txt");
        let tmp = tmp_file_path.to_string_lossy().to_string();

        std::fs::write(&tmp_file_path, "hello\r\nworld")?;
        let res = tool.fix(std::slice::from_ref(&tmp), false, None)?;
        assert!(res.contains("Normalized"));
        std::fs::remove_file(&tmp_file_path)?;

        Ok(())
    }

    #[test]
    fn test_db_migrator_tool() -> Result<(), BridleError> {
        let tool = DBMigratorTool;
        assert_eq!(tool.name(), "db-migrator-tester");
        assert_eq!(
            tool.description(),
            "Tests establishing DB connections and fetching a mock user via SDK"
        );
        assert_eq!(tool.match_regex(), r".*\.sqlite3$");
        assert_eq!(tool.audit(&[], None)?, "db-migrator-tester audit executed");
        assert_eq!(tool.fix(&[], false, None)?, "No database path provided");
        assert_eq!(
            tool.fix(&["test.sqlite3".to_string()], true, None)?,
            "[DRY RUN] Would connect to and migrate test.sqlite3"
        );

        let tf = tempfile::NamedTempFile::new()?;
        let tmp = tf.path().to_str().ok_or("Invalid path")?;

        let res = tool.fix(&[tmp.to_string()], false, None)?;
        assert!(res.contains("Connected"));

        let mut conn = bridle_sdk::db::establish_connection_and_run_migrations(tmp)?;
        let now = chrono::Utc::now().naive_utc();
        let new_user = bridle_sdk::models::User {
            id: 999,
            username: "testuser999".into(),
            email: "test999@example.com".into(),
            password_hash: "hash".into(),
            avatar_url: None,
            bio: None,
            status: None,
            created_at: now,
            updated_at: now,
        };
        bridle_sdk::db::insert_user(&mut conn, &new_user)?;

        let res2 = tool.fix(&[tmp.to_string()], false, None)?;
        assert!(res2.contains("fetched user: testuser999"));

        Ok(())
    }

    #[test]
    fn test_get_tools() {
        let tools = get_tools();
        assert!(tools.len() >= 8);
    }

    #[test]
    fn test_get_tools_for_pattern() {
        let rust_tools = get_tools_for_pattern(r".*\.rs$");
        assert_eq!(rust_tools.len(), 1);
        assert_eq!(rust_tools[0].name(), "rust-unwrap-to-question-mark");

        let rust_tools_alias = get_tools_for_pattern("rust");
        assert_eq!(rust_tools_alias.len(), 1);
        assert_eq!(rust_tools_alias[0].name(), "rust-unwrap-to-question-mark");

        let py_tools = get_tools_for_pattern("python");
        assert_eq!(py_tools.len(), 1);

        let l2n_tools = get_tools_for_pattern(r".*\.(py|ipynb)$");
        assert_eq!(l2n_tools.len(), 1);
        assert_eq!(l2n_tools[0].name(), "lib2notebook2lib");

        let unknown_tools = get_tools_for_pattern("non-existent-pattern");
        assert!(unknown_tools.is_empty());

        let go_tools = get_tools_for_pattern("go");
        assert_eq!(go_tools.len(), 1);

        let gha_tools = get_tools_for_pattern("gha");
        assert_eq!(gha_tools.len(), 1);

        let exact_tools = get_tools_for_pattern(r".*\.txt$");
        assert_eq!(exact_tools.len(), 2);
    }
}
