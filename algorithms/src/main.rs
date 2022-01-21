//! # 算法
//!
//!
mod random_values;
mod sort_vector;

fn main() {
    random_values::generate_random_numbers();
    random_values::generate_random_numbers_within_range();
    if let Err(err) = random_values::generate_random_numbers_with_distribution() {
        eprintln!("生成指定分布随机数发生错误 {}", err);
    }
    random_values::generate_random_values_of_custom_type();
    random_values::generate_random_passwords_from_alphanumeric_characters();
    random_values::generate_random_passwords_from_userdefined_characters();

    sort_vector::sort_vector_of_integers();
    sort_vector::sort_vector_of_floats();
    sort_vector::sort_vector_of_structs();
}
