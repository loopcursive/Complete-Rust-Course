use  std::io::Write;
use std::fs::File;

fn main(){
    let mut _data_file = File::create("delete.txt").expect("Unable to create file");
    println!("File created successfully");

    _data_file.write_all(b"Hello, \nworld! and 1 2").expect("Unable to write to file");
    println!("Data written successfully");
} 