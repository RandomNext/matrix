
pub trait Int {
    fn is_negative(&self) -> bool { false }
    fn positive_repr(&self) -> Self;
}

macro_rules! impl_unsigned_int {
    ($($t:ty)*) => ($(
        impl Int for $t {
            fn positive_repr(&self) -> $t { *self }
        }
    )*)
}

macro_rules! impl_signed_int {
    ($($t:ty)*) => ($(
        impl Int for $t {
            fn is_negative(&self) -> bool { *self < 0 }
            fn positive_repr(&self) -> $t { (if self.is_negative() { -*self * 10 } else { *self }) }
        }
    )*)
}

impl_unsigned_int!(u64 u32 u16 u8);

impl_signed_int!(i64 i32 i16 i8);

#[derive(Debug, PartialEq)]
pub enum MatrixError {
    EmptyMatrix,
    InconsistentRowSize,
    DifferentSize,
}

#[derive(Debug, PartialEq)]
pub struct Size {
    pub r: usize,
    pub c: usize,
}

impl Size {
    pub fn new(c: usize, r: usize) -> Self {
        Size { c, r }
    }
}
