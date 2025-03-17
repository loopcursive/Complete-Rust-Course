fn main() {
    // len() method returns the length of string
    let s1 = String::from("Abhishek");
    println!("{}", s1.len());

    // is_empty() method returns true if string is empty
    let s2 = String::new();
    println!("{}", s1.is_empty()); // false
    println!("{}", s2.is_empty()); // true

    // push_str() method appends a string slice to the end of string
    let mut s3 = String::from("Abhishek");
    s3.push_str(" is a good person");
    println!("{}", s3);

    // push() method appends a single character to the end of string
    let mut s4 = String::from("Abhishek");
    println!("{}", s4);
    s4.push('1');
    println!("{}", s4);

    // insert() method inserts a character at a given index
    let mut s5 = String::from("Abhishek is a good person");
    println!("{}", s5);
    s5.insert(5, '1');
    println!("{}", s5);

    // insert_str() method inserts a string slice at a given index
    let mut s6 = String::from("Abhishek is a good person");
    println!("{}", s6);
    s6.insert_str(5, "Pro");
    println!("{}", s6);

    // trim() method removes leading and trailing whitespace
    let mut s7 = String::from("          Abhishek");
    println!("{}", s7);
    println!("{}", s7.trim());

    // .contains() method checks if a string contains a given substring
    let mut s8 = String::from("Abhishek is a good person");
    println!("{}", s8);
    println!("{}", s8.contains("is"));

    // starts_with() method checks if a string starts with a given substring
    let mut s9 = String::from("Abhishek is a good person");
    println!("{}", s9);
    println!("{}", s9.starts_with("A"));

    // ends_with() method checks if a string ends with a given substring
    let mut s10 = String::from("Abhishek is a good person");
    println!("{}", s10);
    println!("{}", s10.ends_with("n"));

    // to_uppercase() method converts a string to uppercase
    let mut s11 = String::from("abhiSHEK is a good person");
    println!("{}", s11);
    println!("{}", s11.to_uppercase());

    // to_lowercase() method converts a string to lowercase
    let mut s12 = String::from("abhiSHEK is a good person");
    println!("{}", s12);
    println!("{}", s12.to_lowercase());

    // to_string() method converts a type to a string
    let mut s13 = 100;
    println!("{}", s13.to_string() == "100");
}


