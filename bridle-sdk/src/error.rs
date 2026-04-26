#![cfg(not(tarpaulin_include))]
//! Error types for the Bridle SDK.

use derive_more::derive::{Display, Error};

/// Comprehensive error type for the Bridle SDK.
#[derive(Debug, Display, Error)]
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
    Migration(String),

    /// A telemetry initialization error.
    #[display("Telemetry error: {}", _0)]
    #[error(ignore)]
    Telemetry(String),

    /// A configuration error.
    #[display("Configuration error: {}", _0)]
    #[error(ignore)]
    Config(String),

    /// A generic error used as a fallback.
    #[display("Generic error: {}", _0)]
    #[error(ignore)]
    Generic(String),
}

impl From<std::io::Error> for BridleError {
    fn from(err: std::io::Error) -> Self {
        BridleError::Io(err)
    }
}

impl From<diesel::ConnectionError> for BridleError {
    fn from(err: diesel::ConnectionError) -> Self {
        BridleError::Connection(err)
    }
}

impl From<diesel::result::Error> for BridleError {
    fn from(err: diesel::result::Error) -> Self {
        BridleError::Database(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    }
}
