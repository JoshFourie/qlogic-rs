use crate::matrix;
use matrix::interface;

impl<T> interface::EigenValue for matrix::Matrix<T> {
    type Output = Vec<T>;

    fn eigenvalues(self) -> Self::Output {
        unimplemented!()
    }
}