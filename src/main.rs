use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{self, Color, Colors},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::env;
use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;

struct CleanUp;

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

fn main() -> Result<()> {
    let _clean_up = CleanUp;

    // Take a directory given as an argument or default to current directory
    // let args: Vec<String> = env::args().collect();
    let mut current_path = PathBuf::new();

    // TODO: Fix in the case where we take an argument. For now, default to current directory
    current_path = env::current_dir().unwrap();
    if !current_path.is_dir() {
        println!("Not a directory");
    }

    // Setup
    terminal::enable_raw_mode().expect("Unable to enable raw mode");
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .expect("Unable to clear terminal")
        .execute(cursor::MoveTo(0, 0))
        .expect("Unable to move cursor");

    // Read the contents of the current path and print them
    for entry in fs::read_dir(current_path).unwrap() {
        // style::Print("{:?}", entry.unwrap().path());
        let mut line = entry.unwrap().file_name().into_string().unwrap();
        stdout.queue(style::Print(line))?;
        stdout.queue(cursor::MoveToNextLine(1))?;
    }

    stdout.queue(cursor::MoveToRow(0))?;

    stdout.flush()?;

    // Process inputs
    loop {
        let cursor_pos = cursor::position().unwrap();
        if let Event::Key(event) = event::read().expect("Failed to read line") {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => break,
                KeyEvent {
                    code: KeyCode::Char('j'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    stdout.execute(cursor::MoveDown(1))?;
                    highlight_line(cursor_pos.1, Color::Blue)?;
                }
                KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    stdout.execute(cursor::MoveUp(1))?;
                }
                _ => {
                    println!("{:?}\r", event)
                }
            }
        }
    }

    Ok(())
}

// Highlights the given row with the given color
fn highlight_line(row: u16, color: Color) -> Result<()> {
    // Reprint the line?
    let size = terminal::size().unwrap();

    for col in 0..size.1 {
        stdout().execute(style::SetBackgroundColor(color))?;
    }

    Ok(())
}
