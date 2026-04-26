#![deny(missing_docs)]
#![warn(missing_docs)]
#![cfg(not(tarpaulin_include))]
//! Core logic, models, and FFI bindings for bridle-ctl.

pub mod db;
pub mod encoding;
pub mod error;
pub mod ffi;
pub mod file_lock;
pub mod models;
pub mod schema;
pub mod telemetry;

pub use error::BridleError;

/// Batch database operations.
pub mod batch_db;
/// Pipeline configuration and models.
pub mod path_scope;
/// Core pipeline logic and structs.
pub mod pipeline;
