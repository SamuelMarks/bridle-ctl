use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::CodeTool;
    use bridle_sdk::path_scope::PathScope;

    #[test]
    fn test_subprocess_tool() -> Result<(), BridleError> {
        let mut envs = std::collections::HashMap::new();
        envs.insert("TEST_VAR".to_string(), "test_val".to_string());
        let tool = SubprocessTool::new(
            "test".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            Some("1.0".to_string()),
            Some("author".to_string()),
            Some("url".to_string()),
            Some("license".to_string()),
            "echo".to_string(),
            envs,
            false,
        );
        assert_eq!(tool.name(), "test");
        assert_eq!(tool.description(), "desc");
        assert_eq!(tool.match_regex(), ".*");
        assert_eq!(tool.version(), Some("1.0"));
        assert_eq!(tool.author(), Some("author"));
        assert_eq!(tool.url(), Some("url"));
        assert_eq!(tool.license(), Some("license"));

        let audit_res = tool.audit(&["audit_arg".to_string()], None)?;
        assert_eq!(audit_res, "audit audit_arg");

        let fix_res = tool.fix(&["fix_arg".to_string()], false, None)?;
        assert_eq!(fix_res, "fix fix_arg");

        let dry_res = tool.fix(&["fix_arg".to_string()], true, None)?;
        assert_eq!(dry_res, "fix --dry-run fix_arg");

        let bad_tool = SubprocessTool::new(
            "bad".to_string(),
            "bad".to_string(),
            ".*".to_string(),
            None, None, None, None,
            "nonexistent_command_12345".to_string(),
            std::collections::HashMap::new(),
            false,
        );
        assert!(bad_tool.audit(&[], None).is_err());
        assert!(bad_tool.fix(&[], false, None).is_err());

        let false_tool = SubprocessTool::new(
            "false".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            None, None, None, None,
            "false".to_string(),
            std::collections::HashMap::new(),
            false,
        );
        assert!(false_tool.audit(&[], None).is_err());
        assert!(false_tool.fix(&[], false, None).is_err());

        Ok(())
    }

    #[test]
    fn test_subprocess_tool_venv() {
        let tool = SubprocessTool::new(
            "venv_test".to_string(),
            "desc".to_string(),
            ".*".to_string(),
            None, None, None, None,
            "echo".to_string(),
            std::collections::HashMap::new(),
            true,
        );
        std::env::set_var("VIRTUAL_ENV", "/tmp/fake_venv");
        let _cmd = tool.configure_command("audit", &[], None);
        std::env::remove_var("VIRTUAL_ENV");
        
        let _cmd2 = tool.configure_command("audit", &[], None);
    }
}