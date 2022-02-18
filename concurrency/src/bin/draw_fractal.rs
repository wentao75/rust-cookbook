//! # 并发编程
//! 这个部分主要分为两个部分，第一是线程，第二是并发执行（主要是数据的并行处理）
//! 线程使用了`crossbeam`库，并发处理使用了`rayon`库

// 这里使用了`error_chain`库，统一完成错误处理模式，通过error_chain!宏定义引入，后续按照规则使用
#[macro_use]
extern crate error_chain;

use image::{ImageBuffer, Pixel, Rgb};
use num::complex::Complex;
use std::sync::mpsc::{channel, RecvError};
use threadpool::ThreadPool;

error_chain! {
    foreign_links {
        MpscRecv(RecvError);
        Io(std::io::Error);
    }
}

fn main() {
    if let Err(ref e) = draw_fractal_dispatching_work_to_a_threadpool() {
        println!("绘制分型图错误：{}", e);
        for e in e.iter().skip(1) {
            println!("错误原因：{}", e);
        }
    }
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

    let (r, g, b) = (
        normalize(r, factor),
        normalize(g, factor),
        normalize(b, factor),
    );
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
fn draw_fractal_dispatching_work_to_a_threadpool() -> Result<()> {
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
        pool.execute(move || {
            for x in 0..width {
                let i = julia(c, x, y, width, height, iterations);
                let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                tx.send((x, y, pixel)).expect("无法发送数据！");
            }
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }

    let _ = img.save("output.png").chain_err(|| "存储图片错误");
    Ok(())
}
