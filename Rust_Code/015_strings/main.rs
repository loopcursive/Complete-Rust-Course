fn main() {
    // String Literal
    let my_name: &str = "Abhishek";
    println!("{}", my_name);

    // String Object
    let mut my_name_2 = String::new();
    // println!("{}", my_name_2);
    my_name_2.push_str("lopcursive");
    println!("{}", my_name_2);

    let mut my_name_3 = String::from("There is a car");
    println!("{}", my_name_3);

    my_name_3.push_str(" in front of me");
    println!("{}", my_name_3);
}
