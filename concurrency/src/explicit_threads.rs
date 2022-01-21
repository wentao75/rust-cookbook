//! # 并发编程
//! 这个部分主要分为两个部分，第一是线程，第二是并发执行（主要是数据的并行处理）
//! 线程使用了`crossbeam`库，并发处理使用了`rayon`库
extern crate crossbeam;
extern crate crossbeam_channel;
use crossbeam_channel::{bounded, unbounded};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};
use std::sync::mpsc::{channel, RecvError};
use threadpool::ThreadPool;
use num::complex::Complex;
use image::{ImageBuffer, Pixel, Rgb};
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use walkdir::WalkDir;
// use crate::parallel_tasks;

error_chain! {
    foreign_links {
        MpscRecv(RecvError);
        Io(std::io::Error);
    }
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

// Function converting intensity values to RGB
// Based on http://www.efg2.com/Lab/ScienceAndEngineering/Spectra.htm
fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
    let wave = wavelength as f32;

    let (r, g, b) = match wavelength {
        380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
        440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
        490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
        510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
        580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
        645..=780 => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0),
    };

    let factor = match wavelength {
        380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
        701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
        _ => 1.0,
    };

    let (r, g, b) = (normalize(r, factor), normalize(g, factor), normalize(b, factor));
    Rgb::from_channels(r, g, b, 0)
}

// Maps Julia set distance estimation to intensity values
fn julia(c: Complex<f32>, x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
    let width = width as f32;
    let height = height as f32;

    let mut z = Complex {
        // scale and translate the point to image coordinates
        re: 3.0 * (x as f32 - 0.5 * width) / width,
        im: 2.0 * (y as f32 - 0.5 * height) / height,
    };

    let mut i = 0;
    for t in 0..max_iter {
        if z.norm() >= 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }
    i
}

// Normalizes color intensity values within RGB range
fn normalize(color: f32, factor: f32) -> u8 {
    ((color * factor).powf(0.8) * 255.) as u8
}

/// # 使用线程池绘制分形图
/// 这个例子使用线程池分布式计算绘制一个分形图，分形图来自[Julia set](https://en.wikipedia.org/wiki/Julia_set)。
/// 朱莉亚集合是一个在复平面上形成分形的点的集合。以法国数学家Gaston Julia的名字命名。
/// 
/// 定义：f_c(z) = z^2+c
/// 对于固定的复数c，取某一z值（如z=z_0)，可以得到序列 z_0, f_c(z_0),f_c(f_c(z_0)),...
/// 这一序列可能发散于无穷大或始终处于某一范围之内并收敛于某一值。我们将使其不扩散的z值的集合称为朱莉亚集合。
/// 
pub fn draw_fractal_dispatching_work_to_a_threadpool() -> Result<()> {
    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);

    let iterations = 300;

    // 可用的c取值：
    // 0.285+0.01i
    // 0.45+0.1428i
    // -0.70176-0.3842i
    // -0.835-0.2321i
    // -0.8i
    // -0.7269+0.1889i
    // 0.285+0i
    // let c = Complex::new(-0.8, 0.156);
    // let c = Complex::new(0.285, 0.01);
    let c = Complex::new(-0.7269, 0.1889);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || for x in 0..width {
            let i = julia(c, x, y, width, height, iterations);
            let pixel = wavelength_to_rgb(380+i*400/iterations);
            tx.send((x, y, pixel)).expect("无法发送数据！");
        });
    }

    for _ in 0..(width*height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }

    let _ = img.save("output.png").chain_err(|| "存储图片错误");
    Ok(())
}