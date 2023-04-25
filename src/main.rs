use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{self, Attribute, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

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

    // How to store files
    // Hashmap with each key being a directory and value being a vector of it's
    // contents
    // Read the contents of the current path and print them
    let mut dirs = HashMap::new();
    let mut cur_directory = Vec::new();
    for entry in WalkDir::new(&current_path).min_depth(1).max_depth(1) {
        println!("{}", entry?.path().display());
    }
    for entry in fs::read_dir(current_path).unwrap() {
        cur_directory.push(entry.as_ref().unwrap().file_name().into_string().unwrap());
        let name = entry.unwrap();
        if name.file_type()?.is_dir() {
            let mut child_entries = Vec::new();
            for child_entry in fs::read_dir(name.path()).unwrap() {
                child_entries.push(child_entry.unwrap());
            }
            dirs.insert(name.path(), child_entries);
        }
    }

    // // Print out cur_directory
    // for entry in &cur_directory {
    //     stdout.queue(style::Print(entry)).unwrap();
    //     stdout.queue(cursor::MoveToNextLine(1)).unwrap();
    // }

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
                }
                KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    stdout.execute(cursor::MoveUp(1))?;
                }
                KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    stdout.execute(cursor::MoveRight(1))?;
                }
                KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    stdout.execute(cursor::MoveLeft(1))?;
                }
                KeyEvent {
                    code: KeyCode::Char('t'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    // Grab a line
                    stdout.execute(cursor::MoveToColumn(0))?;
                    highlight_line(
                        cursor_pos.1,
                        Color::Blue,
                        &cur_directory[cursor_pos.1 as usize],
                    )?;
                }
                KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    // Grab a line
                    stdout.execute(cursor::MoveToColumn(0))?;
                    unhighlight_line(cursor_pos.1, &cur_directory[cursor_pos.1 as usize])?;
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
fn highlight_line(row: u16, color: Color, contents: &str) -> Result<()> {
    // Reprint the line?
    let size = terminal::size().unwrap();

    stdout()
        .queue(style::PrintStyledContent(
            contents.with(Color::Black).on(Color::Blue),
        ))?
        .queue(cursor::MoveToColumn(0))?;

    Ok(())
}

// Unhighlights the given row with the given color
fn unhighlight_line(row: u16, contents: &str) -> Result<()> {
    // Reprint the line?
    let size = terminal::size().unwrap();

    stdout()
        .queue(style::Print(contents))?
        .queue(cursor::MoveToColumn(0))?;

    Ok(())
}
