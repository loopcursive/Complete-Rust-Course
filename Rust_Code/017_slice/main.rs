fn main() {
    // slicing
    let s = String::from("Number");
    println!("{}", s);

    let slice = &s[0..4];
    println!("{}", slice);

    // OMITTING START
    let s2 = String::from("Boolean.Float");
    let slice1 = &s2[..5];
    println!("{}", slice1);

    // OMITTING END INDEX
    let slice9 = &s2[7..];
    println!("{}", slice9);

    //  accessing single char from string using method 1
    let name = "Abhi";
    let name_single_char = &name[1..2];
    println!("{}", name_single_char);

    //  accessing single char from string using method 2
    let name_1 = "Pro";
    let name_1_single_char = name_1.chars().nth(0).unwrap();
    println!("{}", name_1_single_char);

    //  assigning new string value
    let mut name_2 = String::from("Prop");
    println!("{}", name_2);
    name_2.clear();
    name_2.push_str("Prop2");
    println!("{}", name_2);
}
