#![deny(missing_docs)]
#![warn(missing_docs)]
//! Main entry point for the bridle-agent executable.

use bridle_agent::{generate_claude_manifest, start_agent};
use bridle_sdk::BridleError;

/// Runs the agent CLI startup routine.
pub fn run_agent_main() -> Result<String, BridleError> {
    if let Err(e) = bridle_sdk::telemetry::init_telemetry() {
        eprintln!("Warning: Failed to initialize telemetry: {}", e);
    }

    let manifest = generate_claude_manifest();
    println!("{}", manifest);
    let status = start_agent()?;
    println!("{}", status);
    Ok(status.to_string())
}

/// Main entry point for the `bridle-agent` binary.
fn main() -> Result<(), BridleError> {
    let _ = run_agent_main()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_exec() -> Result<(), BridleError> {
        let status = run_agent_main()?;
        assert_eq!(status, "Agent started");
        main()?;
        Ok(())
    }
}
