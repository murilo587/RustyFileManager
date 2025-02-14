use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() {
    loop {
        let mut choice = String::new();
        println!("MENU");
        println!("1. List all files of the current folder");
        println!("2. Create new file");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\x1b[31mInvalid input, please enter a number\x1b[0m");
                continue;
            }
        };

        match choice {
            1 => {
                list_files("src/files/.");
            }
            2 => {
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

                create_file("src/files/.", file_name, content);
            }
            _ => break,
        }
    }

    fn list_files(directory: &str) {
        let paths = fs::read_dir(directory).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    println!("file: {}", file_name_str);
                }
            }
        }
    }

    fn create_file(directory: &str, file_name: &str, content: &str) {
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
}
