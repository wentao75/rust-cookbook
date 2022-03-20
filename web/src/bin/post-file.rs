#[macro_use]
extern crate error_chain;

use error_chain::error_chain;
use std::fs::File;
use std::io::Read;

error_chain! {
    foreign_links {
        HttpRequest(reqwest::Error);
        IoError(::std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let paste_api = "https://paste.rs";
    let mut file = File::open("output.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let client = reqwest::Client::new();
    let res = client.post(paste_api).body(contents).send().await?;
    let response_text = res.text().await?;
    println!("上传结果位于：{}", response_text);
    Ok(())
}
