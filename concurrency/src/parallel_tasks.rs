use error_chain::error_chain;
use rayon::prelude::*;

/// # 并行修改数组中的元素
/// 例子使用`rayon`包，这是一个数据并行库。
/// `rayon`提供`par_iter_mut`方法对任何可以平行迭代的数据类型进行处理。
/// 这是一个潜在并行运行的类似迭代器的链。
pub fn mutate_elements_of_an_array_in_parallel(){
    println!("并行修改...");
    let mut arr = [0, 7, 9 ,11];
    arr.par_iter_mut().for_each(|p| *p = 1);
    println!("{:?}", arr);
    println!("并行修改完成！");
}

/// # 平行测试一个给定预测是否有一个或全部元素匹配
/// 例子使用`rayon::any`和`rayon::all`方法，对应`std::any`和`std::all`方法的并行运算。
/// `rayon::any`并行检查迭代器的任何一个元素匹配预测，只要有一个呗发现匹配就返回。
/// `rayon::all`并行检查迭代器的所有元素是否匹配预测，如果发现有任何一个不匹配就返回。
pub fn test_in_parallel() {
    println!("并行测试...");
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8));
    assert!(vec.par_iter().all(|n| *n <= 8));

    vec.push(9);
    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8));
    assert!(!vec.par_iter().all(|n| *n <= 8));
    println!("并行测试结束！");
}

/// # 并行搜索给定条件的项目
/// 例子使用`rayon::find_any`和`par_iter`并行搜索一个vector，用来找到闭包给出的满足条件的元素。
/// 如果有多个元素满足定义在闭包中的预测条件，返回第一个找到的元素，而不是队列中的第一个。
pub fn search_item_in_parallel(){
    println!("并行搜索...");
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
    println!("并行搜索完成！");
}

use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

/// # 并行排序
/// 这个例子给一个字符串队列并行排序
/// 
/// 分配一个空字符串队列。`par_iter_mut().for_each`并行填充随机值。
/// 尽管存在多个选项对可枚举类型排序，`par_sort_unstable`通常比稳定排序算法更快。
pub fn sort_in_parallel(){
    println!("并行排序...");
    let mut vec  = vec![String::new(); 100_000];
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        // *p = (0..5).map(|_| rng.sample(&Alphanumeric)).collect()
        *p = rng.sample_iter(&Alphanumeric)
                .take(5)
                .map(char::from)
                .collect();
    });
    vec.par_sort_unstable();
    println!("并行排序完成！");
}

struct Person {
    age: u32,
}

/// # Map-reduce并行计算
/// 例子使用`rayon::filter`，`rayon::map`和`rayon::reduce`来计算`Person`对象超过30岁的平均年龄。
/// 
/// `rayon::filter`返回满足给定条件的元素；
/// `rayon::map`对每一个元素执行一个操作，创建一个新的迭代器；
/// `rayon::reduce`对当前元素和前一个合并执行一个操作。
/// 这里展示了`rayon::sum`操作，这和reduce操作在例子中结果相同。
pub fn map_reduce_in_parallel(){
    println!("map-reduce计算...");
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30 = v.par_iter().map(|x| x.age).filter(|&x| x>30).reduce(||0, |x, y| x + y );

    let alt_sum_30 : u32 = v.par_iter().map(|x| x.age).filter(|&x| x>30).sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < std::f32::EPSILON);
    println!("大于30岁人的平均年龄为：{}", avg_over_30);
}

use std::path::Path;
use std::fs::create_dir_all;
use glob::{glob_with, MatchOptions};
use image::imageops::FilterType;
use image::{ImageError};
use error_chain::ChainedError;

error_chain!{
    foreign_links {
        Image(ImageError);
        Is(std::io::Error);
        Glob(glob::PatternError);
    }
}

/// # 并行生成JPG缩略图
/// 这里例子对当前目录的.jpg文件生成缩略图，并保存在子目录`thumbnails`中。
/// 
/// `glob::glob_with`搜索当前目录中的所有jpeg文件。
/// `rayon`通过`par_iter`调用`DynamicImage::resize`并行调整图片尺寸。
pub fn generate_thumbnails_in_parallel() -> Result<()> {
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.jpg", options)?
            .filter_map(|x| x.ok())
            .collect();
        
    if files.len() == 0 {
        error_chain::bail!("当前目录中没有找到.jpg文件！");
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    println!("保存{}缩略图到{}...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files.par_iter()
            .map(|path| {
                make_thumbnail(path, thumb_dir, 300)
                    .chain_err(|| path.display().to_string())
            })
            .filter_map(|x| x.err())
            .collect();

    image_failures.iter().for_each(|x| println!("{:?}", x));
    println!("{}缩略图成功保存！", files.len()-image_failures.len());
    Ok(())
}

fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);

    Ok(img.resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}