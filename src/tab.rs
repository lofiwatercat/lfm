use crossterm::{
    cursor,
    // event::{self, Event, KeyCode, KeyEvent},
    style::{self, Color, Stylize},
    terminal,
    ExecutableCommand,
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
    pub child_tabs: Option<Vec<Tab>>,
    pub file_entries: Vec<path::PathBuf>,
    pub dir_entries: Vec<path::PathBuf>,
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

        let mut child_tabs: Vec<Tab> = Vec::new();

        for dir in dir_entries.clone() {
            child_tabs.push(Tab::new(dir, Status::Secondary).unwrap());
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
            child_tabs: Some(child_tabs),
            // entries,
            // entries_str,
            current_entry_index: 0,
            status,
        })
    }

    // Updates the parent_tab while placing the current tab data into the parent tab's corresponding child tab
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

    // Creates all the child tabs for each dir entry
    // Doesn't replace existing tabs
    pub fn update_child_tabs(&mut self) {
        let mut child_tabs: Vec<Tab> = Vec::new();
        let mut existing_child_tabs: Vec<path::PathBuf> = Vec::new();

        for child_tab in self.child_tabs.clone().unwrap() {
            existing_child_tabs.push(child_tab.dir_path);
        }

        for dir in self.dir_entries.clone() {
            if !existing_child_tabs.contains(&dir) {
                child_tabs.push(Tab::new(dir, Status::Secondary).unwrap());
            }
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

    pub fn clear2() {
        stdout()
            .queue(terminal::Clear(terminal::ClearType::All))
            .unwrap();
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
    pub fn move_down(&mut self) {
        // If current index is on a dir, then we need to clear the secondary tab
        if self.current_entry_index < self.dir_entries.len() as i32 {
            self.child_tabs.as_mut().unwrap()[self.current_entry_index as usize].clear();
        }
        self.unhighlight_line().unwrap();
        stdout().execute(cursor::MoveDown(1)).unwrap();
        self.current_entry_index += 1;
        self.highlight_line().unwrap();

        // println!("cei: {}", self.current_entry_index);
        // println!("dir_entry.len: {}", self.dir_entries.len());
        // println!("{:?}", self.dir_entries);

        // Don't draw the child tab if it isn't a directory
        if self.current_entry_index < self.dir_entries.len() as i32 {
            self.child_tabs.as_mut().unwrap()[self.current_entry_index as usize].draw();
        }
    }

    // Moves a line up
    pub fn move_up(&mut self) {
        // If current index is on a dir, then we need to clear the secondary tab
        if self.current_entry_index < self.dir_entries.len() as i32 {
            self.child_tabs.as_mut().unwrap()[self.current_entry_index as usize].clear();
        }
        self.unhighlight_line().unwrap();
        stdout()
            .execute(cursor::MoveUp(1))
            .expect("Couldn't move up");
        self.current_entry_index -= 1;
        self.highlight_line().expect("Couldn't highlight");

        if self.current_entry_index < self.dir_entries.len() as i32 {
            self.child_tabs.as_mut().unwrap()[self.current_entry_index as usize].draw();
        }
    }

    // Clears the tabs and makes the parent tab the new primary tab
    pub fn go_to_parent_tab(&mut self) {
        // Clear current tabs
        if self.current_entry_index < self.dir_entries.len() as i32 {
            self.child_tabs.as_mut().unwrap()[self.current_entry_index as usize].clear();
        }
        self.clear();
        // self.update_parent();
        // let mut old_tab = self.clone();
        // // Make the old tab a child tab with secondary status
        // old_tab.status = Status::Secondary;
        // If the parent tab exists, then replace the data with the parent data.
        // If not, create a new tab for the parent.
        // Now the parent tab has takes over. Replace data with the parent data
        let parent_tab: Tab;
        match self.parent_tab.clone() {
            Some(tab) => parent_tab = *tab,
            None => {
                parent_tab = *Box::new(Tab::new(self.parent_path.clone(), Status::Parent).unwrap())
            }
        };

        // Using parent tab as a reference
        // let parent_tab = *(self.parent_tab.clone().unwrap());
        self.dir_path = self.parent_path.clone();
        self.parent_path = self.dir_path.parent().unwrap().to_path_buf();
        self.dir_entries = parent_tab.dir_entries;
        self.file_entries = parent_tab.file_entries;
        self.current_entry_index = 0;
        self.update_child_tabs();
        stdout()
            .queue(cursor::MoveTo(0, self.current_entry_index as u16))
            .unwrap();

        self.draw();
        if self.current_entry_index < self.dir_entries.len() as i32 {
            self.child_tabs.as_mut().unwrap()[self.current_entry_index as usize].draw();
        }
        self.highlight_line().unwrap();
    }

    // Change the primary tab to the child tab
    pub fn go_to_child_tab(&mut self) {
        if self.current_entry_index < self.dir_entries.len() as i32 {
            self.child_tabs.as_mut().unwrap()[self.current_entry_index as usize].clear();
        }
        self.clear();

        let new_tab = self.child_tabs.clone().unwrap()[self.current_entry_index as usize].clone();

        self.parent_path = self.dir_path.clone();
        self.dir_path = new_tab.dir_path;
        self.file_entries = new_tab.file_entries;
        self.dir_entries = new_tab.dir_entries;
        self.current_entry_index = new_tab.current_entry_index;
        self.update_parent();
        self.update_child_tabs();

        stdout().queue(cursor::MoveTo(0, 0)).unwrap();
        self.draw();
        if self.current_entry_index < self.dir_entries.len() as i32 {
            self.child_tabs.as_mut().unwrap()[self.current_entry_index as usize].draw();
        }
        self.highlight_line().unwrap();
    }
}
