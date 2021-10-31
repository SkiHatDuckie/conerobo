// Text user interface backend stuff
use std::{
    sync::mpsc,
    time::{Duration, Instant},
    thread,
};
use crossterm::{
    event::{self, Event as CEvent, KeyEvent},
    terminal::{enable_raw_mode, disable_raw_mode},
    Result,
};


// Data structure for input events
pub enum Event<I> {
    Input(I),
    Tick,
}

// Enable raw mode so that we don't have to wait for the 
// enter key to be pressed to read input
pub fn enable_raw() {
    enable_raw_mode().expect("can run in raw mode");
}

// Diable raw mode
pub fn disable_raw() -> Result<()> {
    disable_raw_mode()?;

    Ok(())
}

// handle input in seperate thread
pub fn handle_input(sender: mpsc::Sender<Event<KeyEvent>>) {
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    sender.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = sender.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });
}
