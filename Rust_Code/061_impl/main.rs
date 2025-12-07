struct Person {
    age: u8,
}

impl Person {
    fn print_age(&self){
        println!("Age: {}", self.age);
    }
}

impl Person {
    fn age_increase(&mut self){
        self.age += 1;
    }
}

impl Person {
    fn die(self){
        println!("Age: {} died", self.age);
    }
}



fn main(){
    let person_1 = Person {
        age: 20,
    };
    person_1.print_age();

    let mut person_2 = Person {
        age: 10,
    };
    person_2.print_age();
    person_2.age_increase();
    person_2.print_age();

    person_2.die();
    person_2.print_age();

    
}