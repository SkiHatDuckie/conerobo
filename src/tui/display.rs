use super::util::get_terminal_width;
use std::{
    cmp::max,
    io
};

pub enum Title {
    Empty,
    Is(String)
}

// Creates a string that can be used as a horizontal border.
pub fn create_border(ch: char, title: Title) -> io::Result<String> {
    let terminal_width = get_terminal_width()? as i32;
    let title_width = match title {
        Title::Empty => 0,
        Title::Is(ref val) => val.len() as i32
    };
    let mut top_border = ch
        .to_string()
        .repeat(max((terminal_width - title_width) as usize, 0));
    match title {
        Title::Empty => {},
        Title::Is(ref val) => top_border.push_str(format!("{}", val).as_str())
    };

    Ok(top_border)
}