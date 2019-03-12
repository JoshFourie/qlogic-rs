use crate::math_primitives::vector::Vector;

#[derive(Debug,PartialEq)]
pub struct Qubit<T>
{
    state: Vector<T>
}

impl<T: Copy> Qubit<T>
{
    pub fn into_inner(self) -> Vector<T> { self.state }
}

impl<T> From<Vector<T>> for Qubit<T>
{
    fn from(state: Vector<T>) -> Self { Self{state} }
}

impl<T> From<Vec<T>> for Qubit<T>
{
    fn from(inner: Vec<T>) -> Self {
        Self {
            state: Vector::from(inner)
        }
    }
}