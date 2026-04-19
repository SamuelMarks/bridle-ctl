//! Error handling for the CLI.

use derive_more::{Display, Error, From};
use std::io;

/// Custom error type for the CLI.
#[derive(Debug, Display, Error, From)]
pub enum CliError {
    /// HTTP Request Error
    #[display("Request Error: {}", _0)]
    Request(reqwest::Error),
    /// IO Error
    #[display("IO Error: {}", _0)]
    Io(io::Error),

    /// JSON serialization/deserialization error
    #[display("JSON Error: {}", _0)]
    Json(serde_json::Error),

    /// Template parsing error from indicatif
    #[display("Template Error: {}", _0)]
    Template(indicatif::style::TemplateError),

    /// Bridle SDK error mapping
    #[display("SDK FFI Error: {}", _0)]
    Sdk(bridle_sdk::ffi::FfiError),

    /// A tool execution error string
    #[display("Tool Execution Error: {}", _0)]
    #[error(ignore)]
    #[from(ignore)]
    Tool(String),

    /// A generic execution error
    #[display("Execution Error: {}", _0)]
    #[error(ignore)]
    #[from(ignore)]
    Execution(String),
}

impl From<std::string::FromUtf8Error> for CliError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        CliError::Execution(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CliError::Execution("Something went wrong".to_string());
        assert_eq!(err.to_string(), "Execution Error: Something went wrong");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: CliError = io_err.into();
        assert!(err.to_string().starts_with("IO Error: file not found"));
    }

    #[test]
    fn test_from_json_error() {
        let json_err = match serde_json::from_str::<serde_json::Value>("{ invalid") {
            Err(e) => e,
            Ok(_) => panic!("Expected JSON error"),
        };
        let err: CliError = json_err.into();
        assert!(err.to_string().starts_with("JSON Error:"));
    }

    #[test]
    fn test_from_utf8_error() {
        let utf8_err = match String::from_utf8(vec![0, 159]) {
            Err(e) => e,
            Ok(_) => panic!("Expected UTF8 error"),
        };
        let err: CliError = utf8_err.into();
        assert!(err.to_string().starts_with("Execution Error:"));
    }

    #[test]
    fn test_source() {
        use std::error::Error;
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let cli_err = CliError::from(io_err);
        assert!(cli_err.source().is_some());

        let msg_err = CliError::Tool("failed".to_string());
        assert!(msg_err.source().is_none());

        let exec_err = CliError::Execution("failed".to_string());
        assert!(exec_err.source().is_none());
    }
}
