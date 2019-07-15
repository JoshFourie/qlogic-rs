use crate::vector;

use vector::interface;

impl<T: Clone> interface::Norm<T> for vector::Vector<T>
where
    T: num::traits::real::Real
{
    fn eucl_norm(self) -> T
    {
        self.into_iter()
            .fold(
                T::zero(), |acc,x| acc + num::pow(x,2) 
            ).sqrt()
    }
}