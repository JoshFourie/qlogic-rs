mod array;
mod space;

#[allow(unused_macros)]

pub enum Implements {
    BinOps,
    UniOps,
    BlasOps
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

pub enum BlasOps {
    VAXPY,
    VAXPYMut,
    DotV,
    DotVMut     
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
                Implements::UniOps::VAdditiveInverseMut,
                Implements::BlasOps::VAXPY,
                Implements::BlasOps::VAXPYMut,
                Implements::BlasOps::DotV,
                Implements::BlasOps::DotVMut                
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

        use algebra::*;
        use vector::*;
    };
} 
