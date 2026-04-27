//! Error types for the Bridle workspace.

use derive_more::derive::{Display, Error, From};

/// Comprehensive error type for the Bridle workspace.
#[derive(Debug, Display, Error, From)]
pub enum BridleError {
    /// A generic I/O error.
    #[display("I/O error: {}", _0)]
    Io(std::io::Error),

    /// A database connection error.
    #[display("Database connection error: {}", _0)]
    Connection(diesel::ConnectionError),

    /// A database execution error.
    #[display("Database error: {}", _0)]
    Database(diesel::result::Error),

    /// A migration execution error.
    #[display("Migration error: {}", _0)]
    #[error(ignore)]
    #[from(ignore)]
    Migration(String),

    /// A telemetry initialization error.
    #[display("Telemetry error: {}", _0)]
    #[error(ignore)]
    #[from(ignore)]
    Telemetry(String),

    /// A configuration error.
    #[display("Configuration error: {}", _0)]
    #[error(ignore)]
    #[from(ignore)]
    Config(String),

    /// A specific database is required but not available.
    #[display("Database not available: {}", _0)]
    #[error(ignore)]
    #[from(ignore)]
    DatabaseNotAvailable(String),

    /// HTTP Request Error
    #[display("Request Error: {}", _0)]
    Request(reqwest::Error),

    /// JSON serialization/deserialization error
    #[display("JSON Error: {}", _0)]
    Json(serde_json::Error),

    /// Template parsing error from indicatif
    #[display("Template Error: {}", _0)]
    Template(indicatif::style::TemplateError),

    /// A tool execution error string
    #[display("Tool Execution Error: {}", _0)]
    #[error(ignore)]
    #[from(ignore)]
    Tool(String),

    /// CLI parsing error
    #[display("CLI Error: {}", _0)]
    Clap(clap::error::Error),

    /// An internal daemon error occurred.
    #[display("Daemon Error: {}", _0)]
    #[error(ignore)]
    #[from(ignore)]
    Daemon(String),

    /// JSON RPC method registration error.
    #[display("JsonRpsee Register Error: {}", _0)]
    #[error(ignore)]
    #[from(ignore)]
    Register(String),

    /// JSON RPC client error.
    #[display("JsonRpsee Client Error: {}", _0)]
    Client(jsonrpsee::core::client::Error),

    /// Bridle SDK error mapping
    #[display("SDK FFI Error: {}", _0)]
    SdkFfi(crate::ffi::FfiError),

    /// A generic error used as a fallback.
    #[display("Generic error: {}", _0)]
    #[error(ignore)]
    #[from(ignore)]
    Generic(String),
}

impl From<std::string::FromUtf8Error> for BridleError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        BridleError::Generic(err.to_string())
    }
}

impl From<&str> for BridleError {
    fn from(err: &str) -> Self {
        BridleError::Generic(err.to_string())
    }
}

impl From<BridleError> for jsonrpsee::types::error::ErrorObjectOwned {
    fn from(err: BridleError) -> Self {
        jsonrpsee::types::error::ErrorObject::owned(-32000, err.to_string(), None::<()>)
    }
}

impl actix_web::ResponseError for BridleError {
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
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_io_error_display() {
        let io_err = Error::new(ErrorKind::NotFound, "file not found");
        let bridle_err = BridleError::from(io_err);
        assert_eq!(format!("{}", bridle_err), "I/O error: file not found");
    }

    #[test]
    fn test_db_error_display() {
        let conn_err =
            BridleError::Connection(diesel::ConnectionError::BadConnection("bad".to_string()));
        assert_eq!(format!("{}", conn_err), "Database connection error: bad");

        let db_err = BridleError::Database(diesel::result::Error::NotFound);
        assert_eq!(format!("{}", db_err), "Database error: Record not found");

        let mig_err = BridleError::Migration("mig fail".to_string());
        assert_eq!(format!("{}", mig_err), "Migration error: mig fail");

        let tel_err = BridleError::Telemetry("init fail".to_string());
        assert_eq!(format!("{}", tel_err), "Telemetry error: init fail");
    }

    #[test]
    fn test_generic_error_display() {
        let err = BridleError::Generic("something broke".to_string());
        assert_eq!(format!("{}", err), "Generic error: something broke");

        let conf_err = BridleError::Config("bad conf".to_string());
        assert_eq!(format!("{}", conf_err), "Configuration error: bad conf");

        let db_na_err = BridleError::DatabaseNotAvailable("mysql".to_string());
        assert_eq!(format!("{}", db_na_err), "Database not available: mysql");

        let req_err = BridleError::Generic("request".to_string());
        assert_eq!(format!("{}", req_err), "Generic error: request");

        let tool_err = BridleError::Tool("failed".to_string());
        assert_eq!(format!("{}", tool_err), "Tool Execution Error: failed");

        let daemon_err = BridleError::Daemon("timeout".to_string());
        assert_eq!(format!("{}", daemon_err), "Daemon Error: timeout");

        let reg_err = BridleError::Register("failed".to_string());
        assert_eq!(format!("{}", reg_err), "JsonRpsee Register Error: failed");
    }

    #[test]
    fn test_from_traits() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "not found");
        let bridle_io = BridleError::from(io_err);
        assert!(matches!(bridle_io, BridleError::Io(_)));

        let conn_err = diesel::ConnectionError::BadConnection("bad".to_string());
        let bridle_conn = BridleError::from(conn_err);
        assert!(matches!(bridle_conn, BridleError::Connection(_)));

        let db_err = diesel::result::Error::NotFound;
        let bridle_db = BridleError::from(db_err);
        assert!(matches!(bridle_db, BridleError::Database(_)));

        let utf8_err = match String::from_utf8(vec![0, 159]) {
            Err(e) => e,
            Ok(_) => panic!("Expected UTF8 error"),
        };
        let err: BridleError = utf8_err.into();
        assert!(err.to_string().starts_with("Generic error:"));

        let str_err: BridleError = "some error".into();
        assert_eq!(str_err.to_string(), "Generic error: some error");
    }

    #[test]
    fn test_actix_and_rpc() {
        let err = BridleError::Generic("test".to_string());
        assert_eq!(
            err.status_code(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        assert!(err.error_response().status().is_server_error());

        let obj: jsonrpsee::types::error::ErrorObjectOwned = err.into();
        assert_eq!(obj.message(), "Generic error: test");
        assert_eq!(obj.code(), -32000);
    }
}
