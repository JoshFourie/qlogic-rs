use crate::vector;

use std::ops;

impl<T> ops::Add<Self> for vector::Vector<T>
where
    T: ops::Add<Output=T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output
    {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a,b)| a+b)
            .collect::<Vec<T>>()
            .into()
    }
}