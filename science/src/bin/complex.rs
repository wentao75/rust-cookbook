use num::complex::Complex;
use std::f64::consts::PI;

fn main() {
    // 生成复数
    let complex_integer = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("整数复数：{}", complex_integer);
    println!("浮点数复数：{}", complex_float);

    // 复数加法
    let complex_num1 = num::complex::Complex::new(10.0, 20.0);
    let complex_num2 = num::complex::Complex::new(3.1, -4.2);

    let sum = complex_num1 + complex_num2;
    println!("复数和：({}) + ({}) = {}", complex_num1, complex_num2, sum);

    // 数学函数
    let x = Complex::new(0.0, 2.0 * PI);

    println!("e^(2i * pi) = {}", x.exp());
}
