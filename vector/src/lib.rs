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
macro_rules! ndarray 
{
    /********************* Convenience DSL ************************/
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
                using: Vec<T>
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

    /********************* Implementation ************************/
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

    (@imports) => {
        use std::{marker, fmt, ops, iter};
        use iter::FromIterator;
        use marker::PhantomData;
        use fmt::Debug;
        use ops::{AddAssign, Add, MulAssign, Mul, Index, IndexMut, Neg};

        use algebra::{VAdd, VAddMut, VScale, VScaleMut, VectorSpace, VPartialEq, VAdditiveInverse, VAdditiveInverseMut};
        use vector::{BinOps, UniOps, vector_base, vectorspace};
    };
} 
