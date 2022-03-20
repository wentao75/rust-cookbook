#[macro_use]
extern crate error_chain;

use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        println!("下载文件：'{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("文件下载于：'{:?}'", fname);
        File::create(fname)?
    };

    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
