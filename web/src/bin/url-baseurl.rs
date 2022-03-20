#[macro_use]
extern crate error_chain;

use error_chain::error_chain;

use url::Url;

error_chain! {
    foreign_links {
        UrlParse(url::ParseError);
    }
    errors {
        CannotBeABase
    }
}

fn main() -> Result<()> {
    let full = "https://github.com/rust-lang/cargo?asdf";

    let url = Url::parse(full)?;
    let base = base_url(url)?;

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of URL is: {}", base);

    Ok(())
}

fn base_url(mut url: Url) -> Result<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(Error::from_kind(ErrorKind::CannotBeABase));
        }
    }

    url.set_query(None);
    Ok(url)
}
