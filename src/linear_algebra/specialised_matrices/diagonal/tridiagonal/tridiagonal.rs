use super::*;

#[derive(Clone, Debug)]
pub struct TridiagonalMatrix<T>
{
    /* pub(crate) sup: Vec<T>,
    pub(crate) sub: Vec<T>,
    pub(crate) main: Vec<T>, */
    pub(crate) inner: Vec<T>,
    pub(crate) row: Option<usize>,
    pub(crate) col: Option<usize>,
    pub(crate) dim: Option<usize>
}

impl<T: Copy + Debug + Num> Tridiagonal<T> for TridiagonalMatrix<T>
where
    T: Num,
    Self: Square<T>
{
    // Only want to work with square tridiagonals for now.
}

impl<T: Copy + Debug + Num> Square<T> for TridiagonalMatrix<T> { }