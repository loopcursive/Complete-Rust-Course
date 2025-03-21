fn main() {
    // match expression
    let num = 5;

    let day = match num {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        // 6 => "Saturday",
        // 7 => "Sunday",
        6 | 7 => "Weekend",
        _ => "Invalid day",
    };

    println!("{}", day);
}
