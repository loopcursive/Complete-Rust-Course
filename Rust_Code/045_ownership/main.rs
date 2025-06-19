fn main() {
    // Transfer and clone
    let s1 = String::from("hello");
    println!("{}", s1);

    let s2 = s1.clone();
    println!("{}", s2);
    println!("{}", s1);

    // COPY
    let x = 5;
    let y = x;
    println!("x = {}", x);
    println!("y = {}", y);
}
