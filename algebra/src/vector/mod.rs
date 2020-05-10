mod vector_space;
pub use vector_space::*;

use std::{ops, iter};
use ops::{Index, IndexMut};
use iter::FromIterator;

pub trait Vector<T> 
{
    // Supertrait.
}

impl<T,U> Vector<T> for U
where
    U: VSubscript<T> + VIterable<T>
{
    // Empty.
}


pub trait VSubscript<T>: Index<usize, Output=T> + IndexMut<usize>
{
    // Supertrait.
}

impl<T,U> VSubscript<T> for U
where
    U: Index<usize, Output=T> + IndexMut<usize>
{
    // Empty.
}


pub trait VIterable<T>: IntoIterator<Item=T> + FromIterator<T>
{
    // Supertrait.
}

impl<T,U> VIterable<T> for U
where
    U: IntoIterator<Item=T> + FromIterator<T>
{
    // Empty.
}
