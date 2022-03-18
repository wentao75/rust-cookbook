//! 读取系统变量
//! 使用`std::env::var`可以读取系统设置的变量

use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let config_path = env::var("CONFIG").unwrap_or("/etc/myapp/config".to_string());

    let config: String = fs::read_to_string(config_path)?;
    println!("Config: {}", config);

    Ok(())
}
