#[allow(unused_macros)]

#[macro_export]
macro_rules! ndvector {

    ($length:tt) => {
        paste::item! {
            pub use [< _ vector $length >]::{[< Vector $length >], [< VectorSpace $length >]};
            
            mod [< _ vector $length >] 
            {
                use std::{marker, ops};
                use marker::PhantomData;
                use ops::AddAssign;

                use super::*;

                #[derive(Copy, Clone)]
                pub struct [< Vector $length >]<T>([T; $length]);  

                impl<T> [< Vector $length >]<T>
                {
                    pub fn new(inner: [T; $length]) -> Self 
                    {
                        [< Vector $length >](inner)
                    }

                    fn iter_mut(&mut self) -> std::slice::IterMut<'_,T>
                    {
                        self.0.iter_mut()
                    }
                }

                impl<'a,T> IntoIterator for &'a [< Vector $length >]<T>
                {
                    type Item = &'a T;
                    type IntoIter = std::slice::Iter<'a,T>;

                    fn into_iter(self) -> Self::IntoIter
                    {
                        self.0.iter()
                    }
                }


                pub struct [< VectorSpace $length >]<T> {
                    _phantom: PhantomData<T>
                }

                impl<T> [< VectorSpace $length >]<T>
                {
                    pub fn new() -> Self 
                    {
                        [< VectorSpace $length >] {
                            _phantom: PhantomData
                        }
                    }
                }

                impl<T> VectorSpace for [< VectorSpace $length >]<T>
                {
                    type Scalar = T;

                    type Vector = [< VectorSpace $length >]<T>;

                    fn dimensions(&self) -> usize 
                    {
                        $length
                    }
                }

                impl<T> VAdd for [< VectorSpace $length >]<T>
                where
                    for <'a> T: AddAssign<&'a T>
                {
                    type Input = [< Vector $length >]<T>;

                    type Output = [< Vector $length >]<T>;

                    fn vadd(&self, mut lhs: Self::Input, rhs: Self::Input) -> Self::Output
                    {
                        lhs
                            .iter_mut()
                            .zip( rhs.into_iter() )
                            .for_each(|(l,r)| *l += r);
                        lhs
                    }
                }

                impl<T> VPartialEq for [< VectorSpace $length >]<T>
                where
                    T: PartialEq
                {
                    type Vector = [< Vector $length >]<T>;

                    fn eq(lhs: Self::Vector, rhs: Self::Vector) -> bool
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
            }
        }
    };

} 

use crate::ndvector;
use crate::vector::*;

ndvector!(1024);
