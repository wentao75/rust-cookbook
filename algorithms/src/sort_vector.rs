//! # 向量排序
//! 

/// # 整数队列排序
/// 使用`vec::sort`排序整数队列，也可以使用`vec::sort_unstable`作为替换，这个方法更快，但是不能保留相等元素的顺序
pub fn sort_vector_of_integers(){
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}

/// # 浮点数队列排序
pub fn sort_vector_of_floats() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

/// # 结构体排序
/// 例子排序一个Person结构体，带有`name`和`age`属性，排序通过自然顺序（名字+年龄）。
/// 为了能够排序，需要四个traits`Eq`，`PartialEq`，`Ord`和`PartialOrd`。
/// 这些traits可以简单的导出。可以提供一个定制的比较方法，使用`vec::sort_by`方法以及仅通过年龄排序。
pub fn sort_vector_of_structs(){
    println!("排序结构体...");
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    people.sort();

    assert_eq!(people, vec![Person::new("Al".to_string(), 60), Person::new("John".to_string(), 1), Person::new("Zoe".to_string(), 25),]);

    // 通过age排序
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(people, vec![Person::new("Al".to_string(), 60), Person::new("Zoe".to_string(), 25), Person::new("John".to_string(), 1)]);
    println!("完成排序结构体");
}