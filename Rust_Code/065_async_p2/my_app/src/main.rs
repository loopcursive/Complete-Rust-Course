// use tokio::time::{Duration, sleep};

// #[tokio::main]
// async fn main() {
//     let start = std::time::Instant::now();
//     println!("Main function start");

    
//     let result =  tokio::spawn(
//         async {
//             sleep(Duration::from_secs(4)).await;
//             println!("Background task done");
//        }); 


//     println!("Main function continues");
//     sleep(Duration::from_secs(3)).await;
//     println!("Main function done");
    
//     result.await.unwrap();

//     let duration = start.elapsed();
//     println!("Total time: {:?}", duration);
    
// }


use tokio::time::{Duration, sleep};
#[tokio::main]
async fn main() {
    match get_data(true).await {
        Ok(data) => println!("Success: {}",data),
        Err(err) => println!("Error: {}", err),
    }
    match get_data(false).await {
        Ok(data) => println!("Success: {}",data),
        Err(err) => println!("Error: {}", err),
    }
}

async fn get_data(success: bool) -> Result<String, String> {
    sleep(Duration::from_secs(1)).await;
    if success {
        Ok("Here is your data".to_string())
    } else {
        Err("Something went wrong".to_string())
    }
}
