fn main() {
    // Interger to Integer | large -> small
    let large_number: i32 = 50;
    let small_number: u8 = large_number as u8;
    println!("Integer to Integer: {}", small_number);

    // Interger to Integer | small -> large
    let large_number_1: i8 = 120;
    let small_number_1: i16 = large_number_1 as i16;
    println!("Integer to Integer: {}", small_number_1);

    // Interger to Integer | signed -> unsigned
    let large_number_2: i32 = -50;
    let small_number_2: u32 = large_number_2 as u32;
    println!("Integer to Integer: {}", small_number_2);

    //  Integer to float
    let integer: i32 = 42;
    let floating: f64 = integer as f64;
    println!("Integer to Floating-Point: {}", floating);

    // Float to integer
    let float: f64 = 3.99;
    let integer: i32 = float as i32;
    println!("Floating-Point to Integer: {}", integer);

    // Float to float
    let double_precision: f64 = 3.141592653589793;
    let single_precision: f32 = double_precision as f32;
    println!("Double precision (f64): {}", double_precision);
    println!("Single precision (f32): {}", single_precision);

    // Char to integer
    let character: char = 'A';
    let code_point: u32 = character as u32;
    println!("Character: {}", character);
    println!("Unicode code point (int): {}", code_point);

    // Integer to char
    let code_point: u8 = 65;
    let character: char = code_point as char;
    println!("Unicode code point (int): {}", code_point);
    println!("Character: {}", character);

    // Boolean to integer
    let my_bool: bool = true;
    let my_int: u8 = my_bool as u8;
    println!("my_bool: {}, my_int: {}", my_bool, my_int);
    let my_bool2: bool = false;
    let my_int2: u8 = my_bool2 as u8;
    println!("my_bool2: {}, my_int2: {}", my_bool2, my_int2);
}
