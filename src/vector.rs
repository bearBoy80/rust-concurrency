use anyhow::Result;
use std::ops::{Add, AddAssign, Deref, Mul};

#[derive(Debug)]
pub struct Vector<T> {
    data: Vec<T>,
}
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign<T>,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Dot product error: a.len != b.len"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}