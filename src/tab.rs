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
    pub entries: Vec<path::PathBuf>,
    entries_str: Vec<String>,
    pub current_entry_index: i32,
    pub status: Status,
}

impl Tab {
    // Lazily highlights a line and flushes it out at the end
    pub fn highlight_line(&self) -> Result<()> {
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

    pub fn unhighlight_line(&self) -> Result<()> {
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

    pub fn new(dir_path: path::PathBuf, status: Status) -> Option<Tab> {
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
            parent_tab: None,
            parent_path,
            child_tabs: None,
            entries,
            entries_str,
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
        for entry in &self.entries {
            child_tabs.push(Tab::new(entry.clone(), Status::Secondary).unwrap());
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
