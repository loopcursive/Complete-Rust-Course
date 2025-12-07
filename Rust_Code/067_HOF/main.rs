fn main(){
    // println!("{}", sqaure_and_add(square, 2, 3));

    let result = get_adder();
    println!("{}", result(5));
}
 
 /* 
fn square(x: i32)-> i32 {
    x * x
}

fn sqaure_and_add(op: fn(i32) -> i32, x: i32, y: i32) -> i32{
    op(x) + y
}
    */

fn add_one(x: i32)-> i32{
    x + 1
}

fn get_adder()-> fn(i32) -> i32{
    add_one
}





