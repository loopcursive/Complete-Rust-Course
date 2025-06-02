fn print_vector(v: &Vec<i32>) {
    println!("Vector = {:?}", v);
}

fn create_vector() -> Vec<i32> {
    vec![10, 20, 30]
}

fn main() {
    //   EXPLICITLY DECLARE TYPE OF VECTOR
    let v: Vec<u8> = vec![1, 2, 3];
    println!("{:?}", v);

    //   EMPTY VECTOR
    let mut v2: Vec<u16> = Vec::new();
    println!("{:?}", v2);
    v2.push(12);
    println!("{:?}", v2);

    //   PASSING VECTOR TO FUNCTION
    let numbers = vec![1, 2, 3, 4, 5];
    print_vector(&numbers);
    println!("{:?}", numbers);

    //   RETURNING VECTOR FROM FUNCTION
    let num = create_vector();
    println!("Returned = {:?}", num);
}
