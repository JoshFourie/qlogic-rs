mod array;
mod vec;
mod structural;

#[allow(unused_macros)]

#[macro_export]
macro_rules! ndarray {
    (
        @vector_space($space:ident) {
            @vector_ident($name:ident)
            @length($length:expr)
            @generic($generic:ident)
            $(@with_array($array:ty))?
            $(@with_vec($vector:ty))?
        }
    ) => {
        paste::item! {
            pub use [< _ vector $name With $space $length >]::{$name, $space};
                
            #[allow(unused_imports)]
            mod [< _ vector $name With $space $length >] 
            {
                use std::{marker, fmt, ops, iter};
                use iter::FromIterator;
                use marker::PhantomData;
                use fmt::Debug;
                use ops::{AddAssign, Add, MulAssign, Mul, Index, IndexMut, Neg};

                use super::{VAdd, VScale, VectorSpace, VPartialEq, VAdditiveInverse, ndarray};

                use vector::{binops, uniops, structural};

                $(
                    structural!($length, $name, $array, $generic);
                    ndarray!(@vectorspace $length, $name, $space, $array, $generic);
                    ndarray!(@array $length, $name, $space, $array, $generic);
                )?

                $(
                    structural!($length, $name, $vector, $generic);
                    ndarray!(@vectorspace $length, $name, $space, $vector, $generic);
                    ndarray!(@vec $length, $name, $space, $vector, $generic);
                )?
            }
        }
    };

    (@vectorspace $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
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

    (@array $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
        binops!(@addition $length, $name, $space, $inner, $T);
        binops!(@scale $length, $name, $space, $inner, $T);
        uniops!(@additive_inverse $length, $name, $space, $inner, $T);
    };

    (@vec $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
        binops!(@addition $length, $name, $space, $inner, $T);
        binops!(@scale $length, $name, $space, $inner, $T);
        uniops!(@additive_inverse $length, $name, $space, $inner, $T);
    };
} 
