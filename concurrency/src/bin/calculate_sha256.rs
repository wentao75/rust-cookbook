//! # 并发编程
//! 这个部分主要分为两个部分，第一是线程，第二是并发执行（主要是数据的并行处理）
//! 线程使用了`crossbeam`库，并发处理使用了`rayon`库

// 这里使用了`error_chain`库，统一完成错误处理模式，通过error_chain!宏定义引入，后续按照规则使用
#[macro_use]
extern crate error_chain;

use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use walkdir::WalkDir;

error_chain! {}

// mod explicit_threads;

fn main() {
    if let Err(ref e) = calculate_sha256_of_isofile() {
        println!("计算文件散列值错误: {}", e);
        for e in e.iter().skip(1) {
            println!("错误原因：{}", e);
        }
    }
}

// Verify the iso extension
fn is_iso(entry: &Path) -> bool {
    match entry.extension() {
        Some(e) if e.to_string_lossy().to_lowercase() == "o" => true,
        _ => false,
    }
}

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P)> {
    let mut buf_reader = BufReader::new(File::open(&filepath).chain_err(|| "打开文件错误")?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer).chain_err(|| "读取文件错误")?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}

/// # 并发计算iso文件的散列值
/// 计算当前文件目录中iso结尾文件的SHA256散列值。
/// 一个线程池创建等同于系统核心数（通过`num_cpus::get`获得）的线程数量。
/// `Walkdir::new`读取当前目录并调用`execute`去执行散列计算。
pub fn calculate_sha256_of_isofile() -> Result<()> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    println!("开始执行");
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            // println!("过滤路径：{:?}", e.path());
            !e.path().is_dir() && is_iso(e.path())
        })
    {
        println!("查看路径：{:?}", entry.path().display());
        let path = entry.path().to_owned();
        let tx = tx.clone();
        pool.execute(move || {
            let digest = compute_digest(path);
            tx.send(digest).chain_err(|| "无法发送数据！");
        });
    }
    println!("执行结束！");
    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}
