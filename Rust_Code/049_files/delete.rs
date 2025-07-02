use std::fs;

 fn main(){
    fs::remove_file("delete.txt").expect("Unable to delete file");
    println!("File deleted successfully");
 }
