use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get("http://172.16.1.227:8080/file/f5cfc5e7-c6e9-4d92-99d4-2b7c9682cf6a/download")
        .await?
        .text()
        .await?;
    println!("\n{}", resp);
    Ok(())
}