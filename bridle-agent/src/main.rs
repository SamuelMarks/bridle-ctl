#![warn(missing_docs)]
//! Main entry point for the bridle-agent executable.

use bridle_agent::{error::AgentError, generate_claude_manifest, start_agent};

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), AgentError> {
    println!("{}", generate_claude_manifest());
    println!("{}", start_agent()?);
    Ok(())
}
