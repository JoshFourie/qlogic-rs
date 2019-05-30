#[macro_use] mod vector_ops;

pub struct Vector<T> 
{
    inner: Vec<T>
}

impl<T> From<Vec<T>> for Vector<T> 
{
    fn from(inner: Vec<T>) -> Self 
    {
        Vector { inner }
    }
}

impl_into_iter!(crate::vector::Vector<T>);
impl_into_iter!(&'a crate::vector::Vector<T>);
impl_into_iter!(&'a mut crate::vector::Vector<T>);