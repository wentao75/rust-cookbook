//! 中心趋势测量
//! 这个例子计算数据集合的中心趋势。返回使用`Option`在集合为空时返回没有结果的情况。
//! 下面例子使用一个可变的`HashMap`收集每一个不同数据的数量，再使用`max_by_key`获得最多频率的值
use std::collections::HashMap;

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });
    println!("frequencies: {:?}", frequencies);

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value);

    println!("数据的Mode为：{:?}", mode);
}
