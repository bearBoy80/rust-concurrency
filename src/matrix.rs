use core::fmt;
use std::ops::{Add, AddAssign, Mul};

use anyhow::Ok;

// 定义一个矩阵
#[derive(Debug)]
pub struct Matrix<T: fmt::Debug> {
    data: Vec<T>,
    row: usize, // 行
    col: usize, //列
}
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + fmt::Debug,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("matrix multi error: a.col != b.row"));
    }
    // a = 1,2, 3,4  2,2 b =1,2,3,4 2,2
    let mut data: Vec<T> = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j]
            }
        }
    }
    let result = Matrix::new(data, a.row, b.col);
    Ok(result)
}
impl<T: fmt::Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mult_matrix() {
        let a: Matrix<i32> = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        // 1,2;3,4;5,6
        let b: Matrix<i32> = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b).unwrap();
        assert!(c.row == a.row);
        assert!(c.col == b.col);
        assert_eq!(c.data, vec![22, 28, 49, 64])
    }
}
