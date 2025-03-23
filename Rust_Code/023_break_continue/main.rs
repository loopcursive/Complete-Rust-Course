fn main() {
    // Break
    for i in 1..=10 {
        if i == 6 {
            break;
        }
        println!("{}", i);
    }

    // continue
    for i in 1..=10 {
        if i == 8 {
            continue;
        }
        println!("{}", i);
    }
}
