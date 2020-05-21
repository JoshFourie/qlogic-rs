#[macro_export]
macro_rules! vectorspace {
    ($length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
        pub struct $space<$T> {
            _phantom: PhantomData<$T>
        }

        impl<$T> $space<$T>
        {
            #[inline]
            pub fn new() -> Self 
            {
                $space {
                    _phantom: PhantomData
                }
            }
        }

        impl<$T> VectorSpace for $space<$T>
        {
            type Scalar = $T;

            type Vector = $space<$T>;

            fn dimensions(&self) -> usize 
            {
                $length
            }
        }

        impl<$T> VPartialEq for $space<$T>
        where
            $T: PartialEq
        {
            type Vector = $name<$T>;

            fn eq(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> bool
            {
                for (l, r) in lhs
                    .into_iter()
                    .zip( rhs.into_iter() ) 
                {
                    if (l != r) {
                        return false
                    }
                }
                return true
            }
        }
    };
} 
