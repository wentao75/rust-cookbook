//! # Clap basic
//! 解析命令行参数，使用`clap`包做命令行参数解析。
//! 在构建器中，`short`和`long`分别控制用户期望输入的参数，`short`表示类似`-f`的参数，`long`类似`--file`。
use clap::{App, Arg};

fn main() {
    let matches = App::new("我的测试程序")
        .version("0.1.0")
        .author("Wen Tao <software.wentao@gmail.com>")
        .about("Rust Cookbook for argument parsing of command line arguments")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("A cool file"),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("number")
                .takes_value(true)
                .help("Five less than your favorite number"),
        )
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("传入文件：{}", myfile);
    let num_str = matches.value_of("num");
    match num_str {
        None => println!("无法知道你的幸运数字！"),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("你的幸运数字一定是：{}", n + 5),
            Err(_) => println!("那不是一个数字！{}", s),
        },
    }
}
