use crate::matrix;
use matrix::interface;
use matrix::ops::householder;

use num::traits::real;
use std::ops;

struct FrancisDoubleStep<T> {
    U: matrix::Matrix<T>,
    T: matrix::Matrix<T>,
    Ut: matrix::Matrix<T>,
}

impl<T> FrancisDoubleStep<T> 
where
    T: real::Real + From<f32> 
{
    pub fn new(H: matrix::Matrix<T>) -> Self {
        let p: usize = H.col;

        while p > 2 {
            let q: usize = p-1;
            let s: T = H[q][q] + H[p][p];
            let t: T = (H[q][q] * H[p][p]) - (H[q][p] * H[p][q]);  

            let (x,y,z): _ = Precomputation::xyz(&H,s,t);

            for k in 0..p-3 {
                let P: _ = {
                    let ae1: _ = householder::AlphaEpsilonOne::manual(vec![x,y,z].into());
                    let helper: _ = householder::Helper::new(&H, k);
                    householder::Householder::from_ae1(ae1, &helper)
                }; 

                let r: usize = Self::max(1, k);
                
                
            }

        }

        unimplemented!()
    }

    fn max(lhs: usize, rhs: usize) -> usize {
        if lhs >= rhs {
            lhs
        } else {
            rhs
        }
    }
}

struct Precomputation<T>(std::marker::PhantomData<T>);

impl<T> Precomputation<T> 
where
    T: real::Real + From<f32> 
{
    fn xyz(H: &matrix::Matrix<T>, s: T, t: T) -> (T,T,T) {

        let x: T = num::pow(H[1][1], 2) 
            + (H[0][1]*H[2][1]) 
            - (s * H[0][0]) 
            + t;
        let y: T = H[1][0] * (H[0][0] + H[1][1] - s);
        let z: T = H[1][0] * H[2][1];
        (x,y,z)
    }

}
