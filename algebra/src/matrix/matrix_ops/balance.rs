use crate::matrix;

use matrix::interface;

use interface::{Identity, Row, Column, Norm};

use std::{ops, convert};

impl<T: Clone> interface::Norm<T> for Vec<T>
where
    T: num::Float
{
    fn eucl_norm(self) -> T
    {
        self.into_iter()
            .fold(
                T::zero(), |acc,x| acc + num::pow(x,2) 
            ).sqrt()
    }
}

impl<T: Copy> interface::Balance for matrix::Matrix<T>
where
    T: num::Float
    + ops::DivAssign
    + ops::MulAssign
    + ops::AddAssign
    + convert::From<f32>
{
    type Output = Self;

    fn balance(mut self) -> Self::Output
    {
        let n: usize = self.row;

        let beta: T = <T as convert::From<_>>::from(2.0);

        let mut D: Self = (&self).identity();

        let mut converged: bool = false;

        while !converged {

            converged = true;

            for i in 0..n {

                let mut c: T = (&self).get_col(i).eucl_norm();

                let mut r: T = (&self).get_row(i).eucl_norm();

                let s: T = num::pow(c,2) + num::pow(r,2);

                let mut f: T = T::one();

                while c < r/beta {
                    c *= beta;
                    r /= beta;
                    f *= beta;
                }

                while c >= r * beta {
                    c /= beta;
                    r *= beta;
                    f /= beta;
                }

                if (num::pow(c,2) + num::pow(r,2)) < <T as convert::From<_>>::from(0.95) * s {
                    
                    converged = false;

                    D[i][i] *= f;

                    for j in 0..n {
                        self[j][i] *= f;
                        self[i][j] /= f;
                    }
                }
            }
        }

        // A = D^(-1)AD

        unimplemented!()
    }
}

#[ignore] #[test] fn test_balance() {

    use interface::Balance;

    let T1: matrix::Matrix<f64> = matrix::Matrix {
        inner: vec![
            -5.5849 * 10_f64.powf(-1.0),
            -2.4075 * 10_f64.powf(7.0),
            -6.1644 * 10_f64.powf(14.0),
            -6.6275 * 10_f64.powf(0.0),
            -7.1724 * 10_f64.powf(-9.0),
            -2.1248 * 10_f64.powf(0.0),
            -3.6083 * 10_f64.powf(6.0),
            -2.6435 * 10_f64.powf(-6.0),
            -4.1508 * 10_f64.powf(-16.0),
            -2.1647 * 10_f64.powf(-7.0),
            1.6229 * 10_f64.powf(-1.0),
            -7.6315 * 10_f64.powf(-14.0),
            4.3648 * 10_f64.powf(-3.0),
            1.2614 * 10_f64.powf(6.0),
            -1.1986 * 10_f64.powf(13.0),
            -6.2002 * 10_f64.powf(-1.0)
        ],
        row: 4,
        col: 4
    };
    let E: matrix::Matrix<_> = matrix::Matrix {
        inner: vec![
            -0.5585, -0.3587, -1.0950, 0.1036,
            -0.4813, -2.1248, -0.4313, 2.7719,
            -0.2337, -1.8158, 0.1623, -0.6713,
            0.2793, 1.2029, -1.3627, -0.6200 
        ],
        row: 4,
        col: 4
    };
    let C: _ = T1.balance();

    for (exp,test) in E.into_iter()
        .zip(C.into_iter())
    {
        match exp - test < 0.0001 {
            true => { },
            false => { assert_eq!(exp,test) }
        }
    }
}