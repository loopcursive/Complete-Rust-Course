use std::io;
fn main() {
    // let age = 19;
    // let name = "Abhishek";

    // println!("My name is {} and my age is {}", name, age);

    let mut name = String::new();
    println!("Please enter: ");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let age: i32 = name.trim().parse().expect("Please type a number!");

    println!("Your is: {}", age);
}
