fn main() {
    // Q1 Write a Rust program to check if a number is positive, negative, or zero using if, else if, and  else.

    let num = 0;
    if num > 0 {
        println!("Positive number");
    } else if num < 0 {
        println!("Negative number");
    } else {
        println!("Zero");
    }

    // Q2 How to store the result of an if condition in a variable in Rust?

    let condition = false;
    let result = if condition { "true" } else { "false" };
    println!("{}", result);

    //  Q4  Write a Rust program that prints numbers from 1 to 10 using a for loop

    for i in 1..=10 {
        println!("{}", i);
    }

    // Q5 Create an infinite loop using loop and break it after 3 iterations.

    let mut count = 1;
    loop {
        if count > 3 {
            break;
        }
        println!("{}", count);
        count += 1;
    }
}
