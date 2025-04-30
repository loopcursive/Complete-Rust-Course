// fn add(){
//     a+b
// }

fn main() {
    // define closure
    let add = |a, b| a + b;

    // call closure
    println!("{}", add(2, 3));

    // closure without parameter
    let greet = || println!("Hello World");
    greet();

    // Multi line closure
    let process_numbers = |a, b| {
        let sum = a + b;
        let product = a * b;
        sum + product
    };

    println!("{}", process_numbers(2, 3));

    // Closure and types
    let add_1 = |a: i32, b: i32| a + b;
    println!("{}", add_1(2, 3));
}
