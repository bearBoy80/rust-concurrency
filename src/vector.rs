use anyhow::Result;
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign<T>,
{
    if a.len() != b.len() {
       return Err(anyhow::anyhow!("Dot product error: a.len != b.len"))
    }
    Err(anyhow::anyhow!(""))
}

impl<T> Deref for Vector<T>{
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
