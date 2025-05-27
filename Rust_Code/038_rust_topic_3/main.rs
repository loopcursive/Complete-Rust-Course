fn print_array(arr: [i32; 3]) {
    println!("{:?}", arr);
}

fn return_array() -> [i32; 3] {
    [0, 6, 1]
}

fn main() {
    //   ITERATING AN ARRAY
    let number = [1, 2, 3, 4, 5, 6];
    println!("{:?}", number);

    for i in number {
        println!("{}", i);
    }

    //   PASSING ARRAY TO FUNCTION

    let newArray = [12, 34, 56];
    print_array(newArray);

    //   RETURNING ARRAY FROM FUNCTION
    println!("Array with return {:?}", return_array());
}
