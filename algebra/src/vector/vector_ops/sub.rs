use crate::vector;
use std::ops;

impl<T> ops::Sub<Self> for vector::Vector<T> 
where
    T: ops::Sub<T,Output=T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs)
            .map(|(x,y)| {
                x-y
            }).collect::<Vec<T>>().into()
    }
}

#[test] fn test_vector_subtraction()
{
    let k: vector::Vector<isize> = vec![4,5].into();
    let v: vector::Vector<isize> = vec![12,2].into();

    let test_vector: _ = v-k;
    let expected_vector: vector::Vector<_> = vec![8,-3].into();

    assert_eq!(test_vector, expected_vector); 
}