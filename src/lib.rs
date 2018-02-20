use std::fmt;

pub mod types;

use types::{MatrixError, Size};

#[derive(Debug)]
pub struct Matrix {
    content: Vec<Vec<u32>>,
    size: Size,
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

        Ok(Matrix {
            content: from,
            size: Size::new(row_n, expected_col_n),
        })
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str_represent = String::new();
        for row in self.content.iter() {
            str_represent.push_str(&format!("|{} |\n", row.iter().fold(String::new(), |cced, val| cced + " " + &val.to_string())));
        }
        write!(f, "{}", str_represent)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}
