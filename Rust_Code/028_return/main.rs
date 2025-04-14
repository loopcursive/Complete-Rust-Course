fn square(num: i32) -> i32 {
    // println!("Square of {} is {}", num,num * num);
    num * num
}

fn main() {
    let store = square(12);
    println!("{}", store);
}
