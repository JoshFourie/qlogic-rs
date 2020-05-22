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

    fn vaxpy_mut(&self, alpha: &Self::Scalar, x: &Self::Vector, y: &mut Self::Vector);
}

/// The `dotv` BLAS routine.
pub trait DotV
{
    type Vector;

    type Scalar;

    fn dotv(&self, x: &Self::Vector, y: &Self::Vector) -> Self::Vector;
}

/// The `dotv` BLAS routine.
pub trait DotVMut
{
    type Vector;

    type Scalar;

    fn dotv_mut(&self, x: &mut Self::Vector, y: &Self::Vector);
}
