mod vector_space;
mod concrete;

pub use vector_space::*;
pub use concrete::*;

use iter::FromIterator;
use ops::{Index, IndexMut};
use std::{iter, ops};

pub trait Vector<T>
{
    // Supertrait.
}

impl<T, U> Vector<T> for U
where
    U: VSubscript<T> + VIterable<T>,
{
    // Empty.
}

pub trait VSubscript<T>: Index<usize, Output = T> + IndexMut<usize> {
    // Supertrait.
}

impl<T, U> VSubscript<T> for U
where
    U: Index<usize, Output = T> + IndexMut<usize>,
{
    // Empty.
}

pub trait VIterable<T>: IntoIterator<Item = T> + FromIterator<T> {
    // Supertrait.
}

impl<T, U> VIterable<T> for U
where
    U: IntoIterator<Item = T> + FromIterator<T>,
{
    // Empty.
}
