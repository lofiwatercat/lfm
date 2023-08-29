use crossterm::{
    cursor,
    // event::{self, Event, KeyCode, KeyEvent},
    style::{self, Color, Stylize},
    terminal,
    // ExecutableCommand,
    QueueableCommand,
    Result,
};
// use std::collections::HashMap;
// use std::env;
use std::io::{stdout, Write};
use std::path;
use walkdir::WalkDir;

#[derive(Clone)]
pub enum Status {
    Primary,
    Secondary,
    Parent,
}

#[derive(Clone)]
pub struct Tab {
    pub dir_path: path::PathBuf,
    pub parent_path: path::PathBuf,
    parent_tab: Option<Box<Tab>>,
    child_tabs: Option<Vec<Tab>>,
    pub file_entries: Vec<path::PathBuf>,
    pub dir_entries: Vec<path::PathBuf>,
    // pub entries_str: Vec<String>,
    pub current_entry_index: i32,
    pub status: Status,
}

impl Tab {
    pub fn get_entries(&self) -> Vec<String> {
        let mut entries = self.dir_entries.clone();

        entries.append(&mut self.file_entries.clone());

        let entries: Vec<String> = entries
            .iter()
            .map(|entry| entry.file_name().unwrap().to_str().unwrap().to_string())
            .collect();

        return entries;
    }

    // Lazily highlights a line and flushes it out at the end
    pub fn highlight_line(&self) -> Result<()> {
        let original_position = cursor::position().unwrap();
        let secondary_offset = terminal::size().unwrap().0 / 2;

        let (mut width, _) = terminal::size().unwrap();
        width /= 2;

        // Make a vector with dirs first then files
        // let entries = self.dir_entries.clone();
        // entries.append(&mut self.file_entries.clone());
        let entries = self.get_entries();

        let content = entries[self.current_entry_index as usize].clone();

        // Queue up the highlighted line
        stdout().queue(style::PrintStyledContent(
            content.clone().with(Color::Black).on(Color::Blue),
        ))?;

        for _ in 0..width - content.len() as u16 {
            stdout().queue(style::PrintStyledContent(
                " ".with(Color::Black).on(Color::Blue),
            ))?;
        }

        stdout().queue(cursor::MoveTo(original_position.0, original_position.1))?;

        Ok(())
    }

    pub fn unhighlight_line(&self) -> Result<()> {
        let original_position = cursor::position().unwrap();
        let secondary_offset = terminal::size().unwrap().0 / 2;

        let (mut width, _) = terminal::size().unwrap();
        width /= 2;

        // let contents = &self.entries_str[self.current_entry_index as usize];
        let content = self.get_entries()[self.current_entry_index as usize].clone();

        stdout().queue(style::Print(content.clone()))?;

        for _ in 0..width - content.len() as u16 {
            stdout().queue(style::Print(" "))?;
        }
        stdout().queue(cursor::MoveTo(original_position.0, original_position.1))?;

        Ok(())
    }

    pub fn new(dir_path: path::PathBuf, status: Status) -> Option<Tab> {
        // let mut entries: Vec<Entry> = Vec::new();
        let mut file_entries: Vec<path::PathBuf> = Vec::new();
        let mut dir_entries: Vec<path::PathBuf> = Vec::new();
        for entry in WalkDir::new(&dir_path)
            .min_depth(1)
            .max_depth(1)
            // .sort_by(|a, b| a.file_name().cmp(b.file_name()))
            .sort_by_key(|a| a.path().is_file())
            .into_iter()
        {
            let entry_path = path::PathBuf::from(entry.as_ref().unwrap().path());
            if entry.unwrap().file_type().is_dir() {
                dir_entries.push(entry_path);
            } else {
                file_entries.push(entry_path)
            }
        }

        // Reverse the order of the directories to be alphabetical
        // if num_dirs > 0 {
        //     num_dirs -= 1;
        // }

        // for index in 0..num_dirs {
        //     entries.swap(index, num_dirs - index);
        // }

        // for entry in &entries {
        //     match entry {
        //         Entry::File(file) => {
        //             entries_str.push(file.file_name().unwrap().to_str().unwrap().to_string())
        //         }
        //         Entry::Dir(tab) => entries_str.push(tab.dir_path.to_str().unwrap().to_string()),
        //     }
        // }

        let parent_path = dir_path.parent().unwrap().to_path_buf();

        Some(Tab {
            dir_path,
            parent_tab: None,
            parent_path,
            file_entries,
            dir_entries,
            child_tabs: None,
            // entries,
            // entries_str,
            current_entry_index: 0,
            status,
        })
    }

    // Updates the parent_tab
    pub fn update_parent(&mut self) {
        let mut parent_tab = Tab::new(self.parent_path.clone(), Status::Parent).unwrap();

        parent_tab.update_child_tabs();

        for mut tab in parent_tab.child_tabs.as_mut().unwrap().iter_mut() {
            if &tab.dir_path == &self.dir_path {
                tab = self;
            }
        }

        self.parent_tab = Some(Box::new(parent_tab));
    }

    pub fn update_child_tabs(&mut self) {
        let mut child_tabs: Vec<Tab> = Vec::new();

        let entries = self.get_entries();

        for entry in entries {
            child_tabs.push(Tab::new(path::PathBuf::from(entry), Status::Secondary).unwrap());
        }

        self.child_tabs = Some(child_tabs);
    }

    // Draws the children
    pub fn draw(&self) {
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

        let entries = self.get_entries();
        for entry in entries {
            let (cursor_x, cursor_y) = cursor::position().unwrap();
            stdout()
                .queue(style::Print(entry))
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
    pub fn clear(&self) {
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

        let entries = self.get_entries();

        for entry in entries {
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

    // Moves a line down
    // pub fn move_down(&self) {
    //     match self.secondary_tab {
    //         Some(tab) => tab.clear(),
    //         None => (),
    //     }
    // }
}
