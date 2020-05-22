#[macro_export]
macro_rules! BlasOps {
    (@VAXPY $name:ident, $space:ident) => {
        impl<T> VAXPY for $space<T>
        {
            type Vector = $name<T>;

            type Scalar = T;

            fn vaxpy(&self, alpha: &Self::Scalar, x: &Self::Vector, y: &Self::Vector) -> Self::Vector
            {
                unimplemented!()
            }
        }
    };

    (@VAXPYMut $name:ident, $space:ident) => {
        impl<T> VAXPYMut for $space<T>
        {
            type Vector = $name<T>;

            type Scalar = T;

            fn vaxpy_mut(&self, alpha: &Self::Scalar, x: &mut Self::Vector, y: &Self::Vector)
            {
                unimplemented!()
            }
        }
    };
}
