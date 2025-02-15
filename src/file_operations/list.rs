use std::fs;

pub fn list_files(directory: &str) {
    let paths = fs::read_dir(directory).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if let Some(file_name) = path.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                println!("\x1b[32mfile: {}\x1b[0m", file_name_str);
            }
        }
    }
}
