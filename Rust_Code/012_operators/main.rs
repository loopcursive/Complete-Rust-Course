fn main() {
    // 5 +
    // Arithematic operator -> + , - , * , / , %
    let a = 12.0;
    let b = 23.0;
    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a / b);
    println!("{}", 19 % 2);

    // Compound Assignment operator -> += , -= , *= , /= , %=
    let mut c = 10;
    println!("{}", c);
    c += 1;
    println!("{}", c);
    c -= 1;
    println!("{}", c);
    c *= 5;
    println!("{}", c);
    c /= 5;
    println!("{}", c);
    c %= 4;
    println!("{}", c);

    // Comparison Operator ->

    println!("{}", 5 > 6);
    println!("{}", 5 < 6);
    println!("{}", 5 <= 5);
    println!("{}", 5 >= 5);
    println!("{}", 5 == 10);
    println!("{}", 5 != 10);

    // Logical operator
    // &&

    println!("Logical && -> {}", true && true);
    println!("Logical && -> {}", true && false);
    println!("Logical && -> {}", false && false);

    // ||

    println!("Logical || -> {}", true || true);
    println!("Logical || -> {}", true || false);
    println!("Logical || -> {}", false || false);

    //  !
    println!("Logical ! -> {}", !true);
    println!("Logical ! -> {}", !false);
}
