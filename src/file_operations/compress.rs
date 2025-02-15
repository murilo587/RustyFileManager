use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn compress_file(path: &str) {
    let mut input_file = String::new();
    let mut output_file = String::new();

    println!("Enter the name of the file to compress (include the extension):");
    io::stdin()
        .read_line(&mut input_file)
        .expect("\x1b[31mFailed to read the file name\x1b[0m");
    println!("Enter the name of the output file (include the .gz extension):");
    io::stdin()
        .read_line(&mut output_file)
        .expect("\x1b[31mFailed to read the output file name\x1b[0m");

    let input_file = input_file.trim();
    let output_file = output_file.trim();
    let input_path = format!("{}/{}", path, input_file);
    let output_path = format!("{}/{}", path, output_file);

    if !Path::new(&input_path).exists() {
        println!(
            "\x1b[31mThe input file does not exist: {}\x1b[0m",
            input_path
        );
        return;
    }

    let input_data = match fs::read(&input_path) {
        Ok(data) => data,
        Err(e) => {
            println!("\x1b[31mError reading the input file: {}\x1b[0m", e);
            return;
        }
    };

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    if let Err(e) = encoder.write_all(&input_data) {
        println!("\x1b[31mError compressing the file: {}\x1b[0m", e);
        return;
    }

    let compressed_data = match encoder.finish() {
        Ok(data) => data,
        Err(e) => {
            println!("\x1b[31mError finalizing compression: {}\x1b[0m", e);
            return;
        }
    };

    if let Err(e) = fs::write(&output_path, compressed_data) {
        println!("\x1b[31mError writing the compressed file: {}\x1b[0m", e);
        return;
    }

    println!(
        "\x1b[32mFile compressed successfully: {}\x1b[0m",
        output_path
    );
}

pub fn decompress_file(path: &str) {
    let mut input_file = String::new();
    let mut output_file = String::new();

    println!("Enter the name of the file to decompress (include the .gz extension):");
    io::stdin()
        .read_line(&mut input_file)
        .expect("\x1b[31mFailed to read the file name\x1b[0m");
    println!("Enter the name of the output file (include the extension):");
    io::stdin()
        .read_line(&mut output_file)
        .expect("\x1b[31mFailed to read the output file name\x1b[0m");

    let input_file = input_file.trim();
    let output_file = output_file.trim();
    let input_path = format!("{}/{}", path, input_file);
    let output_path = format!("{}/{}", path, output_file);

    if !Path::new(&input_path).exists() {
        println!(
            "\x1b[31mThe input file does not exist: {}\x1b[0m",
            input_path
        );
        return;
    }

    let compressed_data = match fs::read(&input_path) {
        Ok(data) => data,
        Err(e) => {
            println!("\x1b[31mError reading the input file: {}\x1b[0m", e);
            return;
        }
    };

    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut output_data = Vec::new();
    if let Err(e) = decoder.read_to_end(&mut output_data) {
        println!("\x1b[31mError decompressing the file: {}\x1b[0m", e);
        return;
    }

    if let Err(e) = fs::write(&output_path, output_data) {
        println!("\x1b[31mError writing the decompressed file: {}\x1b[0m", e);
        return;
    }

    println!(
        "\x1b[32mFile decompressed successfully: {}\x1b[0m",
        output_path
    );
}
