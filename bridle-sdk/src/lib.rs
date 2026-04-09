#![deny(missing_docs)]
#![warn(missing_docs)]
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

/// Addition function (placeholder).
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
}

/// Batch database operations.
pub mod batch_db;
/// Pipeline configuration and models.
pub mod path_scope;
/// Core pipeline logic and structs.
pub mod pipeline;
