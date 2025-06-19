fn main() {
    // BORROWING
    let mut str = String::from("hello world");
    let str_1 = &str;
    println!(" Second string = {}", str_1);
    println!(" First string = {}", str);
    // str_1.push_str(" loop");
    println!(" Second string = {}", str_1);
}
