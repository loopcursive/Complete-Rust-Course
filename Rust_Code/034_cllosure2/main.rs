fn main() {
    // CLOSURE ENVIORNMENT CAPTURING
    let x = 12;
    let add = |y| x + y;
    println!("{}", add(10));

    // CLOSURE BY MOVE
    let s = String::from("Hello");

    let take_ownership = move || {
        println!("Owned string: {}", s);
    };
    take_ownership();
    //  println!("{}", s);
}
