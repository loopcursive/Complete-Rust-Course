use std::thread;
use std::time::Duration;

fn main(){

    let t1 = thread::spawn(||{
        for i in 1..=6{
        println!("From thread -> {}",i);
        thread::sleep(Duration::from_millis(100));
    }
    });

    let t2 = thread::spawn(||{
        for i in 1..=10{
        println!("From thread-2 -> {}",i);
        thread::sleep(Duration::from_millis(100));
    }
    });



    

    for i in 1..=8{
        println!("From 2 -> {}",i);
        thread::sleep(Duration::from_millis(100));
    }

    t1.join().unwrap();
    t2.join().unwrap();

}