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

    /// TOML deserialization error
    #[display("TOML Error: {}", _0)]
    TomlDe(toml::de::Error),

    /// Glob pattern compilation error
    #[display("Glob Error: {}", _0)]
    Glob(globset::Error),

    /// Regular expression compilation error
    #[display("Regex Error: {}", _0)]
    Regex(regex::Error),

    /// Path prefix stripping error
    #[display("StripPrefix Error: {}", _0)]
    StripPrefix(std::path::StripPrefixError),

    /// Interior NUL byte encountered when building a C string
    #[display("Nul Error: {}", _0)]
    NulError(std::ffi::NulError),

    /// Dynamic library loading error
    #[display("Library Loading Error: {}", _0)]
    LibLoading(libloading::Error),

    /// Socket address parsing error
    #[display("Address Parse Error: {}", _0)]
    AddrParse(std::net::AddrParseError),

    /// UTF-8 string conversion error from owned byte vector
    #[display("UTF-8 Conversion Error: {}", _0)]
    Utf8Conversion(std::string::FromUtf8Error),

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

    /// Failed to parse a C string.
    #[display("Invalid C String encountered: {}", _0)]
    InvalidString(std::str::Utf8Error),
}

impl From<BridleError> for jsonrpsee::types::error::ErrorObjectOwned {
    fn from(err: BridleError) -> Self {
        jsonrpsee::types::error::ErrorObject::owned(-32000, err.to_string(), None::<()>)
    }
}

impl actix_web::ResponseError for BridleError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            BridleError::Database(diesel::result::Error::NotFound) => {
                actix_web::http::StatusCode::NOT_FOUND
            }
            BridleError::Json(_)
            | BridleError::TomlDe(_)
            | BridleError::Config(_)
            | BridleError::Clap(_)
            | BridleError::Glob(_)
            | BridleError::Regex(_) => actix_web::http::StatusCode::BAD_REQUEST,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).body(self.to_string())
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
    fn test_domain_error_display() {
        let conf_err = BridleError::Config("bad conf".to_string());
        assert_eq!(format!("{}", conf_err), "Configuration error: bad conf");

        let db_na_err = BridleError::DatabaseNotAvailable("mysql".to_string());
        assert_eq!(format!("{}", db_na_err), "Database not available: mysql");

        let tool_err = BridleError::Tool("failed".to_string());
        assert_eq!(format!("{}", tool_err), "Tool Execution Error: failed");

        let daemon_err = BridleError::Daemon("timeout".to_string());
        assert_eq!(format!("{}", daemon_err), "Daemon Error: timeout");

        let reg_err = BridleError::Register("failed".to_string());
        assert_eq!(format!("{}", reg_err), "JsonRpsee Register Error: failed");
    }

    #[test]
    #[allow(invalid_from_utf8)]
    fn test_all_variants_display_and_from() {
        let bridle_req = reqwest::Client::new()
            .get("htt p:/")
            .build()
            .err()
            .map(BridleError::from);
        assert!(
            bridle_req
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("Request Error:"))
        );

        let bridle_json = serde_json::from_str::<serde_json::Value>("{bad")
            .err()
            .map(BridleError::from);
        assert!(
            bridle_json
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("JSON Error:"))
        );

        let bridle_toml = toml::from_str::<serde_json::Value>("=invalid")
            .err()
            .map(BridleError::from);
        assert!(
            bridle_toml
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("TOML Error:"))
        );

        let bridle_glob = globset::Glob::new("[invalid").err().map(BridleError::from);
        assert!(
            bridle_glob
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("Glob Error:"))
        );

        let bad_re_pat = String::from("[invalid");
        let bridle_regex = regex::Regex::new(&bad_re_pat).err().map(BridleError::from);
        assert!(
            bridle_regex
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("Regex Error:"))
        );

        let bridle_strip = std::path::Path::new("/a/b")
            .strip_prefix("/c/d")
            .err()
            .map(BridleError::from);
        assert!(
            bridle_strip
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("StripPrefix Error:"))
        );

        let bridle_nul = std::ffi::CString::new("a\0b").err().map(BridleError::from);
        assert!(
            bridle_nul
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("Nul Error:"))
        );

        let bridle_lib = unsafe { libloading::Library::new("nonexistent_lib.so") }
            .err()
            .map(BridleError::from);
        assert!(
            bridle_lib
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("Library Loading Error:"))
        );

        let bridle_addr = "not_an_ip"
            .parse::<std::net::SocketAddr>()
            .err()
            .map(BridleError::from);
        assert!(
            bridle_addr
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("Address Parse Error:"))
        );

        let bridle_tpl = indicatif::ProgressStyle::with_template("{foo:^>")
            .err()
            .map(BridleError::from);
        assert!(
            bridle_tpl
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("Template Error:"))
        );

        let clap_err_inner =
            clap::Command::new("test").error(clap::error::ErrorKind::UnknownArgument, "err");
        let bridle_clap = BridleError::from(clap_err_inner);
        assert!(bridle_clap.to_string().starts_with("CLI Error:"));

        let client_err_inner = jsonrpsee::core::client::Error::Custom("custom".to_string());
        let bridle_client = BridleError::from(client_err_inner);
        assert!(
            bridle_client
                .to_string()
                .starts_with("JsonRpsee Client Error:")
        );

        let bridle_utf8 = std::str::from_utf8(&[0, 159]).err().map(BridleError::from);
        assert!(
            bridle_utf8
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("Invalid C String encountered:"))
        );

        let bridle_utf8_conv = String::from_utf8(vec![0, 159]).err().map(BridleError::from);
        assert!(
            bridle_utf8_conv
                .as_ref()
                .is_some_and(|e| e.to_string().starts_with("UTF-8 Conversion Error:"))
        );
    }

    #[test]
    fn test_actix_and_rpc() {
        let err = BridleError::Daemon("test".to_string());
        assert_eq!(
            err.status_code(),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        assert!(err.error_response().status().is_server_error());

        let not_found = BridleError::Database(diesel::result::Error::NotFound);
        assert_eq!(
            not_found.status_code(),
            actix_web::http::StatusCode::NOT_FOUND
        );
        assert_eq!(
            not_found.error_response().status(),
            actix_web::http::StatusCode::NOT_FOUND
        );

        let bad_req = BridleError::Config("bad".to_string());
        assert_eq!(
            bad_req.status_code(),
            actix_web::http::StatusCode::BAD_REQUEST
        );
        assert_eq!(
            bad_req.error_response().status(),
            actix_web::http::StatusCode::BAD_REQUEST
        );

        let obj: jsonrpsee::types::error::ErrorObjectOwned = err.into();
        assert_eq!(obj.message(), "Daemon Error: test");
        assert_eq!(obj.code(), -32000);
    }
}
