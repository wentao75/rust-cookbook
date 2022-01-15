//! # 产生随机数值
//!
//!
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

fn main() {
    generate_random_numbers();
    generate_random_numbers_within_range();
}

/// 使用`rand::Rng`这个随机数生成器来获得随机数，它通过`rand::thread_rng`生成。
/// 每个线程会有一个初始化的生成器：整数在整个类型空间均匀分布，浮点数在[0, 1)区间均匀分布（不包括1）
fn generate_random_numbers() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("随机数 u8: {}", n1);
    println!("随机数 u16: {}", n2);
    println!("随机数 u32: {}", rng.gen::<u32>());
    println!("随机数 i32: {}", rng.gen::<i32>());
    println!("随机数 float: {}", rng.gen::<f64>());
}

/// # 在指定区间获取随机数
/// 指定区间使用半开放区间 [0, 10)（不包括10），使用`Rng::gen_range`.
fn generate_random_numbers_within_range() {
    let mut rng = rand::thread_rng();
    println!("整数：{}", rng.gen_range(0..10));
    println!("浮点数：{}", rng.gen_range(0.0..10.0));

    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("掷骰子：{}", throw);

        if throw == 6 {
            break;
        }
    }
}
