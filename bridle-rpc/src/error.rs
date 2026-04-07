//! Error definitions for the JSON RPC API.

use derive_more::{Display, Error, From};

/// Defines all possible errors that can occur within the RPC server.
#[derive(Debug, Display, Error, From)]
pub enum RpcError {
    /// An input/output error occurred.
    #[display("IO Error: {}", _0)]
    Io(std::io::Error),
    /// An error from the Agent component.
    #[display("Agent Error: {}", _0)]
    Agent(bridle_agent::error::AgentError),
    /// JSON RPC method registration error.
    #[display("JsonRpsee Register Error: {}", _0)]
    #[error(ignore)]
    Register(String),
    /// JSON RPC client error.
    #[display("JsonRpsee Client Error: {}", _0)]
    Client(jsonrpsee::core::client::Error),
    /// An error from the CLI tool runner.
    #[display("CLI Error: {}", _0)]
    Cli(bridle_cli::error::CliError),
    /// An error from the Bridle SDK (e.g. database).
    #[display("SDK Error: {}", _0)]
    Sdk(bridle_sdk::BridleError),
}

impl From<RpcError> for jsonrpsee::types::error::ErrorObjectOwned {
    fn from(err: RpcError) -> Self {
        jsonrpsee::types::error::ErrorObject::owned(-32000, err.to_string(), None::<()>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let io_err = RpcError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
        assert_eq!(io_err.to_string(), "IO Error: file not found");

        let agent_err =
            RpcError::Agent(bridle_agent::error::AgentError::Daemon("test".to_string()));
        assert_eq!(agent_err.to_string(), "Agent Error: Daemon Error: test");

        let reg_err = RpcError::Register("failed".to_string());
        assert_eq!(reg_err.to_string(), "JsonRpsee Register Error: failed");

        let cli_err = RpcError::Cli(bridle_cli::error::CliError::Execution(
            "exec failed".to_string(),
        ));
        assert_eq!(
            cli_err.to_string(),
            "CLI Error: Execution Error: exec failed"
        );

        let sdk_err = RpcError::Sdk(bridle_sdk::BridleError::Generic("db failed".to_string()));
        assert_eq!(sdk_err.to_string(), "SDK Error: Generic error: db failed");
    }

    #[test]
    fn test_error_conversion() {
        let err = RpcError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
        let obj: jsonrpsee::types::error::ErrorObjectOwned = err.into();
        assert_eq!(obj.message(), "IO Error: file not found");
        assert_eq!(obj.code(), -32000);
    }
}
