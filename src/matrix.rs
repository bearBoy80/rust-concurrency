use core::fmt;
use std::{
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use crate::{dot_product, Vector};

// 定义一个矩阵
#[derive(Debug)]
pub struct Matrix<T: fmt::Debug> {
    data: Vec<T>,
    row: usize, // 行
    col: usize, //列
}
pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}
impl<T> MsgInput<T> {
    fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}
pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}
impl<T> Msg<T> {
    fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + fmt::Debug + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow::anyhow!("matrix multi error: a.col != b.row"));
    }
    const NUM_THREADS: usize = 4;
    //构造sender
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Send error: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();
    //
    let matrix_len = a.row * b.col;
    let mut receivers = Vec::with_capacity(matrix_len);
    // 多线程去做点乘
    let mut data: Vec<T> = vec![T::default(); matrix_len];
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}", e);
            }
            receivers.push(rx);
        }
    }
    // map/reduce: reduce phase
    for rx in receivers {
        let output = rx.recv()?;
        data[output.idx] = output.value;
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
    #[test]
    fn test_mult_matrix_by2() {
        let a: Matrix<i32> = Matrix::new([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12], 3, 4);
        let b: Matrix<i32> = Matrix::new([1, 2, 3, 4, 5, 6, 7, 8], 4, 2);
        let c = multiply(&a, &b).unwrap();
        assert_eq!(c.data, vec![50, 60, 114, 140, 178, 220]);
    }
}
