mod array;
mod space;

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
            pub use [< $space:lower >]::{$name, $space};
                
            #[allow(unused_imports)]
            mod [< $space:lower >]
            {
                use vector::ndarray;
    
                ndarray!(@imports);
    
                vector_base!($length, $name, $array, $generic);
                vectorspace!($length, $name, $space, $array, $generic);
                
                binops!(@addition $length, $name, $space, $array, $generic);
                binops!(@scale $length, $name, $space, $array, $generic);
    
                uniops!(@additive_inverse $length, $name, $space, $array, $generic);
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
        use vector::{binops, uniops, vector_base, vectorspace};
    };
} 
