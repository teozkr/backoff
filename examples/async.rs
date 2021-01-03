extern crate tokio_1 as tokio;

use backoff::ExponentialBackoff;
use backoff::tokio::retry;

async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    retry(ExponentialBackoff::default(), async {
        println!("Fetching {}", url);
        Ok(reqwest::get(url).await?.text().await?)
    })
    .await
}

#[tokio::main]
async fn main() {
    match fetch_url("https://www.rust-lang.org").await {
        Ok(_) => println!("Successfully fetched"),
        Err(err) => panic!("Failed to fetch: {}", err),
    }
}