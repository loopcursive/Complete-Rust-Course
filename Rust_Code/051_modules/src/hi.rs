pub fn greet_morning() {
    println!("Good Morning");
}

pub fn greet_afternoon() {
    println!("Good Afternoon");
}

pub fn greet_evening() {
    println!("Good Evening");
}

fn pvt_greet_night() {
    println!("Good Night");
}

pub fn greet_night() {
    pvt_greet_night();
}
