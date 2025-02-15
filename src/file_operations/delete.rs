use std::fs;
use std::io;
use std::path::Path;

pub fn delete_file(directory: &str) {
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

    match fs::remove_file(&path) {
        Ok(_) => {
            println!("\x1b[32mFile '{}' deleted successfully\x1b[0m", file_name);
        }
        Err(e) => {
            println!("\x1b[31mError deleting file: {}\x1b[0m", e);
        }
    }
}
