fn main() {
    //  How can you convert a &str into a String?
    let s1 = "Abhishek";
    let s2 = String::from(s1);
    println!("{}", s2);

    // &s[..]

    let s3 = String::from("Abhishek is a good person");
    let s4 = &s3[..];
    println!("{}", s4);
}
