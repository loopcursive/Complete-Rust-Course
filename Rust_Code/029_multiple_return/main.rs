fn add_product(num1: i32, num2: i32) -> (i32, i32, i32) {
    let sum = num1 + num2;
    let product = num1 * num2;
    (sum, product, sum + product)
}
fn main() {
    let (sum_1, product_1, random) = add_product(10, 20);
    println!("Sum is {} and Product is {}", sum_1, product_1);
    println!("{}", random);
}
