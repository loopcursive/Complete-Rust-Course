use reqwest;
use tokio;
#[tokio::main]
async fn main() {
    match fetch_data().await {
        Ok(data) => println!("Response:\n{}", data),
        Err(e) => println!("Error: {}", e),
    }
}

async fn fetch_data() -> Result<String, reqwest::Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = reqwest::get(url).await?;
    // Send GET request
    let body = response.text().await?;
    // Get response body as text
    Ok(body)
}

