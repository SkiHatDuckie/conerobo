use crossterm::terminal;

use crate::error::Result;

// Ensures that raw mode is disabled in the scenario where the TUI crashes.
pub struct RawModeGuard;
impl RawModeGuard {
    pub fn new() -> Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(RawModeGuard)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}