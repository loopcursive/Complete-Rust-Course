fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Can't divide by zero!".to_string())
        // Err(100)
    } else {
        Ok(a / b)
    }
 }

fn main() {
    let result = divide(10, 0);
    // match result {
    //     Ok(answer) => println!("Answer is {}", answer),
    //     Err(error) => println!("Error: {}", error),
    // }
    // result.unwrap();
    // result.unwrap_err();
    
   let a =  result.is_ok();
   println!("a: {}", a);
   let b =  result.is_err();
   println!("b: {}", b);

    println!("Hello, world!");
 }

