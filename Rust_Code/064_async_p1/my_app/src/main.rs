 use tokio::{time::{sleep, Duration}, join};

#[tokio::main]
async fn main(){

    let start = std::time::Instant::now();

    // println!("Fetching data");
    // let result = fetch_data(true).await;
    
    // match result {
    //     Ok(data) => println!("Data: {}", data),
    //     Err(err) => println!("Error: {}", err),
    // }

    // fetch_data(1).await;
    // fetch_data(2).await;
    
    join!(
        fetch_data(1),
    fetch_data(2)
    );
     

    let duration = start.elapsed();
    println!("Total time: {:?}", duration);

}

// async fn fetch_data(sucess: bool) -> Result<String, String>{
//     if sucess {
//             sleep(Duration::from_secs(2)).await;
//         Ok("Data fetched successfully".to_string())
//     } else {
//             sleep(Duration::from_secs(2)).await;
//         Err("Failed to fetch data".to_string())
//     }
// } 

async fn fetch_data(id: u32) {
    println!("Task {} started", id);
    sleep(Duration::from_secs(2)).await;
    println!("Task {} finished", id);
 }