#![cfg(not(tarpaulin_include))]
#![deny(missing_docs)]
#![warn(missing_docs)]
//! Main entry point for the bridle-agent executable.

use bridle_agent::{generate_claude_manifest, start_agent};
use bridle_sdk::BridleError;

fn main() -> Result<(), BridleError> {
    if let Err(e) = bridle_sdk::telemetry::init_telemetry() {
        eprintln!("Warning: Failed to initialize telemetry: {}", e);
    }

    println!("{}", generate_claude_manifest());
    println!("{}", start_agent()?);
    Ok(())
}
