use crate::matrix;
use matrix::interface;
use interface::Minor;
use matrix::ops::{minor,householder};

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
    pub fn new(mut H: matrix::Matrix<T>) -> Self {
        let n: usize = H.row;
        let p: usize = H.col;

        while p > 2 {
            let q: usize = p-1;
            let s: T = H[q][q] + H[p][p];
            let t: T = (H[q][q] * H[p][p]) - (H[q][p] * H[p][q]);  

            let (mut x, mut y, mut z): _ = Precomputation::xyz(&H,s,t);

            for k in 0..p-3 {
                let house: _ = FrancisDoubleStep::householder(&H, k, (x,y,z));
                FrancisDoubleStep::map_to_minors(&mut H, house, k, p);
                
                x = H[k+2][k+1];
                y = H[k+3][k+1];
                if k < p-3 {
                    z = H[k+4][k+1]
                }
            }

        }

        unimplemented!()
    }

    fn householder(mat: &matrix::Matrix<T>, k: usize, xyz: (T,T,T)) -> matrix::Matrix<T> {
        let (x,y,z): _ = xyz;
        let ae1: _ = householder::AlphaEpsilonOne::manual(vec![x,y,z].into());
        let helper: _ = householder::Helper::new(&mat, k);
        householder::Householder::from_ae1(ae1, &helper).into()
    }

    fn map_to_minors(mat: &mut matrix::Matrix<T>, house: matrix::Matrix<T>, k: usize, p: usize) {
        let r: usize = max(1, k);
        let range: _ = minor::MinorRange::new(k+1..k+3, r..mat.row);
        let minor: minor::MatrixMinor<T> = mat.minor(range);
        let buf: _ = house.clone() * minor;
        mat.clone_from_minor(buf);
        
        let r: usize = min(k+4, p);
        let range: _ = minor::MinorRange::new(0..r, k+1..k+3);
        let minor: minor::MatrixMinor<T> = house.minor(range);
        let buf: _ = minor * house;
        mat.clone_from_minor(buf);
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

fn max(lhs: usize, rhs: usize) -> usize {
    if lhs >= rhs {
        lhs
    } else {
        rhs
    }
}

fn min(lhs: usize, rhs: usize) -> usize {
    if lhs <= rhs {
        lhs
    } else {
        rhs
    }
}
