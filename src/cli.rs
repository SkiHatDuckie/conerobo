// command line interface
use std::io::{
    Write, stdout, Stdout
};
use crossterm::{
    QueueableCommand, terminal, Result, cursor,
    style::Print,
};


pub struct Cli {
    stdout: Stdout,
}

impl Cli {
    // create instance of cli
    pub fn init() -> Result<Cli> {
        let mut cli = Cli {
            stdout: stdout(),
        };

        cli.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 14))?;

        cli.stdout.flush()?;

        Ok(cli)
    }

    // write text to screen
    pub fn output(&mut self, s: &str) {
        self.stdout.queue(Print(s.to_owned() + "\n")).unwrap();
    }
}
