//! Telemetry and logging initialization for the bridle workspace.

use crate::BridleError;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Initializes structured JSON logging for the application.
///
/// This sets up `tracing_subscriber` to output JSON formatted logs to stdout.
/// It uses the `RUST_LOG` environment variable for filtering (defaults to `info`).
///
/// # Errors
/// Returns `BridleError::Telemetry` if the global logger cannot be initialized.
pub fn init_telemetry() -> Result<(), BridleError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_current_span(false)
        .with_span_list(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .try_init()
        .map_err(|e| BridleError::Telemetry(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_telemetry() {
        // Since try_init() works only once per process, we just ensure it either
        // succeeds or returns our mapped telemetry error.
        let result = init_telemetry();
        match result {
            Ok(_) => {}                          // First time init
            Err(BridleError::Telemetry(_)) => {} // Subsequent inits
            Err(_) => panic!("Unexpected error type"),
        }
    }
}
