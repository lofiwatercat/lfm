use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{self, Attribute, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::collections::HashMap;
use std::env;
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

    // TODO: Fix in the case where we take an argument. For now, default to current directory
    let mut current_path = env::current_dir().unwrap();
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
    let mut dirs: HashMap<&PathBuf, Vec<walkdir::DirEntry>> = HashMap::new();
    let mut cur_directory: Vec<String> = Vec::new();
    // for entry in WalkDir::new(&current_path).min_depth(1).max_depth(1) {
    //     println!("{}", entry?.file_name().to_str().unwrap());
    //     stdout.queue(cursor::MoveToColumn(0))?;
    // }

    add_to_dirs(&current_path, &mut dirs);
    print_dir_contents(&current_path, Color::White);
    // for entry in fs::read_dir(current_path).unwrap() {
    //     cur_directory.push(entry.as_ref().unwrap().file_name().into_string().unwrap());
    //     let name = entry.unwrap();
    //     if name.file_type()?.is_dir() {
    //         let mut child_entries = Vec::new();
    //         for child_entry in fs::read_dir(name.path()).unwrap() {
    //             child_entries.push(child_entry.unwrap());
    //         }
    //         dirs.insert(name.path(), child_entries);
    //     }
    // }

    // Testing, see what is in dirs
    // for (key, value) in dirs.iter() {
    //     println!("Key: {:?}, DirEntry: {:?}", key, value);
    //     stdout.execute(cursor::MoveToColumn(0))?;
    // }

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

// Adds the directory at the path to the given HashMap
fn add_to_dirs<'a>(
    current_dir: &'a PathBuf,
    dirs: &mut HashMap<&'a PathBuf, Vec<walkdir::DirEntry>>,
) {
    // Check if path is a directory
    if !current_dir.is_dir() {
        println!("Not a directory!");
        return;
    }

    // Add the contents of the directory to a vector
    let mut contents: Vec<walkdir::DirEntry> = Vec::new();

    // Use WalkDir
    for entry in WalkDir::new(current_dir).min_depth(1).max_depth(1) {
        contents.push(entry.unwrap());
    }

    dirs.insert(current_dir, contents);
}

// Prints the contents of the given directory
fn print_dir_contents(dir: &PathBuf, color: Color) {
    for entry in WalkDir::new(dir).min_depth(1).max_depth(1) {
        stdout()
            .queue(style::PrintStyledContent(
                entry
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .with(Color::White),
            ))
            .unwrap()
            .queue(cursor::MoveToNextLine(1))
            .unwrap();
    }
    stdout().flush().unwrap();
}
