use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct HessenbergMatrix<T>
{
    pub(crate) inner: Vec<T>,
    pub(crate) dim: Option<usize>,
}

// EVD