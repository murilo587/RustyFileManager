use std::fs::File;
use std::io::Read;
use std::io::{self, Write};
use std::path::Path;

pub fn edit_file(directory: &str) {
    let mut file_name = String::new();
    println!("Enter file name (with extension):");
    io::stdin()
        .read_line(&mut file_name)
        .expect("Invalid input");

    let file_name = file_name.trim();
    let path = Path::new(directory).join(file_name);

    if !path.exists() {
        println!("\x1b[31mFile does not exist\x1b[0m");
        return;
    }

    let mut current_content = String::new();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("\x1b[31mFailed to open file for editing\x1b[0m");
            return;
        }
    };

    if let Err(_) = file.read_to_string(&mut current_content) {
        println!("\x1b[31mFailed to read content from file\x1b[0m");
        return;
    }

    println!("Current content of '{}':", file_name);
    println!("{}", current_content);

    let mut new_content = String::new();
    println!("Enter new content (or press 'Enter' to keep the current content):");
    io::stdin()
        .read_line(&mut new_content)
        .expect("Invalid input");

    if new_content.trim().is_empty() {
        println!("\x1b[32mFile remains unchanged!\x1b[0m");
        return;
    }

    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("\x1b[31mFailed to open file for editing\x1b[0m");
            return;
        }
    };

    if let Err(_) = file.write_all(new_content.as_bytes()) {
        println!("\x1b[31mFailed to write new content to file\x1b[0m");
        return;
    }

    println!("\x1b[32mFile edited successfully!\x1b[0m");
}
