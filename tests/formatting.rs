extern crate matrix;

use matrix::Matrix;

#[test]
fn consistent_one_row() {
    let m = Matrix::new(vec![
                            vec![1, 2, 3]
    ]);
    assert_eq!(format!("{}", m.unwrap()), "| 1 2 3 |\n");
}

#[test]
fn consistent_one_col() {
    let m = Matrix::new(vec![
                            vec![1],
                            vec![4]
    ]);
    assert_eq!(format!("{}", m.unwrap()), "| 1 |\n| 4 |\n");
}

#[test]
fn consistent_multiple_row() {
    let m = Matrix::new(vec![
        vec![1, 2],
        vec![5, 6],
    ]);
    assert_eq!(format!("{}", m.unwrap()), "| 1 2 |\n| 5 6 |\n");
}

#[test]
fn inconsistent_multiple_row() {
    let m = Matrix::new(vec![
                            vec![1, 222, 3],
                            vec![1, 2, 3],
    ]);
    assert_eq!(format!("{}", m.unwrap()), "| 1 222 3 |\n| 1   2 3 |\n");
}

#[test]
fn negative_numbers() {
    let m = Matrix::new(vec![
                            vec![-23, 3525, 234, 1],
                            vec![34, -21, -123, -324],
    ]);
    assert_eq!(format!("{}", m.unwrap()),
"| -23 3525  234    1 |
|  34  -21 -123 -324 |
")
}
