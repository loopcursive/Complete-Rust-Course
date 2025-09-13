mod outer {

   pub mod inner_1 {
        pub fn greet_1_inner(){
            println!("Hello from inner_1");
        }
    }

   pub mod inner_2 {
        pub fn greet_2_inner(){
            println!("Hello from inner_2");
        }
    }

}

fn main(){
    outer::inner_2::greet_2_inner();
    outer::inner_1::greet_1_inner();
}