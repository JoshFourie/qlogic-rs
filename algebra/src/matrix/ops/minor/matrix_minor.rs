use std::ops;
use crate::matrix;

pub struct MatrixMinor<T> {
    pub(super) mat: matrix::Matrix<T>,
    pub(super) range: MinorRange
}

impl<T:Copy> MatrixMinor<T> {
    pub fn new(mat: matrix::Matrix<T>, range: MinorRange) -> Self {
        Self { mat, range }
    }

    pub fn into_matrix(self) -> matrix::Matrix<T> {
        self.mat
    }
}

impl<T:Copy> ops::Mul<Self> for MatrixMinor<T> 
where
    T: ops::Mul<T,Output=T> + num::Zero
{
    type Output = matrix::Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mat * rhs.mat
    }
}

impl<T:Copy> ops::Mul<matrix::Matrix<T>> for MatrixMinor<T> 
where
    T: ops::Mul<T,Output=T> + num::Zero
{
    type Output = MatrixMinor<T>;

    fn mul(self, rhs: matrix::Matrix<T>) -> Self::Output {
        let buf: _ = self.mat * rhs;
        Self::new(buf, self.range)
    }
}

impl<T:Copy> ops::Mul<MatrixMinor<T>> for matrix::Matrix<T>
where
    T: ops::Mul<T,Output=T> + num::Zero
{
    type Output = MatrixMinor<T>;

    fn mul(self, rhs: MatrixMinor<T>) -> Self::Output {
        let buf: _ = self * rhs.mat;
        MatrixMinor::new(buf, rhs.range)
    }
}

impl<T> From<MatrixMinor<T>> for matrix::Matrix<T> {
    fn from(minor: MatrixMinor<T>) -> Self {
        minor.mat
    }
}

impl<T> PartialEq<Self> for MatrixMinor<T> 
where
    T: PartialEq<T>
{
    fn eq(&self, rhs: &MatrixMinor<T>) -> bool {
        (rhs.mat == self.mat) && (rhs.range == self.range)
    }
}

impl<T:Clone> std::fmt::Debug for MatrixMinor<T> 
where
    T: std::fmt::Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} \n{:?}", self.mat, self.range)
    }
}

impl<T:Copy> IntoIterator for MatrixMinor<T> {
    type Item = T;
    type IntoIter = matrix::iter::MatrixIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.mat.into_iter()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct MinorRange {
    pub(super) cols: ops::Range<usize>,
    pub(super) rows: ops::Range<usize>
}

impl MinorRange {
    pub fn new(rows: ops::Range<usize>, cols: ops::Range<usize>) -> Self {
        MinorRange{ cols, rows }
    }

    pub fn into_tuple(self) -> (ops::Range<usize>,ops::Range<usize>) {
        (self.rows, self.cols)
    }
}
