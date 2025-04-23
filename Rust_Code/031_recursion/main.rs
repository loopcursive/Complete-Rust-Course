fn countdown(n: u64) {
    // Base case: stop the recursion when n reaches 0
    if n == 0 {
        println!("{}", n); // Print 0
    } else {
        println!("{}", n); // Print the current number
        countdown(n - 1); // Recursive call with n - 1
    }
}

fn main() {
    let start = 5;
    println!("Starting countdown from {}:", start);
    countdown(start);
}
