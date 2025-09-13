macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

macro_rules! say_hello_1 {
    () => {
        println!("null");
    };

    ($name:expr) => {
        println!("Hello from {}", $name);
    };
}

macro_rules! say_hello_2 {
    ($($name:expr), *) => {
        $(println!("Hello from {}", $name);)* 
    };
}






fn main(){
    say_hello!();
    say_hello_1!("fjkhgfkjh");
    say_hello_2!("Abhi" , "Abhishek1" , 123);
}