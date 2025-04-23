use std::clone::Clone;
use std::fmt::Debug;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            return 0;
        }
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

// Implement PartialEq for Matrix to allow comparison in tests
impl<T: Clone + PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        // Matrices are equal if they have the same dimensions and all elements are equal
        if self.number_of_rows() != other.number_of_rows() || self.number_of_cols() != other.number_of_cols() {
            return false;
        }
        
        // Compare each element
        for i in 0..self.number_of_rows() {
            for j in 0..self.number_of_cols() {
                if self.0[i][j] != other.0[i][j] {
                    return false;
                }
            }
        }
        
        true
    }
}

impl<T> Mul for Matrix<T>
where
    T: Clone + Default + std::ops::AddAssign + std::ops::Mul<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_cols() != rhs.number_of_rows() {
            return None;
        }

        let rows = self.number_of_rows();
        let cols = rhs.number_of_cols();
        let inner_dim = self.number_of_cols(); // or rhs.number_of_rows()

        let mut result = vec![vec![T::default(); cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                for k in 0..inner_dim {
                    let product = self.0[i][k].clone() * rhs.0[k][j].clone();
                    result[i][j] += product;
                }
            }
        }

        Some(Matrix(result))
    }
}