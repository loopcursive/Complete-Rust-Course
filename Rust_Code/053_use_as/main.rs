mod greet {
    pub fn greet_morning() {
        println!("Good Morning");
    }
    pub fn greet_afternoon() {
        println!("Good Afternoon");
    }
}

mod database {
    pub fn connect() {
        println!("Connected to the database!");
    }
}

mod network {
    pub fn connect() {
        println!("Connected to the network!");
    }
}

mod inner;

use greet::greet_afternoon;
use greet::greet_morning;

use inner::greet_afternoon_inner;
use inner::greet_morning_inner;

use database::connect as db_connect;
use network::connect as net_connect;

fn main() {
    greet::greet_morning();
    greet::greet_afternoon();

    greet_morning();
    greet_afternoon();

    inner::greet_morning_inner();
    inner::greet_afternoon_inner();

    greet_morning_inner();
    greet_afternoon_inner();

    database::connect();
    network::connect();

    db_connect();
    net_connect();
}
