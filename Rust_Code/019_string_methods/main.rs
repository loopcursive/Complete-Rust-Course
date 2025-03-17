fn main() {
    let s1 = String::from("Abhishek");
    println!("{}", s1);

    // len() methods
    println!("{}", s1.len());

    // is_empty() method
    let s2 = String::new();
    println!("{}", s1.is_empty());
    println!("{}", s2.is_empty());

    // push_str() method
    let mut s3 = String::from("Abhishek");
    s3.push_str(" is a good person");
    println!("{}", s3);

    // push() method
    let mut s4 = String::from("Abhishek");
    println!("{}", s4);
    s4.push('1');
    println!("{}", s4);

    // insert() method
    let mut s5 = String::from("Abhishek is a good person");
    println!("{}", s5);
    s5.insert(5, '1');
    println!("{}", s5);

    // insert_str() method
    let mut s6 = String::from("Abhishek is a good person");
    println!("{}", s6);
    s6.insert_str(5, "Pro");
    println!("{}", s6);

    // trim() method
    let mut s7 = String::from("          Abhishek");
    println!("{}", s7);
    println!("{}", s7.trim());

    // .contains() method
    let mut s8 = String::from("Abhishek is a good person");
    println!("{}", s8);
    println!("{}", s8.contains("is"));

    // starts_with() method
    let mut s9 = String::from("Abhishek is a good person");
    println!("{}", s9);
    println!("{}", s9.starts_with("A"));

    // ends_with() method
    let mut s10 = String::from("Abhishek is a good person");
    println!("{}", s10);
    println!("{}", s10.ends_with("n"));

    // to_uppercase() method
    let mut s11 = String::from("abhiSHEK is a good person");
    println!("{}", s11);
    println!("{}", s11.to_uppercase());

    // to_lowercase() method
    let mut s12 = String::from("abhiSHEK is a good person");
    println!("{}", s12);
    println!("{}", s12.to_lowercase());

    // to_string() method
    let mut s13 = 100;
    println!("{}", s13.to_string() == "100");
}
