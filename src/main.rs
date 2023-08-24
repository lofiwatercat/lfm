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
// use std::path;
// use walkdir::WalkDir;

fn main() -> Result<()> {
    let _clean_up = cleanup::CleanUp;

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
    // let mut dirs: HashMap<&path::PathBuf, Vec<walkdir::DirEntry>> = HashMap::new();
    // let mut cur_directory_entries: Vec<String> = Vec::new();
    // add_to_dirs(&current_path, &mut dirs);

    let copy_path = current_path.clone();
    // Current tab will show the contents of the current directory
    let mut primary_tab = tab::Tab::new(copy_path, tab::Status::Primary).unwrap();
    // child_tab will either be Some or None
    let mut secondary_tab = tab::Tab::new(primary_tab.entries[0].clone(), tab::Status::Secondary);

    let mut parent_tab =
        tab::Tab::new(primary_tab.parent_path.clone(), tab::Status::Parent).unwrap();

    // Prints the contents of the current tab
    // stdout.queue(cursor::Show).unwrap();
    primary_tab.draw();
    match secondary_tab {
        Some(ref i) => i.draw(),
        None => (),
    }

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
                    match secondary_tab {
                        Some(tab) => tab.clear(),
                        None => (),
                    };
                    primary_tab.unhighlight_line().unwrap();
                    stdout.execute(cursor::MoveDown(1))?;
                    cursor_pos = cursor::position().unwrap();
                    primary_tab.current_entry_index += 1;
                    primary_tab.highlight_line().unwrap();

                    secondary_tab = tab::Tab::new(
                        primary_tab.entries[cursor_pos.1 as usize].clone(),
                        tab::Status::Secondary,
                    );

                    match secondary_tab {
                        Some(ref tab) => tab.draw(),
                        None => (),
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    match secondary_tab {
                        Some(tab) => tab.clear(),
                        None => (),
                    };

                    primary_tab.unhighlight_line().unwrap();
                    stdout.execute(cursor::MoveUp(1)).expect("Couldn't move up");
                    cursor_pos = cursor::position().unwrap();
                    primary_tab.current_entry_index -= 1;
                    primary_tab.highlight_line().expect("Couldn't highlight");

                    secondary_tab = tab::Tab::new(
                        primary_tab.entries[cursor_pos.1 as usize].clone(),
                        tab::Status::Secondary,
                    );

                    match secondary_tab {
                        Some(ref i) => i.draw(),
                        None => (),
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    primary_tab.clear();
                    match secondary_tab {
                        Some(ref tab) => {
                            tab.clear();
                            parent_tab = primary_tab.clone();
                            parent_tab.status = tab::Status::Parent;
                            // primary_tab = Tab::new(tab.dir_path.clone(), Status::Primary).unwrap();
                            primary_tab = tab.clone();
                        }
                        None => (),
                    }

                    // Update primary tab to be primary tab after cloning secondary tab
                    primary_tab.status = tab::Status::Primary;
                    primary_tab.clear();
                    primary_tab.draw();
                    stdout
                        .queue(cursor::MoveTo(0, primary_tab.current_entry_index as u16))
                        .unwrap();
                    primary_tab.highlight_line().unwrap();

                    secondary_tab = tab::Tab::new(
                        primary_tab.entries[0 as usize].clone(),
                        tab::Status::Secondary,
                    );

                    match secondary_tab {
                        Some(ref tab) => {
                            tab.draw();
                        }
                        None => (),
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    let old_index = primary_tab.current_entry_index;
                    // Clear the old tabs
                    primary_tab.clear();
                    match secondary_tab {
                        Some(ref tab) => {
                            tab.clear();
                        }
                        None => (),
                    }

                    // Remember index of the current tab before we go back
                    let current_dir = primary_tab.dir_path;

                    primary_tab = parent_tab.clone();
                    // print!("{:?}", primary_tab.parent_path);
                    parent_tab =
                        tab::Tab::new(primary_tab.parent_path.clone(), tab::Status::Parent)
                            .expect("Couldn't make parent tab");

                    let current_index = primary_tab
                        .entries
                        .iter()
                        .position(|entry| entry == &current_dir)
                        .expect("Couldn't make current index");

                    let current_dir = primary_tab.dir_path.clone();

                    let parent_index = parent_tab
                        .entries
                        .iter()
                        .position(|entry| entry == &current_dir)
                        .expect("Couldn't make parent index");

                    parent_tab.current_entry_index = parent_index as i32;

                    primary_tab.current_entry_index = current_index as i32;
                    primary_tab.status = tab::Status::Primary;
                    secondary_tab = tab::Tab::new(
                        primary_tab.entries[primary_tab.current_entry_index as usize].clone(),
                        tab::Status::Secondary,
                    );

                    match secondary_tab {
                        Some(ref mut tab) => tab.current_entry_index = old_index,
                        None => (),
                    }
                    primary_tab.draw();

                    // primary_tab = Tab::new(primary_tab.parent_path, Status::Parent).unwrap();
                    // secondary_tab = Tab::new(primary_tab.entries[0].clone(), Status::Secondary);
                    // primary_tab.draw();

                    match secondary_tab {
                        Some(ref tab) => {
                            tab.draw();
                        }
                        None => (),
                    }

                    stdout
                        .queue(cursor::MoveToRow(primary_tab.current_entry_index as u16))
                        .expect("Couldn't move cursor");

                    primary_tab.highlight_line().expect("Couldn't highlight");
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
                    primary_tab.highlight_line().unwrap();
                }
                _ => {
                    println!("{:?}\r", event)
                }
            }
        }
    }

    Ok(())
}
