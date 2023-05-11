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
use walkdir::WalkDir;

struct CleanUp;

enum Status {
    Primary,
    Secondary,
    Parent,
}

struct Tab {
    dir_path: path::PathBuf,
    parent_path: path::PathBuf,
    entries: Vec<path::PathBuf>,
    entries_str: Vec<String>,
    current_entry_index: i32,
    status: Status,
}

impl Tab {
    fn highlight_line(&self) -> Result<()> {
        let original_position = cursor::position().unwrap();
        let secondary_offset = terminal::size().unwrap().0 / 2;

        let (mut width, _) = terminal::size().unwrap();
        width /= 2;

        let contents = &self.entries_str[self.current_entry_index as usize];

        stdout().queue(style::PrintStyledContent(
            contents.clone().with(Color::Black).on(Color::Blue),
        ))?;

        for _ in 0..width - contents.len() as u16 {
            stdout().queue(style::PrintStyledContent(
                " ".with(Color::Black).on(Color::Blue),
            ))?;
        }

        stdout().queue(cursor::MoveTo(original_position.0, original_position.1))?;

        Ok(())
    }

    fn unhighlight_line(&self) -> Result<()> {
        let original_position = cursor::position().unwrap();
        let secondary_offset = terminal::size().unwrap().0 / 2;

        let (mut width, _) = terminal::size().unwrap();
        width /= 2;

        let contents = &self.entries_str[self.current_entry_index as usize];

        stdout().queue(style::Print(contents))?;

        for _ in 0..width - contents.len() as u16 {
            stdout().queue(style::Print(" "))?;
        }
        stdout().queue(cursor::MoveTo(original_position.0, original_position.1))?;

        Ok(())
    }

    fn new(dir_path: path::PathBuf, status: Status) -> Option<Tab> {
        let mut entries: Vec<path::PathBuf> = Vec::new();
        let mut entries_str: Vec<String> = Vec::new();
        let mut num_dirs = 0;
        for entry in WalkDir::new(&dir_path)
            .min_depth(1)
            .max_depth(1)
            // .sort_by(|a, b| a.file_name().cmp(b.file_name()))
            .sort_by_key(|a| a.path().is_file())
            .into_iter()
        {
            let dir_entry = entry.unwrap();
            if dir_entry.clone().into_path().is_dir() {
                num_dirs += 1;
            }
            entries.push(dir_entry.into_path());
        }

        // Reverse the order of the directories to be alphabetical
        if num_dirs > 0 {
            num_dirs -= 1;
        }

        for index in 0..num_dirs {
            entries.swap(index, num_dirs - index);
        }

        for entry in &entries {
            entries_str.push(entry.file_name().unwrap().to_str().unwrap().to_string());
        }

        let parent_path = dir_path.parent().unwrap().to_path_buf();

        Some(Tab {
            dir_path,
            parent_path,
            entries,
            entries_str,
            current_entry_index: 0,
            status,
        })
    }

    // Draws the children
    fn draw(&self) {
        // Move the cursor to the right position.
        // Primary -> All the way left.
        // Secondary -> Middle
        let original_position = cursor::position().unwrap();
        let secondary_offset = terminal::size().unwrap().0 / 2;
        match self.status {
            Status::Primary => {
                stdout().queue(cursor::MoveTo(0, 0)).unwrap();
            }
            Status::Secondary => {
                stdout().queue(cursor::MoveTo(secondary_offset, 0)).unwrap();
            }
            _ => {}
        }
        stdout()
            .queue(style::SetForegroundColor(Color::White))
            .unwrap();
        for entry in &self.entries {
            let (cursor_x, cursor_y) = cursor::position().unwrap();
            stdout()
                .queue(style::Print(entry.file_name().unwrap().to_str().unwrap()))
                .unwrap()
                .queue(cursor::MoveTo(cursor_x, cursor_y + 1))
                .unwrap();
        }
        stdout()
            .queue(style::ResetColor)
            .unwrap()
            .queue(cursor::MoveTo(original_position.0, original_position.1))
            .unwrap();
        stdout().flush().unwrap();
    }

    // Clears the tab. Primarily used for the secondary tab
    fn clear(&self) {
        let original_position = cursor::position().unwrap();
        let secondary_offset = terminal::size().unwrap().0 / 2;
        match self.status {
            Status::Primary => (),
            Status::Secondary => {
                stdout().queue(cursor::MoveTo(secondary_offset, 0)).unwrap();
            }
            _ => {}
        }
        for entry in &self.entries {
            let (cursor_x, cursor_y) = cursor::position().unwrap();
            stdout()
                .queue(terminal::Clear(terminal::ClearType::UntilNewLine))
                .unwrap()
                .queue(cursor::MoveTo(cursor_x, cursor_y + 1))
                .unwrap();
        }
        stdout()
            .queue(cursor::MoveTo(original_position.0, original_position.1))
            .unwrap();
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
    // let mut dirs: HashMap<&path::PathBuf, Vec<walkdir::DirEntry>> = HashMap::new();
    // let mut cur_directory_entries: Vec<String> = Vec::new();
    // add_to_dirs(&current_path, &mut dirs);

    let copy_path = current_path.clone();
    // Current tab will show the contents of the current directory
    let mut primary_tab = Tab::new(copy_path, Status::Primary).unwrap();
    // child_tab will either be Some or None
    let mut secondary_tab = Tab::new(primary_tab.entries[0].clone(), Status::Secondary);

    let mut parent_tab = Tab::new(primary_tab.parent_path.clone(), Status::Parent).unwrap();

    // Prints the contents of the current tab
    stdout.queue(cursor::Show).unwrap();
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
    stdout.queue(cursor::Show).unwrap();

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

                    secondary_tab = Tab::new(
                        primary_tab.entries[cursor_pos.1 as usize].clone(),
                        Status::Secondary,
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
                    stdout.execute(cursor::MoveUp(1))?;
                    cursor_pos = cursor::position().unwrap();
                    primary_tab.current_entry_index -= 1;
                    primary_tab.highlight_line().unwrap();

                    secondary_tab = Tab::new(
                        primary_tab.entries[cursor_pos.1 as usize].clone(),
                        Status::Secondary,
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
                    match secondary_tab {
                        Some(ref tab) => {
                            primary_tab.clear();
                            tab.clear();
                            parent_tab = primary_tab;
                            parent_tab.status = Status::Parent;
                            primary_tab = Tab::new(tab.dir_path.clone(), Status::Primary).unwrap();
                        }
                        None => (),
                    }
                    primary_tab.draw();
                    stdout.queue(cursor::MoveTo(0, 0)).unwrap();
                    primary_tab.highlight_line().unwrap();

                    secondary_tab = Tab::new(
                        primary_tab.entries[cursor_pos.1 as usize].clone(),
                        Status::Secondary,
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
                    secondary_tab = Some(primary_tab);
                    primary_tab = parent_tab;
                    primary_tab.status = Status::Primary;
                    parent_tab = Tab::new(primary_tab.parent_path.clone(), Status::Parent).unwrap();
                }
                KeyEvent {
                    code: KeyCode::Char('t'),
                    modifiers: event::KeyModifiers::NONE,
                    ..
                } => {
                    // Grab a line
                    stdout.execute(cursor::MoveToColumn(0))?;
                    // entries = get_strings_from_dir(&current_path, &dirs);
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
