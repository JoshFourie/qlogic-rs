use crate::matrix;
use matrix::interface;
use interface::{Identity, LinearSystem};

use crate::vector;

use std::ops;

impl<T:Copy> interface::Inverse for matrix::Matrix<T>
where
    T: num::Zero 
    + num::One 
    + ops::Div<Output=T>
    + ops::Mul<Output=T>
    + ops::Sub<Output=T>
    + ops::AddAssign<T>
    + num::Signed
    + PartialOrd<T>
{
    type Output = Self;

    fn inverse(self) -> Self::Output 
    {
        let mut buf: _ = (&self).identity();
        for col in 0..buf.col {
            let vec: vector::Vector<T> = buf[col].to_vec().into();
            let sol: Vec<T> = self.clone().solve(vec).into();
            buf[col].clone_from_slice(sol.as_slice())
        } 
        buf
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use interface::Inverse;
    use float_cmp::ApproxEq;

    #[test] fn test_inversion() 
    {
        let matrix: matrix::Matrix<f64> = vec![
            2.0, -1.0, 0.0, 
            -1.0, 2.0, -1.0, 
            0.0, -1.0, 2.0
        ].into();   

        let test: _ = matrix.inverse();

        let exp: matrix::Matrix<f64> = vec![
            3.0/4.0, 1.0/2.0, 1.0/4.0, 
            1.0/2.0, 1.0, 1.0/2.0, 
            1.0/4.0, 1.0/2.0, 3.0/4.0
        ].into();

        println!("{:?} {:?}", test, exp);

        for (t,e) in test.into_iter()
            .zip(exp)
        {
            if !t.abs().approx_eq(e.abs(), (0.001, 4)) {
                panic!("{:?} != {:?}", t, e)
            }
        }       
    }

}
