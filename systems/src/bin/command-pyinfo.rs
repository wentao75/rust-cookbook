#[macro_use]
extern crate error_chain;

use error_chain::error_chain;

use std::collections::HashSet;
use std::io::Write;
use std::process::{Command, Stdio};

error_chain! {
    errors {CmdError}
    foreign_links {
        Io(std::io::Error);
        Utf8(std::string::FromUtf8Error);
    }
}

fn main() -> Result<()> {
    let mut child = Command::new("python")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or("子进程stdin没有捕获！")?
        .write_all(b"import this;copyright();credits();exit()")?;
    let output = child.wait_with_output()?;

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let words = raw_output
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("找到{}个唯一单词: ", words.len());
        println!("{:?}", words);
        Ok(())
    } else {
        let err = String::from_utf8(output.stderr)?;
        error_chain::bail!("外部命令失败：\n {}", err);
    }
}
