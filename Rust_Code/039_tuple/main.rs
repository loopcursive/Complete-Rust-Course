fn main() {
    //   TUPLE WITHOUT DATA TYPE
    let tup = ("Abhishek", 20, 3.3);
    println!("{:?}", tup);
    println!("{}", tup.0);
    println!("{}", tup.2);

    //   TUPLE WITH DATA TYPE
    let tup_1: (&str, i32, f32) = ("Kumar", 30, 6.9);
    println!("{:?}", tup_1);
    println!("{}", tup_1.0);
    println!("{}", tup_1.2);

    //   MUTABLE TUPLE
    let mut tup_2: (&str, i32, f32) = ("Kumar", 30, 6.9);
    println!("{:?}", tup_2);
    tup_2.0 = "fdjhgfdhg";
    println!("{:?}", tup_2);

    //   DESTRUCTURING A TUPLE
    let tup_4 = (12, 34, 56);
    let (a, b, c) = tup_4;

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);

    println!("{:?}", tup_4);
}
