

#[derive(Debug, PartialEq)]
pub enum MatrixError {
    EmptyMatrix,
    InconsistentRowSize,
}

#[derive(Debug)]
pub struct Size {
    pub r: usize,
    pub c: usize,
}

impl Size {
    pub fn new(c: usize, r: usize) -> Self {
        Size { c, r }
    }
}
