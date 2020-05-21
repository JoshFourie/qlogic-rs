mod array;
mod space;
mod structural;

#[allow(unused_macros)]

#[macro_export]
macro_rules! ndarray {
    (
        @vector_space($space:ident) {
            @vector_ident($name:ident)
            @length($length:expr)
            @generic($generic:ident)
            @with($array:ty)
        }
    ) => {
        paste::item! {
            pub use [< _ vector $name With $space $length >]::{$name, $space};
                
            #[allow(unused_imports)]
            mod [< _ vector $name With $space $length >] 
            {
                use vector::ndarray;

                ndarray!(@imports);

                structural!($length, $name, $array, $generic);
                vectorspace!($length, $name, $space, $array, $generic);
                ndarray!(@array $length, $name, $space, $array, $generic);
            }
        }
    };

    (@imports) => {
        use std::{marker, fmt, ops, iter};
        use iter::FromIterator;
        use marker::PhantomData;
        use fmt::Debug;
        use ops::{AddAssign, Add, MulAssign, Mul, Index, IndexMut, Neg};

        use algebra::vector::{VAdd, VScale, VectorSpace, VPartialEq, VAdditiveInverse};
        use vector::{binops, uniops, structural, vectorspace};
    };

    (@array $length:expr, $name:ident, $space:ident, $inner:ty, $T:ident) => {
        binops!(@addition $length, $name, $space, $inner, $T);
        binops!(@scale $length, $name, $space, $inner, $T);
        uniops!(@additive_inverse $length, $name, $space, $inner, $T);
    };
} 
