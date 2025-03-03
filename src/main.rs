use std::io;
mod file_operations;

fn main() {
    loop {
        let mut choice = String::new();
        println!("MENU");
        println!("1. List all files of the current folder");
        println!("2. Create new file");
        println!("3. Edit an existing file");
        println!("4. Delete an existing file");
        println!("5. Compress a File");
        println!("6. Decompress a File");
        println!("7. Exit");

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
                file_operations::list::list_files("src/files/.");
            }
            2 => {
                file_operations::create::create_file("src/files/.");
            }
            3 => {
                file_operations::edit::edit_file("src/files/.");
            }
            4 => {
                file_operations::delete::delete_file("src/files/.");
            }
            5 => {
                file_operations::compress::compress_file("src/files/.");
            }
            6 => {
                file_operations::compress::decompress_file("src/files/.");
            }
            7 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("\x1b[31mInvalid choice, please select a valid option\x1b[0m");
            },
        }
    }
}
