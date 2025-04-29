fn add() {
    let add = 12;
    println!("Add function -> {}", add);
}

fn main() {
    let a = 12;
    println!("{}", a);
    {
        let b = 20;
        println!("{}", b);
    }
    println!("{}", a);

    let rank = 1;
    println!("{}", rank);
    let rank = 2;
    println!("{}", rank);
    add();

    // println!("{}", add);
}
