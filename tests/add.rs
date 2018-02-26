extern crate matrix;

use matrix::Matrix;

#[test]
fn simple_add() {
    let a = Matrix::new(vec![vec![1, 2], vec![5, 5]]).unwrap();
    let b = Matrix::new(vec![vec![3, 5], vec![3, 0]]).unwrap();

    let x = Matrix::new(vec![vec![4, 7], vec![8, 5]]).unwrap();

    assert_eq!((a + b).unwrap(), x);
}

#[test]
fn assoc_add() {
    let a = Matrix::new(vec![vec![1, 2], vec![5, 5]]).unwrap();
    let b = Matrix::new(vec![vec![3, 5], vec![3, 0]]).unwrap();
    let c = Matrix::new(vec![vec![5, 2], vec![2, 4]]).unwrap();

    let x = Matrix::new(vec![vec![9, 9], vec![10, 9]]).unwrap();

    let res = (((a + b).unwrap()) + c).unwrap();
    assert_eq!(res, x);
}
