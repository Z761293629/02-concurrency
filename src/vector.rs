use anyhow::Result;
use std::ops::{Add, AddAssign, Deref, Mul};

#[derive(Debug)]
pub struct Vector<T>(Vec<T>);

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Vector<T> {
    pub fn new(value: impl Into<Vec<T>>) -> Self {
        Self(value.into())
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: AddAssign + Mul<Output = T> + Add<Output = T> + Default + Copy,
{
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("dot product error a.len != b.len"));
    }
    let mut result = T::default();
    for i in 0..a.len() {
        result += a[i] * b[i]
    }
    Ok(result)
}
