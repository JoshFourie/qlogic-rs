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
                use ops::{AddAssign, Index, IndexMut};

                use super::*;

                #[derive(Copy, Clone)]
                pub struct [< Vector $length >]<T>([T; $length]);  

                impl<T> [< Vector $length >]<T>
                {
                    pub fn new(inner: [T; $length]) -> Self 
                    {
                        [< Vector $length >](inner)
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

                impl<T> Index<usize> for [< Vector $length >]<T>
                {
                    type Output = T;

                    fn index(&self, idx: usize) -> &Self::Output 
                    {
                        &self.0[idx]
                    }
                }

                impl<T> IndexMut<usize> for [< Vector $length >]<T>
                {
                    fn index_mut(&mut self, idx: usize) -> &mut Self::Output 
                    {
                        &mut self.0[idx]
                    }
                }


                pub struct [< VectorSpace $length >]<T> {
                    _phantom: PhantomData<T>
                }

                impl<T> [< VectorSpace $length >]<T>
                {
                    #[inline]
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
                    T: Copy + AddAssign<T>
                {
                    type Vector = [< Vector $length >]<T>;

                    fn vadd(&self, lhs: &mut Self::Vector, rhs: &Self::Vector)
                    {
                        for idx in 0..$length {
                            unsafe { 
                                lhs.0.get_unchecked_mut(idx).add_assign( rhs.0.get_unchecked(idx).clone() ) 
                            }
                        }
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
