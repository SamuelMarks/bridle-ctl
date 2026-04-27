#![deny(missing_docs)]
#![warn(missing_docs)]
//! Core logic, models, and FFI bindings for bridle-ctl.

pub mod db;
pub mod encoding;
pub mod error;
pub mod ffi;
pub mod file_lock;
pub use bridle_models::models;
pub use bridle_models::schema;
pub mod telemetry;

pub use error::BridleError;

/// Batch database operations.
pub mod batch_db;
/// Pipeline configuration and models.
pub mod path_scope;
/// Core pipeline logic and structs.
pub mod pipeline;
