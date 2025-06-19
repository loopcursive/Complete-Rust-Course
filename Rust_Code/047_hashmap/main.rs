use std::collections::HashMap;

fn main() {
    // Declare HashMap
    let mut phone_book = HashMap::new();

    // Insert into Hashmap
    phone_book.insert("Abhishek", 1234);
    phone_book.insert("A1", 1234);
    phone_book.insert("A2", 1234);
    phone_book.insert("A3", 1234);
    phone_book.insert("A4", 1234);
    phone_book.insert("loop", 1254);
    // phone_book.insert("Loop" , "Abhishek");
    println!("{:#?}", phone_book);

    /*// Access Element - String way
    let number_abhishek =  phone_book.get("Abhishek").cloned().unwrap_or("Unkown");
    println!("{}", number_abhishek);*/

    // Access Element - normal way
    let number_abhishek = phone_book.get("Loop").copied().unwrap_or(0);
    println!("{}", number_abhishek);

    // Updaet Value
    phone_book.insert("Abhishek", 1000);
    println!("{:#?}", phone_book);

    // Remove Element
    phone_book.remove("Abhishek");
    println!("{:#?}", phone_book);

    // Loop through HashMap
    for (i, x) in &phone_book {
        println!("{} -> {}", i, x);
    }
}
