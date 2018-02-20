use std::fmt;

pub mod types;

use types::{MatrixError, Size};

#[derive(Debug)]
pub struct Matrix {
    content: Vec<Vec<u32>>,
    size: Size,
    largest_digit_n: Vec<usize>,
}


impl Matrix {
    pub fn new(from: Vec<Vec<u32>>) -> Result<Self, MatrixError> {
        if from.is_empty() || from[0].is_empty() {
           return Err(MatrixError::EmptyMatrix);
        }
        let expected_col_n = from[0].len();
        match from.iter().all(|ref row| row.len() == expected_col_n) {
            true => {},
            false => return Err(MatrixError::InconsistentRowSize),
        }
        let row_n = from.len();
        let mut largest_digit_n = vec![0; expected_col_n];
        for row in &from {
            for i in 0..(expected_col_n) {
                if row[i].to_string().len() > largest_digit_n[i] {
                    largest_digit_n[i] = row[i].to_string().len();
                }
            }
        }

        Ok(Matrix {
            content: from,
            size: Size::new(row_n, expected_col_n),
            largest_digit_n,
        })
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.content.iter() {
            write!(f, "|")?;
            for i in 0..row.len() {
                write!(f, " {number:>width$}", number=row[i], width=self.largest_digit_n[i])?;
            }
            write!(f, " |\n")?;
        }
        Ok(())
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}


#[test]
fn test () {

    println!("{}", Matrix::new(vec![vec![3, 5, 6453], vec![34, 453, 3]]).unwrap());
    assert!(true);
}
