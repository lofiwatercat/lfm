use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::env;
use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;

fn main() {
    // Take a directory given as an argument or default to current directory
    // let args: Vec<String> = env::args().collect();
    let mut current_path = PathBuf::new();

    // TODO: Fix in the case where we take an argument. For now, default to current directory
    current_path = env::current_dir().unwrap();
    if !current_path.is_dir() {
        println!("Not a directory");
    }

    println!("Default: {}", current_path.display());

    // Setup
    let mut stdout = stdout();
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .expect("Unable to clear terminal")
        .queue(cursor::MoveTo(0, 0))
        .expect("Unable to move cursor");

    // Read the contents of the current path
    for entry in fs::read_dir(current_path).unwrap() {
        println!("{:?}", entry.unwrap().path());
    }
}
