use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vector<const DIMENSIONS: usize>([i32; DIMENSIONS]);

impl<const DIMENSIONS: usize> Vector<DIMENSIONS> {
    pub fn new(vector: [i32; DIMENSIONS]) -> Self {
        Self(vector)
    }
}

impl<const DIMENSIONS: usize> Deref for Vector<DIMENSIONS> {
    type Target = [i32; DIMENSIONS];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const DIMENSIONS: usize> DerefMut for Vector<DIMENSIONS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const DIMENSIONS: usize> Default for Vector<DIMENSIONS> {
    fn default() -> Self {
        Vector([0; DIMENSIONS])
    }
}
