use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn create_file(directory: &str) {
    let mut file_name = String::new();
    let mut content = String::new();

    println!("Enter file name (with extension):");
    io::stdin()
        .read_line(&mut file_name)
        .expect("Invalid input");
    println!("Enter file content:");
    io::stdin().read_line(&mut content).expect("Invalid input");

    let file_name = file_name.trim();
    let content = content.trim();
    let path = Path::new(directory).join(file_name);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(_) => {
            println!("\x1b[31mFailed to create file\x1b[0m");
            return;
        }
    };

    if let Err(_) = file.write_all(content.as_bytes()) {
        println!("\x1b[31mFailed to write content to file\x1b[0m");
        return;
    }

    println!("\x1b[32mDone!\x1b[0m");
}
