use num::bigint::{BigInt, ToBigInt};

fn factorial(x: i32) -> BigInt {
    if let Some(mut factorial) = 1.to_bigint() {
        for i in 1..=x {
            factorial = factorial * i;
        }
        factorial
    } else {
        panic!("无法计算阶乘！");
    }
}

fn main() {
    println!("{}! = {}", 100, factorial(100));
}
