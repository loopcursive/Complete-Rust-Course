fn main(){
    // For loop
    for i in 1..=10{
        println!("{}", i);
    }


    // While loop
    let mut num = 1;
    while num <= 10{
        println!("{}", num);
        num += 1;
    }

    // loop -> infinite loop

    let mut count = 1;

    loop{

        println!("{}", count);
        count += 1;

        if count > 5{
            break;
        }


    }


}