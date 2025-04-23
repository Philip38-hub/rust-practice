// lalgebra_vector/src/lib.rs

use std::fmt::Debug;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

pub trait Scalar: Copy + Add<Output = Self> + Mul<Output = Self> + Debug + PartialEq + Eq {}
impl<T> Scalar for T where T: Copy + Add<Output = Self> + Mul<Output = Self> + Debug + PartialEq + Eq {}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut result = None;

        for (a, b) in self.0.iter().zip(other.0.iter()) {
            let product = *a * *b;
            result = Some(match result {
                Some(sum) => sum + product,
                None => product,
            });
        }

        result
    }
}

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let sum_vec = self.0.iter().zip(rhs.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Some(Vector(sum_vec))
    }
}
