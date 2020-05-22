/// The `axpyv` BLAS routine.
pub trait VAXPY
{
    type Vector;

    type Scalar;

    fn vaxpy(&self, alpha: &Self::Scalar, x: &Self::Vector, y: &Self::Vector) -> Self::Vector;
}

/// The `axpyv` BLAS routine.
pub trait VAXPYMut
{
    type Vector;

    type Scalar;

    fn vaxpy_mut(&self, alpha: &Self::Scalar, x: &mut Self::Vector, y: &Self::Vector);
}
