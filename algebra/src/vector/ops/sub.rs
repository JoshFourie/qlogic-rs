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

impl<'a,T: Clone> ops::Sub<Self> for &'a vector::Vector<T> 
where
    T: ops::Sub<T,Output=T>
{
    type Output = vector::Vector<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.inner
            .iter()
            .zip(rhs.inner.iter())
            .map(|(x,y)| {
                x.clone()-y.clone()
            }).collect::<Vec<T>>().into()
    }
}

#[test] 
fn test_vector_subtraction() {
    let k: vector::Vector<isize> = vec![14,0,0].into();
    let v: vector::Vector<isize> = vec![12,6,-4].into();

    let test_vector: _ = v-k;
    let expected_vector: vector::Vector<_> = vec![-2,6,-4].into();

    assert_eq!(test_vector, expected_vector); 
}

