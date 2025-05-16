fn main() {
    //  ARRAY WITHOUT DATA TYPE
    let number = [10, 20, 30];
    println!("Array {:?}", number);

    //   ARRAY WITH DATA TYPE
    let number_1: [i32; 4] = [10, 20, 30, 50];
    println!("Array {:?}", number_1);

    //   ARRAY WITH DEFAULT VALUES
    let number_2 = [6; 3];
    println!("Array {:?}", number_2);
}
