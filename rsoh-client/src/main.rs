#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::body;
use hyper::{Client};


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let url = String::from("http://api.ipify.org?format=json");

    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        // TODO: HTTPS
        println!("This PoC only works with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}


async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();

    let res = client.get(url).await?;

    println!("Response: {}", res.status());

    let body_bytes = body::to_bytes(res.into_body()).await?;
    let body = String::from_utf8(body_bytes.to_vec()).expect("response was not valid utf-8");
    println!("Body: {}", body);

    Ok(())
}