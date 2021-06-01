use std::result::Result;

#[derive(Debug)]
pub struct VectorWrapper<T> {
    vec: Vec<T>,
}

#[derive(Debug)]
pub enum VectorError {
    IndexOutOfBoundsError,
}

impl<T> VectorWrapper<T> {
    pub fn new() -> Self {
        Self { vec: vec![] }
    }

    pub fn push(&mut self, value: T) {
        self.vec.push(value);
    }

    pub fn get(self, index: usize) -> Result<T, VectorError>
    where
        T: Copy,
    {
        if self.vec.len() <= index {
            return Err(VectorError::IndexOutOfBoundsError);
        }

        Ok(self.vec[index])
    }
}
