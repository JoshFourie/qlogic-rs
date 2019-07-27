use crate::matrix;
use matrix::{interface, ops};

struct SchurDecomposition<T> {
    Q: matrix::Matrix<T>,
    U: matrix::Matrix<T>,
    Q_star: matrix::Matrix<T>
}

impl<T> SchurDecomposition<T> {

    fn new(mat: matrix::Matrix<T>) -> Self {
        unimplemented!()
    }

}
