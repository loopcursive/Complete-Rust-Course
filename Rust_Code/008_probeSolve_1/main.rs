fn main() {
    // Q1 .What will be the output of the following Rust code?
    println!("{} + {} = {}", 5, 10, 5 + 10);

    // Q2 What are the two types of comments in Rust? Provide an example of each.

    // Signle line comment
    /* Multi line comment */

    // Q3 What is the difference between mutable and immutable variables in Rust? Provide an  example.

    // immutable variable
    let x = 12;
    println!("x is {}", x);
    // x = 123;
    // println!("x is {}", x);

    let mut y = 12;
    println!("y is {}", y);
    y = 123;
    println!("y is {}", y);

    // Q4  What will happen if you try to modify an immutable variable in Rust?
    let z = 12;
    println!("z is {}", z);
    // z = 123;
    // println!("z is {}", z);
}
