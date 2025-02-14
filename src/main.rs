use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    loop {
        let mut choice = String::new();
        println!("MENU");
        println!("1. List all files of the current folder");
        println!("2. Create new file");
        println!("3. Edit an existing file");
        println!("4. Delete an existing file");

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
            3 => {
                let mut file_name = String::new();

                println!("Enter file name (with extension):");
                io::stdin()
                    .read_line(&mut file_name)
                    .expect("Invalid input");

                let file_name = file_name.trim();

                edit_file("src/files/.", file_name);
            }
            4 => {
                let mut file_name = String::new();
                println!("Enter file name (with extension):");
                io::stdin()
                    .read_line(&mut file_name)
                    .expect("Invalid input");

                let file_name = file_name.trim();
                delete_file("src/files/.", file_name);
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

    fn edit_file(directory: &str, file_name: &str) {
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

    fn delete_file(directory: &str, file_name: &str) {
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
}
