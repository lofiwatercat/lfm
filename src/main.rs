use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{self, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::collections::HashMap;
use std::env;
use std::io::{stdout, Write};
use std::path;
use walkdir::WalkDir;

struct CleanUp;

struct Tab {
    dir_path: path::PathBuf,
    entries: Vec<path::PathBuf>,
}

impl Tab {
    fn new(dir_path: path::PathBuf) -> Tab {
        let mut entries: Vec<path::PathBuf> = Vec::new();
        for entry in WalkDir::new(&dir_path).min_depth(1).max_depth(1) {
            entries.push(entry.unwrap().into_path())
        }
        Tab { dir_path, entries }
    }
    // Draws the children
    fn draw(&self) {
        stdout()
            .queue(style::SetForegroundColor(Color::White))
            .unwrap();
        for entry in &self.entries {
            stdout()
                .queue(style::Print(entry.file_name().unwrap().to_str().unwrap()))
                .unwrap()
                .queue(cursor::MoveToNextLine(1))
                .unwrap();
        }
        stdout().queue(style::ResetColor).unwrap();
        stdout().flush().unwrap();
    }
}

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
        .expect("Unable to move cursor")
        .queue(cursor::Hide)?;

    // How to store files
    // Hashmap with each key being a directory and value being a vector of it's
    // contents
    // Read the contents of the current path and print them
    let mut dirs: HashMap<&path::PathBuf, Vec<walkdir::DirEntry>> = HashMap::new();
    let mut cur_directory_entries: Vec<String> = Vec::new();
    add_to_dirs(&current_path, &mut dirs);

    let copy_path = current_path.clone();
    // Current tab will show the contents of the current directory
    let cur_tab = Tab::new(copy_path);

    // Prints the contents of the current tab
    cur_tab.draw();

    // Setup for loop
    // let mut entries = get_strings_from_dir(&current_path, &dirs);
    stdout.queue(cursor::MoveToRow(0))?;
    let mut entries = cur_tab.entries;
    let mut entries: Vec<&str> = entries
        .iter()
        .map(|entry| entry.file_name().unwrap().to_str().unwrap())
        .collect();
    highlight_line(
        cursor::position().unwrap().1,
        Color::Blue,
        &entries[cursor::position().unwrap().1 as usize],
    )?;

    stdout.flush()?;

    // Process inputs
    loop {
        let mut cursor_pos = cursor::position().unwrap();
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
                    unhighlight_line(cursor_pos.1, &entries[cursor_pos.1 as usize])?;
                    stdout.queue(cursor::MoveDown(1))?;
                    cursor_pos = cursor::position().unwrap();
                    highlight_line(cursor_pos.1, Color::Blue, &entries[cursor_pos.1 as usize])?;

                    // print children if it is a dir
                    let children: &Vec<walkdir::DirEntry> = dirs.get(&current_path).unwrap();
                    // print_dir_contents(children[cursor_pos.1 as usize].path(), Color::White);

                    // print_dir_contents(dirs.get(current_path).unwrap()[cursor_pos.1]);
                }
                KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    unhighlight_line(cursor_pos.1, &entries[cursor_pos.1 as usize])?;
                    stdout.execute(cursor::MoveUp(1))?;
                    cursor_pos = cursor::position().unwrap();
                    highlight_line(cursor_pos.1, Color::Blue, &entries[cursor_pos.1 as usize])?;
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
                    entries = get_strings_from_dir(&current_path, &dirs);
                    highlight_line(cursor_pos.1, Color::Blue, &entries[cursor_pos.1 as usize])?;
                }
                KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    // Grab a line
                    stdout.execute(cursor::MoveToColumn(0))?;
                    entries = get_strings_from_dir(&current_path, &dirs);
                    unhighlight_line(cursor_pos.1, &entries[cursor_pos.1 as usize])?;
                }
                _ => {
                    println!("{:?}\r", event)
                }
            }
        }
    }

    Ok(())
}

// Returns a vector of strings converted from DirEntries from the given directory
fn get_strings_from_dir<'a>(
    current_dir: &'a path::PathBuf,
    dirs: &'a HashMap<&'a path::PathBuf, Vec<walkdir::DirEntry>>,
) -> Vec<&'a str> {
    let strings: Vec<&str> = dirs
        .get(current_dir)
        .unwrap()
        .iter()
        .map(|dir_entry| dir_entry.file_name().to_str().unwrap())
        .collect();
    strings
}

// Highlights the given row with the given color
fn highlight_line(row: u16, color: Color, contents: &str) -> Result<()> {
    // Reprint the line?
    let (width, _) = terminal::size().unwrap();

    stdout().queue(style::PrintStyledContent(
        contents.with(Color::Black).on(Color::Blue),
    ))?;

    for _ in 0..width / 2 - contents.len() as u16 {
        stdout().queue(style::PrintStyledContent(
            " ".with(Color::Black).on(Color::Blue),
        ))?;
    }
    stdout().queue(cursor::MoveToColumn(0))?;
    stdout().flush()?;

    Ok(())
}

// Unhighlights the given row with the given color
fn unhighlight_line(row: u16, contents: &str) -> Result<()> {
    // Reprint the line?
    let (width, _) = terminal::size().unwrap();

    stdout().queue(style::Print(contents))?;

    for _ in 0..width - contents.len() as u16 {
        stdout().queue(style::Print(" "))?;
    }
    stdout().queue(cursor::MoveToColumn(0))?;
    stdout().flush()?;

    Ok(())
}

// Adds the directory at the path to the given HashMap
fn add_to_dirs<'a>(
    current_dir: &'a path::PathBuf,
    dirs: &mut HashMap<&'a path::PathBuf, Vec<walkdir::DirEntry>>,
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
fn print_dir_contents(dir: &path::Path, color: Color) {
    let (width, _) = terminal::size().unwrap();
    stdout().queue(cursor::MoveToColumn(width / 2)).unwrap();
    let (start_x, _) = cursor::position().unwrap();
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
            .queue(cursor::MoveDown(1))
            .unwrap()
            .queue(cursor::MoveToColumn(start_x))
            .unwrap();
    }
    stdout().flush().unwrap();
}
