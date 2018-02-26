use std::fmt;
use std::string::ToString;
use std::ops::{Add};

pub mod types;

use types::{MatrixError, Size, Int};

#[derive(Debug)]
pub struct Matrix<T: Int> {
    content: Vec<Vec<T>>,
    size: Size,
    largest_digit_n: Vec<usize>,
}

impl<T: Int + ToString + Copy + Ord> Matrix<T> {
    pub fn new(from: Vec<Vec<T>>) -> Result<Self, MatrixError> {
        if from.is_empty() || from[0].is_empty() {
            return Err(MatrixError::EmptyMatrix);
        }
        let expected_col_n = from[0].len();
        match from.iter().all(|ref row| row.len() == expected_col_n) {
            true => {}
            false => return Err(MatrixError::InconsistentRowSize),
        }
        let row_n = from.len();
        let mut largest_digit_n = vec![0; expected_col_n];
        for i in 0..(expected_col_n) {
            let mut max_col_v = from[0][0];
            for row in &from { // TODO: Would be great to skip first item [line above]
                max_col_v = row[i].positive_repr().max(max_col_v);
            }
            largest_digit_n[i] = max_col_v.to_string().len();
        }

        Ok(Matrix {
            content: from,
            size: Size::new(row_n, expected_col_n),
            largest_digit_n,
        })
    }
}

impl<T: Int + Add<Output=T> + Copy> Add for Matrix<T> {
    type Output = Result<Self, MatrixError>;
    fn add(mut self, rhs: Self) -> Self::Output {
        if self.size != rhs.size {
            return Err(MatrixError::DifferentSize);
        }
        for (i, row) in rhs.content.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                self.content[i][j] = self.content[i][j] + *value;
            }
        }
        Ok(self)
    }
}

impl<T: Int + fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.content.iter() {
            write!(f, "|")?;
            for i in 0..row.len() {
                write!(
                    f,
                    " {number:>width$}",
                    number = row[i],
                    width = self.largest_digit_n[i]
                )?;
            }
            write!(f, " |\n")?;
        }
        Ok(())
    }
}

impl<T: Int + PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

#[test]
fn test() {
    println!(
        "{}",
        Matrix::new(vec![vec![3, 5, 6453], vec![34, 453, 3]]).unwrap()
    );
    assert!(true);
}
