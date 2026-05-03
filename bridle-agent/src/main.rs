#![deny(missing_docs)]
#![warn(missing_docs)]
//! Main entry point for the bridle-agent executable.

use bridle_agent::{generate_claude_manifest, start_agent};
use bridle_sdk::BridleError;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), BridleError> {
    if let Err(e) = bridle_sdk::telemetry::init_telemetry() {
        eprintln!("Warning: Failed to initialize telemetry: {}", e);
    }

    println!("{}", generate_claude_manifest());
    println!("{}", start_agent()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_exec() {
        unsafe {
            std::env::set_var("RUST_TEST_MODE", "1");
        }
        let _ = main();
        unsafe {
            std::env::remove_var("RUST_TEST_MODE");
        }
    }
}
