mod array;
mod space;

#[allow(unused_macros)]

pub enum Implements {
    BinOps
}

pub enum BinOps {
    VAdd,
    VAddMut,
    VScale,
    VScaleMut
}

pub enum UniOps {
    VAdditiveInverse,
    VAdditiveInverseMut
}

#[macro_export]
macro_rules! ndarray {
    (
        $space:ident {
            vector: $name:ident,
            dimension: $length:expr
        }
    ) => {
        ndarray! {
            $space {
                vector: $name,
                dimension: $length,
                using: Vec<T>,
                Implements::BinOps::VAddMut,
                Implements::BinOps::VAdd,
                Implements::BinOps::VScale,
                Implements::BinOps::VScaleMut,
                Implements::UniOps::VAdditiveInverse,
                Implements::UniOps::VAdditiveInverseMut
            }
        }
    };

    (
        $space:ident {
            vector: $name:ident,
            dimension: $length:expr,
            using: $inner:ty
        }
    ) => {
        ndarray! {
            $space {
                vector: $name,
                dimension: $length,
                using: $inner,
                Implements::BinOps::VAddMut,
                Implements::BinOps::VAdd,
                Implements::BinOps::VScale,
                Implements::BinOps::VScaleMut,
                Implements::UniOps::VAdditiveInverse,
                Implements::UniOps::VAdditiveInverseMut
            }
        }
    };

    (
        $space:ident {
            vector: $name:ident,
            dimension: $length:expr,
            using: $inner:ty,
            $(Implements::$kind:ident::$branch:ident),*
        }
    ) => {
        paste::item! {
            pub use [< $space:lower >]::{$name, $space};
                
            #[allow(unused_imports)]
            mod [< $space:lower >]
            {
                use vector::ndarray;
    
                ndarray!(@imports);
    
                vector_base!($length, $name, $inner, T);
                vectorspace!($length, $name, $space, $inner, T);    

                ndarray!(@implements $name, $space, $($kind, $branch),*);
            }
        }
    };

    (@implements $name:ident, $space:ident, $($kind:ident, $branch:ident),*) => {
        $(
            $kind!(@ $branch $name, $space);
        )*      
    };

    // (
    //     @vector_space($space:ident) {
    //         @vector_ident($name:ident)
    //         @length($length:expr)
    //     }
    // ) => {
    //     vector::ndarray!{
    //         @vector_space($space) {
    //             @vector_ident($name)
    //             @length($length)
    //             @generic(T)
    //             @with(Vec<T>)
    //         }
    //     }
    // };

    // (
    //     @vector_space($space:ident) {
    //         @vector_ident($name:ident)
    //         @length($length:expr)
    //         @generic($generic:ident)
    //         @with($array:ty)
    //     }
    // ) => {
    //     paste::item! {
    //         pub use [< $space:lower >]::{$name, $space};
                
    //         #[allow(unused_imports)]
    //         mod [< $space:lower >]
    //         {
    //             use vector::ndarray;
    
    //             ndarray!(@imports);
    
    //             vector_base!($length, $name, $array, $generic);
    //             vectorspace!($length, $name, $space, $array, $generic);
                
    //             binops!(@addition $length, $name, $space, $array, $generic);
    //             binops!(@scale $length, $name, $space, $array, $generic);
    
    //             uniops!(@additive_inverse $length, $name, $space, $array, $generic);
    //         }
    //     }
    // };

    (@imports) => {
        use std::{marker, fmt, ops, iter};
        use iter::FromIterator;
        use marker::PhantomData;
        use fmt::Debug;
        use ops::{AddAssign, Add, MulAssign, Mul, Index, IndexMut, Neg};

        use algebra::vector::{VAdd, VAddMut, VScale, VScaleMut, VectorSpace, VPartialEq, VAdditiveInverse, VAdditiveInverseMut};
        use vector::{BinOps, UniOps, vector_base, vectorspace};
    };
} 
