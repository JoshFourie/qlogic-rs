mod structure;

pub mod interface;

mod ops;

#[derive(Debug, PartialEq, Clone)]
pub struct Vector<T> {
    inner: Vec<T>
}

impl<T> Vector<T>{

    #[inline]
    pub fn push(&mut self, val: T) 
    {
        self.inner.push(val)
    } 

    #[inline]
    pub fn swap(&mut self, i: usize, j: usize) {
        self.inner.swap(i,j)
    }

}

impl<T> Into<Vec<T>> for Vector<T>
{
    fn into(self) -> Vec<T>
    {
        self.inner
    }
}

impl<T> Default for Vector<T>
{
    #[inline]
    fn default() -> Self 
    {
        Self { inner: Vec::default() }
    }

}

impl<T> interface::Length<usize> for Vector<T>
{
    #[inline] fn len(self) -> usize { self.inner.len() }
}

impl<'a,T> interface::Length<usize> for &'a Vector<T>
{
    #[inline] fn len(self) -> usize { self.inner.len() }
}
