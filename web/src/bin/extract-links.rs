//! 提取网页HTML中的所有链接
//! 使用`reqwest::get`读取一个网页，然后使用`Document::from_read`解析返回HTML文档。
//! 使用`find`查找所有a标签的链接，调用`filter_map`获取具有"href"属性的链接

#[macro_use]
extern crate error_chain;

use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        Io(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://www.rust-lang.org/en-US")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}
