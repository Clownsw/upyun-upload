use std::{env, fs::File, io::Read};

use chrono::Utc;
use reqwest::{Client, StatusCode};

pub static API_URL: &str = "http://v0.api.upyun.com/";

async fn init() {
    dotenv::dotenv().ok().unwrap();
}

async fn load_file(path: String) -> Result<String, anyhow::Error> {
    let mut path = File::open(path)?;
    let mut buffer = String::new();

    path.read_to_string(&mut buffer)?;

    Ok(buffer)
}

async fn remote_mkdir_directory(name: String) -> Result<(), anyhow::Error> {
    let request_url = format!("{}{}/{}", API_URL, "smile-uyun", name);

    let resp = Client::new()
        .post(request_url.as_str())
        .query(&[("folder", true)])
        .basic_auth(env::var("operator")?, Some(env::var("password")?))
        .send()
        .await?;

    if resp.status() == StatusCode::OK {
        println!("成功!");
    } else {
        println!("失败, resp text {}", resp.text().await?);
    }

    Ok(())
}

async fn remote_upload_file(file_content: String) -> Result<(), anyhow::Error> {
    let mut file_name = String::from("/sql_dump/dump_");
    file_name.push_str(Utc::now().to_string().as_str());
    file_name.push_str(".sql");

    let request_url = format!("{}{}/{}", API_URL, "smile-uyun", file_name);

    let resp = Client::new()
        .post(request_url.as_str())
        .basic_auth(env::var("operator")?, Some(env::var("password")?))
        .body(file_content.as_bytes().to_vec())
        .send()
        .await?;

    if resp.status() == StatusCode::OK {
        println!("成功!");
    } else {
        println!("失败, resp text {}", resp.text().await?);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init().await;

    let file_content = load_file("D:\\dump.sql".to_string()).await?;

    // remote_mkdir_directory("/aaa/".to_string()).await?;
    remote_upload_file(file_content).await?;

    Ok(())
}
