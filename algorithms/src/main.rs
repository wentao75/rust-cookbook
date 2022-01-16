//! # 产生随机数值
//!
//!
use rand::distributions::Uniform;
use rand::thread_rng;
use rand::Rng;

fn main() {
    generate_random_numbers();
    generate_random_numbers_within_range();
    if let Err(err) = generate_random_numbers_with_distribution() {
        eprintln!("生成指定分布随机数发生错误 {}", err);
    }
    generate_random_values_of_custom_type();
    generate_random_passwords_from_alphanumeric_characters();
    generate_random_passwords_from_userdefined_characters();
}

/// # 生成随机数
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

/// # 在指定区间生成随机数
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

use rand_distr::{Distribution, Normal, NormalError};
/// # 生成指定分布的随机数
/// 默认的随机数使用均匀分布，`rand_distr` crate提供了其它类型的分布形态。通过创建一个分布实例，
/// 通过随机数生成器`rand::Rng`就可以使用`Distribution::sample`在指定分布中采样。
///
/// 下面的给出了使用`Normal`分布的演示，完整的分布实例可参考[文档](https://docs.rs/rand_distr/*/rand_distr/index.html).
fn generate_random_numbers_with_distribution() -> Result<(), NormalError> {
    let mut rng = thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("来自正态分布N(均值=2, 偏差=3)的随机数：{}", v);
    Ok(())
}

use rand::distributions::Standard;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

/// # 生成定制类型的随机值
/// 随机生成一个tuple(i32, bool, f64)和用户定义的类型`Point`.
/// 对`Standard`实现`Distribution`，这样就允许生成随机数
fn generate_random_values_of_custom_type() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("随机tuple: {:?}", rand_tuple);
    println!("随机Point: {:?}", rand_point);
}

use rand::distributions::Alphanumeric;

/// # 生成字符组成的随机密码
/// 随机生成给定长度的ASCII字符（A-Z，a-z，0-9）组成的随机密码，使用`Alphanumeric`采样
fn generate_random_passwords_from_alphanumeric_characters() {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("随机密码：{}", rand_string);
}

/// # 生成用户定义字符组成的随机密码
/// 随机生成一个给定长度的用户定义字符集的字符串
fn generate_random_passwords_from_userdefined_characters() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("随机指定字符集密码：{:?}", password);
}
