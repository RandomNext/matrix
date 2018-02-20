extern crate matrix;

use matrix::Matrix;

#[test]
fn partial_eq() {
    let m0 = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 7]]).unwrap();
    let m1 = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 7]]).unwrap();
    
    assert_eq!(m0, m1);
    assert_eq!(m1, m0);
}
