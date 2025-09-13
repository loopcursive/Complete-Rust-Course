use my_app::{lib_fn , hi , users};

fn main(){
    println!("Hello from server");
    lib_fn();
    hi::hi_fn();
    users::user_1::fn_from_user_1();
    users::user_2::fn_from_user_2();
}