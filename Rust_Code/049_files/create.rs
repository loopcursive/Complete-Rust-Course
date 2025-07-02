use std::fs::File;

fn main() {
    let mut _data_file = File::create("read.rs").expect("Unable to create file");
    println!("File created successfully");
}
