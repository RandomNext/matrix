

#[derive(Debug, PartialEq)]
pub enum MatrixError {
    EmptyMatrix,
    InconsistentRowSize,
}

#[derive(Debug)]
pub struct Size {
    r: usize,
    c: usize,
}

impl Size {
    pub fn new(c: usize, r: usize) -> Self {
        Size { c, r }
    }
}
