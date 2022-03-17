//! 序列化和反序列化Matrix
//! 这里使用了`serde_json::to_string`和`serde_json::from_str`来执行序列化和反序列化
//! 这里需要注意引入nalgebra包的时候增加feature："serde-serialize"
//!
extern crate nalgebra;
extern crate serde_json;

use nalgebra::DMatrix;

fn main() -> Result<(), std::io::Error> {
    let row_slice: Vec<i32> = (1..5001).collect();
    let matrix = DMatrix::from_row_slice(50, 100, &row_slice);

    let serialized_matrix = serde_json::to_string(&matrix)?;
    let deserialized_matrix: DMatrix<i32> = serde_json::from_str(&serialized_matrix)?;

    assert!(deserialized_matrix == matrix);

    Ok(())
}
