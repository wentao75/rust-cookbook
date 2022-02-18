//! # 并发编程
//! 这个部分主要分为两个部分，第一是线程，第二是并发执行（主要是数据的并行处理）
//! 线程使用了`crossbeam`库，并发处理使用了`rayon`库

// 这里使用了`error_chain`库，统一完成错误处理模式，通过error_chain!宏定义引入，后续按照规则使用
#[macro_use]
extern crate error_chain;

use error_chain::ChainedError;
use glob::{glob_with, MatchOptions};
use image::imageops::FilterType;
use image::ImageError;
use rayon::prelude::*;
use std::fs::create_dir_all;
use std::path::Path;

error_chain! {
    foreign_links {
        Image(ImageError);
        Is(std::io::Error);
        Glob(glob::PatternError);
    }
}

fn main() {
    if let Err(ref e) = generate_thumbnails_in_parallel() {
        println!("生成缩略图错误：{}", e);
        for e in e.iter().skip(1) {
            println!("错误原因：{}", e);
        }
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

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| make_thumbnail(path, thumb_dir, 300).chain_err(|| path.display().to_string()))
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{:?}", x));
    println!("{}缩略图成功保存！", files.len() - image_failures.len());
    Ok(())
}

fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);

    Ok(img
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(file_path)?)
}
