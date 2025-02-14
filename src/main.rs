use std::io;

fn main() {
    loop {
        let mut choice = String::new();
        println!("MENU");
        println!("1. List all files of the current folder");
        println!("2. Exit");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number");
                continue;
            }
        };

        match choice {
            1 => {
                //function to list all files
            }
            2 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, try again!"),
        }
    }
}
