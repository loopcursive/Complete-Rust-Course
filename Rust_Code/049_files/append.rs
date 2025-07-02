use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut _data_file = OpenOptions::new()
        .append(true)
        .open("learn.txt")
        .expect("Unable to open file");
    
    println!("File opened successfully");

    _data_file
        .write_all(b"\nThis is a test file for reading and writing files in Rust.")
        .expect("Unable to write to file");
    println!("Data written successfully");
}
