/// The `axpyv` BLAS routine.
pub trait AccumulateScaledVector
{
    type Vector;

    type Scalar;

    fn axpyv(&self, alpha: &Self::Scalar, x: &Self::Vector, y: &Self::Vector) -> Self::Vector;
}