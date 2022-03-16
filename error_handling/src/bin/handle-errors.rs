//! 在main中处理错误
//!
#[macro_use]
extern crate error_chain;

use error_chain::error_chain;

use std::fs::File;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        ParseInt(::std::num::ParseIntError);
    }
}

fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;

    Ok(uptime
        .split('.')
        .next()
        .ok_or("不能解析uptime数据")?
        .parse()?)
}

fn main() {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {} 秒", uptime),
        Err(err) => eprintln!("错误：{}", err),
    };
}
