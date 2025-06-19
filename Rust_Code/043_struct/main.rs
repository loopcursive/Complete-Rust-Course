struct Person{
    name:String ,
    age:i32
}

fn main(){
    let mut person_1 = Person{name:"Abhishek".to_string(),age:20};
    println!("{}",person_1.name);
    println!("{}",person_1.age);
    person_1.age = 10;
    println!("{}",person_1.age);

    let Person { name, age } = person_1;
    println!("{}",name);
    println!("{}",age);
}