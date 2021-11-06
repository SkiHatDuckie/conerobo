// Main
mod tui_util;
pub mod parts;
pub mod lua2rust;

use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use tui_util::{
    app::{App, run_app},
    backend,
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    backend::enable_raw();

    // Setup tui Terminal with CrosstermBackend
    let stdout = io::stdout();
    let crossterm_backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(crossterm_backend)?;
    terminal.clear()?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
