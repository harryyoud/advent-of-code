use crate::vector::Vector;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Grid<T> {
    inner: HashMap<Vector<2>, T>,
}

impl<T> Grid<T> {
    pub fn new(inner: HashMap<Vector<2>, T>) -> Self {
        Self { inner }
    }

    pub fn get(&self, point: Vector<2>) -> Option<&T> {
        self.inner.get(&point)
    }

    pub fn insert(&mut self, point: Vector<2>, value: T) {
        self.inner.insert(point, value);
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vector<2>, &T)> {
        self.inner.iter().map(|(point, value)| (*point, value))
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.inner.values()
    }

    pub fn keys(&self) -> impl Iterator<Item = Vector<2>> + use<'_, T> {
        self.inner.keys().copied()
    }

    pub fn parse<F>(input: &str, f: F) -> Self
    where
        F: Fn(char) -> T,
    {
        let inner = |v, c| (v, f(c));
        Self {
            inner: input
                .lines()
                .enumerate()
                .flat_map(move |(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(move |(x, c)| inner(Vector::new([x as i32, y as i32]), c))
                })
                .collect::<HashMap<Vector<2>, T>>(),
        }
    }

    pub fn tick<F>(&mut self, mut f: F)
    where
        F: FnMut(Vector<2>, &T) -> T,
    {
        self.inner = self
            .inner
            .iter()
            .map(|(point, value)| (*point, f(*point, value)))
            .collect()
    }
}
