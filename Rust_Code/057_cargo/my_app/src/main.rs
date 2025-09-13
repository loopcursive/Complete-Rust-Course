use rand::Rng;
fn main() {
    println!("Hello, world!");
    let mut rng = rand::rng();
    let i: u8 = rng.random();
    println!("{}", i);
    println!("{}", rng.random_range(1..=5));
}
