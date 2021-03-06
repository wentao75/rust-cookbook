//! # 并发编程
//! 这个部分主要分为两个部分，第一是线程，第二是并发执行（主要是数据的并行处理）
//! 线程使用了`crossbeam`库，并发处理使用了`rayon`库
extern crate crossbeam;
extern crate crossbeam_channel;
use crossbeam_channel::{bounded, unbounded};
// use image::{ImageBuffer, Pixel, Rgb};
use lazy_static::lazy_static;
// use num::complex::Complex;
// use ring::digest::{Context, Digest, SHA256};
// use std::fs::File;
// use std::io::{BufReader, Read};
// use std::path::Path;
// use std::sync::mpsc::{channel, RecvError};
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};
// use threadpool::ThreadPool;
// use walkdir::WalkDir;
// use crate::parallel_tasks;

error_chain! {
    // foreign_links {
    //     MpscRecv(RecvError);
    //     Io(std::io::Error);
    // }
}

/// # 创建短周期线程
/// 这里使用`crossbeam`库，这个库为并发和并行编程提供了数据结构和方法。
/// `Scope::spawn`创建一个局部线程用来保证在闭包终止前返回，并且可以从调用函数中引用数据。
pub fn spawn_short_lived_thread() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap();
        let max_r = thread_r.join().unwrap();

        max_l.max(max_r)
        // Some()
    })
    .unwrap()
}

/// # 创建并行通道
/// 下面的例子使用`crossbeam`和`crossbeam-channel`库创建并行通道，类似于MQ，有一个数据源和一个数据接收器，
/// 数据从源到接收器的过程中，两个工作线程并行处理。
///
/// 我们使用`crossbeam_channel::bounded`容量限定为1的有界通道。由于创建信息远快于处理速度，创建者必须在它自己的线程中。
/// 这也意味着创建者调用`[crossbeam_channel::Sender::send]`会被阻塞直到通道中的信息被消费线程处理。
/// 同时注意通道中的数据被第一个接收到的线程消费，因此信息被送到单一一个工作者而不是全部工作者。
///
/// 从通道通过迭代器`crossbeam_channel::Receiver::iter`方法读取消息会阻塞，直到读取下一条消息或者通道关闭。
/// 由于通道在`crossbeam::scope`中创建，我们必须手工通过`drop`关闭它以防止整个程序被阻塞在工作者for循环等待中。
/// 我们可以考虑在没有信息被送达时以信号的方式调用`drop`
pub fn create_parallel_pipeline() {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);

    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // 发送源
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("源，发送{}", i);
            }

            drop(snd1);
        });

        // 并行处理，2个工作线程
        for _ in 0..n_workers {
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());

            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                for msg in recvr.iter() {
                    println!("工作者{:?} 接收信息 {}", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }

        drop(snd2);

        for msg in rcv2.iter() {
            println!("消耗接收数据 {}", msg);
        }
    })
    .unwrap();
}

/// # 两个线程间传输数据
/// 下面的例子验证了在一个创建者和一个消费者（SPSC）环境下使用`crossbeam-channel`。
///
pub fn pass_data_between_two_threads() {
    let (snd, rcv) = unbounded();
    let n_msgs = 5;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
        });
    })
    .unwrap();

    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("收到信息：{}", msg);
    }
}

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT.lock().map_err(|_| "获取MutextGuard失败")?;
    db.push(fruit.to_string());
    Ok(())
}

/// # 保持全局可变状态
/// 使用`lazy_static.lazy_static`声明的全局状态创建一个全局可用的`static ref`，这需要一个`Mutex`来允许变更。
/// `Mutex`封装确保状态不能同时被多个线程访问，以阻止竞争条件。`MutexGuard`需要被获得去读或者改动存储在`Mutex`中的数据。
pub fn maintain_global_mutable_state() -> Result<()> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT.lock().map_err(|_| "获取MutexGuard失败")?;
        db.iter()
            .enumerate()
            .for_each(|(i, item)| println!("数据 {}: {}", i, item));
    }
    insert("grape")?;
    Ok(())
}
