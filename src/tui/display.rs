use std::{
    cmp::max,
    collections::VecDeque
};
use crate::error::Result;
use super::util::get_terminal_width;

pub enum Title {
    Empty,
    Is(String)
}

pub struct MessageQueue(VecDeque<String>);
impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue(VecDeque::new())
    }
    pub fn push_msg(&mut self, msg: String) {
        self.0.push_back(msg);
    }
    /// Returns an empty string if queue is empty.
    pub fn pop_msg(&mut self) -> String {
        match self.0.pop_front() {
            Some(msg) => msg,
            None => String::new()
        }
    }
}

// Creates a string that can be used as a horizontal border.
pub fn create_border(ch: char, title: Title) -> Result<String> {
    let terminal_width = get_terminal_width()? as i32;
    let title_width = match title {
        Title::Empty => 0,
        Title::Is(ref val) => val.len() as i32
    };
    let mut top_border = match title {
        Title::Empty => String::new(),
        Title::Is(val) => val
    };
    top_border.push_str(
        ch
            .to_string()
            .repeat(max((terminal_width - title_width) as usize, 0))
            .as_str());

    Ok(top_border)
}