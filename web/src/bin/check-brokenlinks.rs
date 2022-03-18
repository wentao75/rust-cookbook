#[macro_use]
extern crate error_chain;

use error_chain::error_chain;
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use url::{Position, Url};

error_chain! {
  foreign_links {
      ReqError(reqwest::Error);
      IoError(std::io::Error);
      UrlParseError(url::ParseError);
      JoinError(tokio::task::JoinError);
  }
}

async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);
    let base_url =
        base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
    Ok(base_url)
}

async fn check_link(url: &Url) -> Result<bool> {
    let res = reqwest::get(url.as_ref()).await?;
    Ok(res.status() != StatusCode::NOT_FOUND)
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse("https://www.rust-lang.org/en-US/")?;
    let res = reqwest::get(url.as_ref()).await?.text().await?;
    let document = Document::from(res.as_str());
    let base_url = get_base_url(&url, &document).await?;
    let base_parser = Url::options().base_url(Some(&base_url));
    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();
    let mut tasks = vec![];

    for link in links {
        tasks.push(tokio::spawn(async move {
            if check_link(&link).await.unwrap() {
                println!("{} is ok", link);
            } else {
                println!("{} is broken", link);
            }
        }));
    }

    for task in tasks {
        task.await?
    }

    Ok(())
}
