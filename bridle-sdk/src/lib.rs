#![warn(missing_docs)]
//! Core logic, models, and FFI bindings for bridle-ctl.

pub mod db;
pub mod encoding;
pub mod error;
pub mod ffi;
pub mod file_lock;
pub mod models;
pub mod schema;

pub use error::BridleError;

/// Addition function (placeholder).
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
