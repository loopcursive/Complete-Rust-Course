//  declare enum
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

#[derive(Debug)]
enum Login_screen {
    Username(String),
    Password(i32),
}

fn main() {
    // use enum
    let red = TrafficLight::Yellow;
    println!("{:?}", red);

    // Match
    match red {
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Wait"),
        TrafficLight::Green => println!("Go"),
    }

    // Enum with data type
    let username = Login_screen::Username(String::from("Abhishek"));
    println!("{:?}", username);

    let password = Login_screen::Password(123456);
    println!("{:?}", password);

    match password {
        Login_screen::Username(username) => println!("Username: {}", username),
        Login_screen::Password(password) => println!("Password: {}", password),
    }
}
