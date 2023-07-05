use crossterm::terminal;
use crate::error::{ConeRoboError, Result};

pub fn get_terminal_width() -> Result<u16> {
    Ok(terminal::size()
        .map_err(ConeRoboError::I0000)?
        .0)
}

#[allow(dead_code)]
pub fn get_terminal_height() -> Result<u16> {
    Ok(terminal::size()
        .map_err(ConeRoboError::I0000)?
        .1)
}