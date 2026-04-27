//! Plugin configuration handling.

use crate::error::CliError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Core tools configuration (e.g., bridle-tools.toml)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CoreConfig {
    /// A map of tool name to path of its detailed .toml configuration
    #[serde(default)]
    pub plugins: HashMap<String, String>,

    /// A map of tool name to enabled status
    #[serde(default)]
    pub enabled: HashMap<String, bool>,
}

/// A single tool's configuration (loaded from a separate .toml file)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDef {
    /// Description of the tool
    pub description: String,
    /// Regex to match files
    pub match_regex: String,
    /// Optional tool version
    #[serde(default)]
    pub version: Option<String>,
    /// Optional tool author
    #[serde(default)]
    pub author: Option<String>,
    /// Optional tool URL
    #[serde(default)]
    pub url: Option<String>,
    /// Optional tool license
    #[serde(default)]
    pub license: Option<String>,
    /// Specific configuration for dynamic tools
    #[serde(flatten)]
    pub dynamic: DynamicToolConfig,
}

/// The type of dynamic tool being configured
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DynamicToolConfig {
    /// A subprocess plugin
    #[serde(rename = "subprocess")]
    Subprocess {
        /// The executable path or command
        command: String,
        /// Optional environment variables
        #[serde(default)]
        env: HashMap<String, String>,
        /// Optional virtual environment awareness
        #[serde(default)]
        venv_aware: bool,
    },
    /// A dynamic library plugin
    #[serde(rename = "dlopen")]
    Dlopen {
        /// Path to the .so, .dll, or .dylib
        path: String,
        /// Command to build the library
        build_command: Option<String>,
    },
    /// A JSON-RPC over HTTP
    #[serde(rename = "jsonrpc")]
    JsonRpc {
        /// Endpoint URL
        endpoint: String,
        /// Command to launch the server
        launch_command: Option<String>,
    },
    /// A statically linked FFI wrapper defined in TOML
    #[serde(rename = "ffi")]
    Ffi {
        /// The FFI wrapper name to invoke
        wrapper: String,
        /// Optional subcommand (e.g. for cdd tools)
        #[serde(default)]
        subcommand: Option<String>,
    },
}

impl CoreConfig {
    /// Loads the configuration from a given file path. If the file doesn't exist, returns Default.
    pub fn load(path: &str) -> Result<Self, CliError> {
        if !Path::new(path).exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        let config: Self =
            toml::from_str(&content).map_err(|e| CliError::Execution(e.to_string()))?;
        Ok(config)
    }
}

impl PluginDef {
    /// Loads a plugin definition from a given file path.
    pub fn load(path: &str) -> Result<Self, CliError> {
        let content = fs::read_to_string(path)?;
        let def: Self = toml::from_str(&content).map_err(|e| CliError::Execution(e.to_string()))?;
        Ok(def)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_core_config_load() -> Result<(), CliError> {
        let path = "test_core_config.toml";
        let config_toml = r#"
        [plugins]
        "test-tool" = "test-tool.toml"

        [enabled]
        "test-tool" = true
        "#;
        fs::write(path, config_toml)?;
        let config = CoreConfig::load(path)?;
        assert_eq!(config.plugins.len(), 1);
        assert_eq!(config.plugins["test-tool"], "test-tool.toml");
        assert!(config.enabled["test-tool"]);
        fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn test_plugin_def_load() -> Result<(), CliError> {
        let path = "test_plugin_def.toml";
        let config_toml = r#"
        description = "Test tool"
        match_regex = ".*"
        type = "subprocess"
        command = "echo"
        "#;
        fs::write(path, config_toml)?;
        let def = PluginDef::load(path)?;
        assert_eq!(def.description, "Test tool");
        assert_eq!(def.match_regex, ".*");
        if let DynamicToolConfig::Subprocess { command, .. } = def.dynamic {
            assert_eq!(command, "echo");
        } else {
            panic!("Expected subprocess tool");
        }
        fs::remove_file(path)?;
        Ok(())
    }
}
