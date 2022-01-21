//! # 并发编程
//! 这个部分主要分为两个部分，第一是线程，第二是并发执行（主要是数据的并行处理）
//! 线程使用了`crossbeam`库，并发处理使用了`rayon`库

// 这里使用了`error_chain`库，统一完成错误处理模式，通过error_chain!宏定义引入，后续按照规则使用
#[macro_use]
extern crate error_chain;

mod explicit_threads;
mod parallel_tasks;

fn main() {
    explicit_threads::spawn_short_lived_thread();

    explicit_threads::create_parallel_pipeline();

    explicit_threads::pass_data_between_two_threads();

    if let Err(ref e) = explicit_threads::maintain_global_mutable_state() {
        println!("保持全局可变状态发生错误：{}", e);
        for e in e.iter().skip(1) {
            println!("错误原因：{}", e);
        }
    }

    // if let Err(ref e) = explicit_threads::calculate_sha256_of_isofile() {
    //     println!("计算文件散列值错误: {}", e);
    //     for e in e.iter().skip(1) {
    //         println!("错误原因：{}", e);
    //     }
    // }

    // if let Err(ref e) = explicit_threads::draw_fractal_dispatching_work_to_a_threadpool() {
    //     println!("绘制分型图错误：{}", e);
    //     for e in e.iter().skip(1) {
    //         println!("错误原因：{}", e);
    //     }
    // }

    parallel_tasks::mutate_elements_of_an_array_in_parallel();
    parallel_tasks::test_in_parallel();
    parallel_tasks::search_item_in_parallel();
    parallel_tasks::sort_in_parallel();
    parallel_tasks::map_reduce_in_parallel();

    if let Err(ref e) = parallel_tasks::generate_thumbnails_in_parallel() {
        println!("生成缩略图错误：{}", e);
        for e in e.iter().skip(1) {
            println!("错误原因：{}", e);
        }
    }
}