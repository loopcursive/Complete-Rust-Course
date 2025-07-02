use std::fs::File;
use std::io::Read;

fn main(){
    let mut _data_file_1 = File::open("learn.txt").unwrap();
    let mut _data = String::new();
    _data_file_1.read_to_string(&mut _data).unwrap();
    println!("{:?}", _data);

}