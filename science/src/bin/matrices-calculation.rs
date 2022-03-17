//! 线性代数计算
//! 矩阵的加法，乘法等计算
//! 在vector comparison中做向量比较使用approx包，这里需要引入ndarray的特性，按照帮助，这里使用的是0.15版本
//! 包含的features区分了0.5和0.4两个版本，名称不同，因此需要按照帮助引入approx-0_5
use approx::assert_abs_diff_eq;
use nalgebra::Matrix3;
use ndarray::Array;
use ndarray::{arr1, arr2, Array1};
use ndarray::{array, ArrayView1};

fn main() {
    println!("------------ Adding Matrices ------------");
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);

    let sum = &a + &b;

    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum);

    println!("------------ Multiplying Matrices ------------");
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let b = arr2(&[[6, 3], [5, 2], [4, 1]]);

    println!("{}", a.dot(&b));

    println!("------------ Multiply a scalar with a vector with a matrix ------------");
    let scalar = 4;
    let vector = arr1(&[1, 2, 3]);
    let matrix = arr2(&[[4, 5, 6], [7, 8, 9]]);
    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);

    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);

    println!("------------ Vector comparison ------------");
    let a = Array::from(vec![1., 2., 3., 4., 5.]);
    let b = Array::from(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
    let mut d = Array::from(vec![5., 4., 3., 2., 1.]);

    let z = a + b;
    let w = &c + &d;

    assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));

    println!("c={}", c);
    c[0] = 10.;
    d[1] = 10.;

    assert_abs_diff_eq!(w, Array::from(vec![6., 6., 6., 6., 6.]));

    println!("------------ Vector norm ------------");
    let x = array![1., 2., 3., 4., 5.];
    println!("||x||_2 = {}", l2_norm(x.view()));
    println!("||x||_1 = {}", l1_norm(x.view()));
    println!("Normalizing x yields {:?}", normalize(x));

    println!("------------ Invert matrix ------------");
    let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
    println!("m1 = {}", m1);
    match m1.try_inverse() {
        Some(inv) => {
            println!("The inverse of m1 is: {}", inv);
        }
        None => {
            println!("m1 is not invertible!");
        }
    }

    println!("------------ （De)-Serialize a matrix ------------");
}

fn l1_norm(x: ArrayView1<f64>) -> f64 {
    x.fold(0., |acc, elem| acc + elem.abs())
}

fn l2_norm(x: ArrayView1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

fn normalize(mut x: Array1<f64>) -> Array1<f64> {
    let norm = l2_norm(x.view());
    x.mapv_inplace(|e| e / norm);
    x
}
