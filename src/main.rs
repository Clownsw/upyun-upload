use std::env;

use reqwest::{Client, StatusCode};

pub static API_URL: &str = "http://v0.api.upyun.com/";

async fn init() {
    dotenv::dotenv().ok().unwrap();
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init().await;

    let request_url = format!("{}{}/{}", API_URL, "smile-uyun", "aaa");

    let resp = Client::new()
        .post(request_url.as_str())
        .basic_auth(env::var("operator")?, Some(env::var("password")?))
        .send()
        .await?;

    if resp.status() == StatusCode::OK {
        println!("成功!");
    } else {
        println!("失败");
    }

    Ok(())
}
