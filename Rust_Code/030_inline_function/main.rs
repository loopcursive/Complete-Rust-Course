// #[inline(always)]
// #[inline(never)]

#[inline]
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let x = square(5);
    println!("{}", x);
}
