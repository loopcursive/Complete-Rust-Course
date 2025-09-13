use std::fs;
fn main() {
    // using match
    /*  let result = fs::read_to_string("index.html");
    match result {
        Ok(content_1) => println!("File content:\n{}", content_1),
        Err(error) => println!("Failed to read file: {}", error),
    }
    println!("Hello, world!");*/

    // USING IF LET
    /*  if let Ok(file) = fs::read_to_string("index.html") {
        println!("File opened!{}", file);
    } else {
        println!("Could not open file.");
    }
    println!("Hello, world!")*/

    // .UNWRAP()
    /*   let file = fs::read_to_string("index1.html").unwrap();
    println!("File opened!{}", file);
    println!("Hello, world!"); */

    // .EXPECT()
    let file = fs::read_to_string("index1.html").expect("Failed to open file");
    println!("File opened!{}", file);
    println!("Hello, world!");
}
