use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{self, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};

use std::io::{stdout, Write};

pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw-mode");
        stdout()
            .execute(cursor::Show)
            .expect("Unable to show cursor");
        stdout()
            .queue(terminal::Clear(terminal::ClearType::All))
            .expect("Unable to show cursor")
            .queue(cursor::MoveTo(0, 0))
            .expect("Unable to reset the cursor")
            .queue(style::ResetColor)
            .expect("Unable to reset color");
    }
}
