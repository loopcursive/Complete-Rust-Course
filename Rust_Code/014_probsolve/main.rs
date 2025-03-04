const MAX_USERS: u16 = 1000;
fn main() {
    // Output ->  (50 / (5 + 5)) * (8 - 3) + 2 * (4 - 1);
    let result = (50 / (5 + 5)) * (8 - 3) + 2 * (4 - 1);
    /*
    (50 / (5 + 5)) * (8 - 3) + 2 * (4 - 1);
    (50 /  10) *  5 + 2 * 3
    5 * 5 + 2 * 3
    25 + 2 * 3
    25 + 6
    31
    */
    println!("{}", result);

    let declare_boolean: bool = false;
    println!("{}", declare_boolean);

    let float = 12.344543535;
    let convert_f64_into_i32: i32 = float as i32;
    println!("{}", convert_f64_into_i32);

    let large_integr: i32 = 123894;
    let convert_largeint_into_smallint: i8 = large_integr as i8;
    println!("{}", convert_largeint_into_smallint);

    // 5 = 5
    println!("{}", !true);

    println!("5 == 10 -> {}", 5 == 5);

    println!("{}", MAX_USERS);

    let a = 10;
    let b = 20;

    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", 20 % 6);
}
