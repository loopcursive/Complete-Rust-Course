fn main(){
    //   CREATING  VECTOR
    let v = vec![10,20,30];
    println!("{:?}", v);

    //   ACCESS ELEMENTS OF VECTOR
    println!("{}", v[2]);

    //   ADDING ELEMENTS TO VECTOR
    let mut num = vec![12,34,56];
    println!("{:?}", num);
    num.push(100);
    println!("{:?}", num);

    //   REMOVE LAST ELEMENT OF VECTOR
    let mut num_1 = vec![12,34,56];
    println!("{:?}", num_1);
    num_1.pop();
    println!("{:?}", num_1);

    //   REMOVE ELEMENTS AT SPECIFIED INDEX
    let mut num_2 = vec!["one" , "two" , "three"];
    println!("{:?}", num_2);
    println!("{}", num_2[1]);
    num_2.remove(1);
    println!("{:?}", num_2);
    println!("{}", num_2[1]);

    //   ADD ELEMENTS AT SPECIFIED INDEX
    let mut num_3 = vec!["one" , "two" , "three"];
    println!("{:?}", num_3);
    num_3.insert(1, "four");
    println!("{:?}", num_3);

    //   LOOP THROUGH VECTOR
    let num_5 = vec![1,2,3,4,5];
    for i in num_5{
        println!("{}", i);
    }




    



}