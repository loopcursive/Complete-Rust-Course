fn main() {
    //  using + operator
    let s1 = String::from("Hello ");
    let s2 = String::from("world!");

    let s3 = s1 + &s2;
    println!("{}", s3);
    // println!("{}", s1); // ownership tranfer
    println!("{}", s2);

    //  using format macro
    let s4 = String::from("Pro ");
    let s5 = String::from("loop");

    let s6 = format!("{}{}", s4, s5);
    println!("{}", s6);
    println!("{}", s4);
    println!("{}", s5);

    //  using push() and push_str()
    let mut s7 = String::from("One");
    let mut s8 = String::from("Two");

    println!("{}", s7);
    s7.push('%');
    println!("{}", s7);

    println!("{}", s8);
    s8.push_str(" Three");
    println!("{}", s8);

    // Type caste int to string
    let number = 123;
    println!("{}", number);

    let number_to_str = number.to_string();
    println!("{}", number_to_str);
    println!("{}", number_to_str == "2020");
}
