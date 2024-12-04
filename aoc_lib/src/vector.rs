use itertools::Itertools;
use std::ops::{Add, Deref, DerefMut, Mul, Sub};

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

impl<const DIMENSIONS: usize> Add for Vector<DIMENSIONS> {
    type Output = Vector<DIMENSIONS>;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(
            self.0
                .into_iter()
                .zip(rhs.0)
                .map(|(l, r)| l + r)
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }
}

impl<const DIMENSIONS: usize> Sub for Vector<DIMENSIONS> {
    type Output = Vector<DIMENSIONS>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector(
            self.0
                .into_iter()
                .zip(rhs.0)
                .map(|(l, r)| l - r)
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }
}

impl<const DIMENSIONS: usize> Mul for Vector<DIMENSIONS> {
    type Output = Vector<DIMENSIONS>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector(
            self.0
                .into_iter()
                .zip(rhs.0)
                .map(|(l, r)| l * r)
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }
}

impl<const DIMENSIONS: usize> Mul<i32> for Vector<DIMENSIONS> {
    type Output = Vector<DIMENSIONS>;

    fn mul(self, rhs: i32) -> Self::Output {
        Vector(
            self.0
                .into_iter()
                .map(|l| l * rhs)
                .collect_vec()
                .try_into()
                .unwrap(),
        )
    }
}

impl<const DIMENSIONS: usize> Vector<DIMENSIONS> {
    pub fn neighbours_diagonals(self) -> impl Iterator<Item = Vector<DIMENSIONS>> {
        self.0
            .into_iter()
            .map(|x| (x - 1)..=(x + 1))
            .multi_cartesian_product()
            .map(|x| Vector::new(x.try_into().unwrap()))
            .filter(move |x| *x != self)
    }

    pub fn neighbours(self) -> impl Iterator<Item = Vector<DIMENSIONS>> {
        self.neighbours_diagonals()
            .filter(move |x| x.manhattan_distance(self) == 1)
    }

    pub fn manhattan_distance(&self, other: Self) -> u32 {
        self.0
            .into_iter()
            .zip(other.0)
            .map(|(l, r)| l.abs_diff(r))
            .sum::<u32>()
    }
}
