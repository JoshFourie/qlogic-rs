use crate::vector;

use std::{ops, iter};

impl<T> iter::IntoIterator for vector::Vector<T> {

    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter { self.inner.into_iter() }

}

impl<T> From<Vec<T>> for vector::Vector<T>
{
    fn from(inner: Vec<T>) -> Self
    {
        Self { 
            inner: inner
        }
    }
}

impl<T> ops::Index<usize> for vector::Vector<T> 
{
    type Output = T;

    fn index<'a>(&'a self, idx: usize) -> &'a Self::Output { &self.inner[idx] }
}

impl<T> ops::IndexMut<usize> for vector::Vector<T> {

    fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output 
    {
        &mut self.inner[idx]
    }

}

impl<'a, T> ops::Index<usize> for &'a vector::Vector<T>
{
    type Output = T;

    fn index<'b>(&'b self, idx: usize) -> &'b Self::Output { &self.inner[idx] }
}

impl<'a, T> ops::Index<usize> for &'a mut vector::Vector<T>
{
    type Output = T;

    fn index<'b>(&'b self, idx: usize) -> &'b Self::Output { &self.inner[idx] }
}


impl<'a, T> ops::IndexMut<usize> for &'a mut vector::Vector<T>
{
    fn index_mut<'b>(&'b mut self, idx: usize) -> &'b mut Self::Output 
    {
        &mut self.inner[idx]
    }
}