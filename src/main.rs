mod cleanup;
mod tab;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    style::{self, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
// use std::collections::HashMap;
use std::env;
use std::io::{stdout, Write};
use std::path;
// use walkdir::WalkDir;

// Moves a line down

fn main() -> Result<()> {
    let _clean_up = cleanup::CleanUp;

    // Take a directory given as an argument or default to current directory
    // let args: Vec<String> = env::args().collect();

    // TODO: Fix in the case where we take an argument. For now, default to current directory
    let mut current_path = env::current_dir().unwrap();
    if !current_path.is_dir() {
        println!("Not a directory");
    }

    // Setup the terminal
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
    // let mut dirs: HashMap<&path::PathBuf, Vec<walkdir::DirEntry>> = HashMap::new();
    // let mut cur_directory_entries: Vec<String> = Vec::new();
    // add_to_dirs(&current_path, &mut dirs);

    // Current tab will show the contents of the current directory
    let mut primary_tab = tab::Tab::new(current_path, tab::Status::Primary).unwrap();
    // child_tab will either be Some or None
    let entries = primary_tab.get_entries();
    let mut secondary_tab = tab::Tab::new(
        path::PathBuf::from(entries[0].clone()),
        tab::Status::Secondary,
    );

    // primary_tab.update_child_tabs();

    primary_tab.child_tabs.as_ref().unwrap()[0].draw();

    let mut parent_tab =
        tab::Tab::new(primary_tab.parent_path.clone(), tab::Status::Parent).unwrap();

    // Prints the contents of the current tab
    // stdout.queue(cursor::Show).unwrap();
    primary_tab.draw();
    // match secondary_tab {
    //     Some(ref i) => i.draw(),
    //     None => (),
    // }

    // Setup for loop
    // let mut entries = get_strings_from_dir(&current_path, &dirs);
    stdout.queue(cursor::MoveToRow(0))?;
    // let mut entries = primary_tab.entries.clone();
    // let mut entries: Vec<&str> = entries
    //     .iter()
    //     .map(|entry| entry.file_name().unwrap().to_str().unwrap())
    //     .collect();

    primary_tab.highlight_line().unwrap();

    stdout.flush()?;
    // stdout.queue(cursor::Show).unwrap();

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
                    primary_tab.move_down();
                }
                KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    primary_tab.move_up();
                }
                KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    primary_tab.go_to_parent_tab();
                }
                KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    primary_tab.go_to_child_tab();
                }
                KeyEvent {
                    code: KeyCode::Char('t'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    // Grab a line
                    stdout.execute(cursor::MoveToColumn(0))?;
                    primary_tab.highlight_line().unwrap();
                }
                KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    // Grab a line
                    stdout.execute(cursor::MoveToColumn(0))?;
                    // entries = get_strings_from_dir(&current_path, &dirs);
                    primary_tab.clone().highlight_line().unwrap();
                }
                _ => {
                    println!("{:?}\r", event)
                }
            }
        }
    }

    Ok(())
}
