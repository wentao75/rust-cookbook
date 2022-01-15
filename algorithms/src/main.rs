//! # 产生随机数值
//!
//!
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

fn main() {
    generate_random_numbers();
    generate_random_numbers_within_range();
}

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
