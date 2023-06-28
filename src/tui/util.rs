use crossterm::terminal;

pub fn get_terminal_width() -> crossterm::Result<u16> {
    Ok(terminal::size()?.0)
}

#[allow(dead_code)]
pub fn get_terminal_height() -> crossterm::Result<u16> {
    Ok(terminal::size()?.1)
}