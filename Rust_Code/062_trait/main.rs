struct Person {
    rank: u8,
    age: u8
}

trait MyRules{
    fn println_rank(&self);
}

impl Person {
    fn println_age(&self){
        println!("Age: {}", self.age);
    }
}

impl MyRules for Person {
    fn println_rank(&self){
        println!("Rank: {}", self.rank);
    }
}

fn main(){
    let abhishek = Person {
        rank:10 ,
        age: 20
    };
    abhishek.println_age();

    abhishek.println_rank();
    abhishek.println_rank_1();
}