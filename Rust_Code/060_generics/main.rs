use std::fmt::Display;

fn println_any_data(x: u8){
    println!("{}", x);
}

fn print_any_type<T: Display>(y: T){
    println!("{}", y);
}

fn print_any_type_2<T: Display , U: Display>(y: T , x: U){
    println!("{}", y);
    println!("{}", x);
}

struct Point<T , U>{
    x: T,
    y: U,
}



fn main(){
    println_any_data(10);
    print_any_type(10);

    print_any_type("Abhishek");
    print_any_type(-1.23);

    print_any_type_2("Abhi" , 89);
    print_any_type_2(12 , true);

    let p = Point{
        x: 10,
        y: 20
    };

    println!("{}", p.x);
    println!("{}", p.y);

    
    let p_1 = Point{
        x: "Abhi",
        y: 123
    };

    println!("{}", p_1.x);
    println!("{}", p_1.y);



}