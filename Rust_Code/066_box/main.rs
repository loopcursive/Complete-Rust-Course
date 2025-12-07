struct Point{
    x: i32,
    y: i32,
}

fn main(){
    let a = 12;
    let b = 3;
    let c = Box::new(100);
    println!("{} and {} and {}", a, b , c);
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);

    let add = 10 + b;
    println!("{}",add);

    let add_1 = 10 + *c;
    println!("{}",add_1);

    let p1 = Box::new(Point{x: 10, y: 20}); 
    println!("{} and {}", p1.x, p1.y);


}