//! Error definitions for the Agent interface.

use derive_more::{Display, Error, From};

/// Defines all possible errors that can occur within the agent.
#[derive(Debug, Display, Error, From)]
pub enum AgentError {
    /// An input/output error occurred.
    #[display("IO Error: {}", _0)]
    Io(std::io::Error),
    /// An internal daemon error occurred.
    #[display("Daemon Error: {}", _0)]
    #[error(ignore)]
    Daemon(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let io_err = AgentError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
        assert_eq!(io_err.to_string(), "IO Error: file not found");

        let daemon_err = AgentError::Daemon("timeout".to_string());
        assert_eq!(daemon_err.to_string(), "Daemon Error: timeout");
    }
}
