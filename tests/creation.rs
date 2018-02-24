extern crate matrix;

#[test]
fn test_creation() {
    let m = matrix::Matrix::new(
        vec![
            vec![2, 4, 4, 4],
            vec![0, 5, 0, 5],
        ],
    );
    assert!(match m {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[test]
fn test_no_rows() {
    assert!(match matrix::Matrix::new(vec![] as Vec<Vec<u32>>) {
        Ok(_) => false,
        Err(_e @ matrix::types::MatrixError::EmptyMatrix) => true,
        Err(_) => false,
    })
}

#[test]
fn test_no_cols_0() {
    assert!(match matrix::Matrix::new(vec![vec![]] as Vec<Vec<u32>>) {
        Ok(_) => false,
        Err(_e @ matrix::types::MatrixError::EmptyMatrix) => true,
        Err(_) => false,
    })
}

#[test]
fn test_no_cols_1() {
    assert!(match matrix::Matrix::new(vec![vec![], vec![1, 3, 4]]) {
        Ok(_) => false,
        Err(_e @ matrix::types::MatrixError::EmptyMatrix) => true,
        Err(_) => false,
    })
}

#[test]
fn test_inconsistency_0() {
    assert!(match matrix::Matrix::new(vec![vec![1, 2], vec![1]]) {
        Ok(_) => false,
        Err(_e @ matrix::types::MatrixError::InconsistentRowSize) => true,
        Err(_) => false,
    })
}

#[test]
fn test_inconsistency_1() {
    assert!(match matrix::Matrix::new(vec![vec![1, 2], vec![]]) {
        Ok(_) => false,
        Err(_e @ matrix::types::MatrixError::InconsistentRowSize) => true,
        Err(_) => false,
    })
}

#[test]
fn test_inconsistency_2() {
    assert!(match matrix::Matrix::new(vec![vec![1, 2], vec![1, 3, 5]]) {
        Ok(_) => false,
        Err(_e @ matrix::types::MatrixError::InconsistentRowSize) => true,
        Err(_) => false,
    })
}

#[test]
fn test_inconsistency_3() {
    assert!(match matrix::Matrix::new(vec![vec![1, 2], vec![1, 3], vec![3]]) {
        Ok(_) => false,
        Err(_e @ matrix::types::MatrixError::InconsistentRowSize) => true,
        Err(_) => false,
    })
}

#[test]
fn test_negative_numbers() {
    assert!(match matrix::Matrix::new(vec![vec![-1, -345], vec![435, 345]]) {
        Ok(_) => true,
        Err(_) => false,
    })
}

