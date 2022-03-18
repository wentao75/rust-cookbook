//! 中心趋势测量
//! 这个例子计算数据集合的中心趋势。返回使用`Option`在集合为空时返回没有结果的情况。
//! 下面例子使用基础的方法计算数据集合的和，数量以及平均值=和/数量

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    let mean = match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    };

    println!("数据的平均值为：{:?}", mean);
}
