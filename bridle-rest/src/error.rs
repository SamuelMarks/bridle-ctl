//! Error definitions for the REST API.

use derive_more::{Display, Error, From};

/// Defines all possible errors that can occur within the REST API.
#[derive(Debug, Display, Error, From)]
pub enum RestError {
    /// An input/output error occurred.
    #[display("IO Error: {}", _0)]
    Io(std::io::Error),
    /// An error from the Agent component.
    #[display("Agent Error: {}", _0)]
    Agent(bridle_agent::error::AgentError),
    /// An error from the CLI tool runner.
    #[display("CLI Error: {}", _0)]
    Cli(bridle_cli::error::CliError),
    /// An error from the Bridle SDK (e.g. database).
    #[display("SDK Error: {}", _0)]
    Sdk(bridle_sdk::BridleError),
}

impl actix_web::ResponseError for RestError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().body(self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::ResponseError;

    #[test]
    fn test_error_display() {
        let io_err = RestError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
        assert_eq!(io_err.to_string(), "IO Error: file not found");
        assert_eq!(
            io_err.status_code(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        assert!(io_err.error_response().status().is_server_error());

        let agent_err =
            RestError::Agent(bridle_agent::error::AgentError::Daemon("test".to_string()));
        assert_eq!(agent_err.to_string(), "Agent Error: Daemon Error: test");

        let cli_err = RestError::Cli(bridle_cli::error::CliError::Execution(
            "exec failed".to_string(),
        ));
        assert_eq!(
            cli_err.to_string(),
            "CLI Error: Execution Error: exec failed"
        );

        let sdk_err = RestError::Sdk(bridle_sdk::BridleError::Generic("db failed".to_string()));
        assert_eq!(sdk_err.to_string(), "SDK Error: Generic error: db failed");
    }
}
