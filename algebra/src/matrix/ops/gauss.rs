//! Docs: InProgress, view src.
use crate::matrix;
use crate::vector;
use matrix::interface;
use interface::{LU, ForwardSubstitution, BackwardSubstitution};

use std::ops;

impl<T:Copy> interface::LinearSystem<T> for matrix::Matrix<T> 
where
    T: num::Zero    
    + num::One
    + num::Signed
    + ops::Div<T,Output=T>
    + ops::Sub<T,Output=T>
    + ops::Mul<T,Output=T>
    + ops::AddAssign
    + PartialOrd<T>
{
    type Output = Self::Vector;

    type Vector = vector::Vector<T>;

    fn solve(self, rhs: Self::Vector) -> Self::Output {
        let (P,L,U) = self.lu();
        let buf = L.forward_substitution(P*rhs);
        U.backward_substitution(buf)
    }
}

// pseudo-code: http://mathfaculty.fullerton.edu/mathews/n2003/BackSubstitutionMod.html
impl<T:Copy> interface::ForwardSubstitution<T> for matrix::Matrix<T>
where
    T: num::Zero
    + ops::Div<Output=T>
    + ops::Sub<Output=T>
    + ops::Mul<Output=T>
    + ops::AddAssign
{
    type Output = vector::Vector<T>;

    type Vector = vector::Vector<T>;

    fn forward_substitution(self, rhs: vector::Vector<T>) -> Self::Output
    {
        let mut x: vector::Vector<T> = Vec::new().into();
        for i in 0..self.row {
            let dot: T = {
                let mut val: T = T::zero();
                for j in 0..i {
                    val += self[i][j] * x[j];
                }
                val
            };
            x.push(
                (rhs[i] - dot)/self[i][i]
            );      
        }
        x
    }   
}

impl<T:Copy> interface::BackwardSubstitution<T> for matrix::Matrix<T>
where
    T: ops::Div<Output=T>
    + ops::Mul<Output=T>
    + ops::Sub<Output=T>
    + ops::AddAssign
    + num::Zero   
{
    type Output = Self::Vector;

    type Vector = vector::Vector<T>;
    
    fn backward_substitution(self, b: Self::Vector) -> Self::Output
    {
        let mut x: vector::Vector<T> = vec![T::zero(); self.row].into();

        for i in (0..self.row).rev()
        {
            let dot: T = {
                let mut sigma: T = T::zero();
                for j in i+1..self.row {
                    sigma += self[i][j] * x[j]
                }
                sigma
            }; 

            x[i] = (b[i] - dot)/self[i][i]
        }

        x
    }
}

impl<'a, T:Copy> interface::BackwardSubstitution<T> for &'a matrix::Matrix<T>
where
    T: ops::Div<Output=T>
    + ops::Mul<Output=T>
    + ops::Sub<Output=T>
    + ops::AddAssign<T>
    + num::Zero   
{
    type Output = vector::Vector<T>;

    type Vector = vector::Vector<T>;

    fn backward_substitution(self, b: Self::Vector) -> Self::Output
    {
        let mut x: vector::Vector<T> = vec![T::zero(); self.row].into();

        for i in (0..self.row).rev()
        {
            let dot: T = {
                let mut sigma: T = T::zero();
                for j in i+1..self.row {
                    sigma += self[i][j] * x[j]
                }
                sigma
            }; 

            x[i] = (b[i] - dot)/self[i][i]
        }

        x
    }
}

#[cfg(test)] mod tests
{
    use crate::matrix;

    use matrix::interface::{LU, LinearSystem, ForwardSubstitution, BackwardSubstitution};

    use crate::vector;

    use float_cmp::ApproxEq;

    #[test] fn test_forward_substitution() {
        let matrix: matrix::Matrix<f64> = vec![
            3.0, 0.0, 0.0, 0.0,
            -1.0, 1.0, 0.0, 0.0,
            3.0, -2.0, -1.0, 0.0,
            1.0, -2.0, 6.0, 2.0
        ].into();

        let vector: vector::Vector<f64> = vec![
            5.0, 6.0, 4.0, 2.0
        ].into();

        let exp: vector::Vector<f64> = vec![
            5.0/3.0, 23.0/3.0, -43.0/3.0, 305.0/6.0, 
        ].into();

        let test: _ = matrix.forward_substitution(vector);

        for (test,exp) in test.into_iter()
            .zip(exp)
        {
            if !test.approx_eq(exp, (0.001, 4)) {
                panic!("{} != {}", test, exp)
            }
        }
    }

    #[test] fn test_backward_substitution() 
    {
        let matrix: matrix::Matrix<f64> = vec![
            4.0, -1.0, 2.0, 3.0,
            0.0, -2.0, 7.0, -4.0,
            0.0, 0.0, 6.0, 5.0,
            0.0, 0.0, 0.0, 7.0
        ].into();

        let vector: vector::Vector<f64> = vec![
            20.0, -7.0, 4.0, 6.0
        ].into();

        let exp: vector::Vector<f64> = vec![
            4.78571, 1.61905, -0.047_619, 0.857_143
        ].into();

        let test: _ = matrix.backward_substitution(vector);

        for (test,exp) in test.into_iter()
            .zip(exp)
        {
            if !test.approx_eq(exp, (0.001, 4)) {
                panic!("{} != {}", test, exp)
            }
        }
    }

    #[test]
    fn test_solve_by_lu() {
        let A: matrix::Matrix<f32> = vec![
            1.0, 3.0, 1.0,
            2.0, 2.0, -1.0,
            2.0, -1.0, 0.0
        ].into();

        let (P,L,U) = {
            let (P,L,U) = A.lu();

            let exp_P: matrix::Matrix<f32> = vec![
                0.0, 1.0, 0.0,
                0.0, 0.0, 1.0, 
                1.0, 0.0, 0.0, 
            ].into();
            assert_eq!(P, exp_P);
            
            let exp_L: matrix::Matrix<f32> = vec![
                1.0, 0.0, 0.0,
                1.0, 1.0, 0.0,
                0.5, -2.0/3.0, 1.0
            ].into();
            assert_eq!(L, exp_L);


            let exp_U: matrix::Matrix<f32> = vec![
                2.0, 2.0, -1.0,
                0.0, -3.0, 1.0,
                0.0, 0.0, 13.0/6.0 
            ].into();
            assert_eq!(U, exp_U);

            (exp_P,L,U)
        };

        let b: vector::Vector<f32> = {
            let buf: vector::Vector<f32> = vec![
                1.0, -3.0, 3.0
            ].into(); 
            P * buf
        };
        let exp_b: vector::Vector<f32> = vec![
            -3.0, 3.0, 1.0
        ].into();
        assert_eq!(b, exp_b);
    }

    #[test]
    fn test_solve_linear_system() {
        let A: matrix::Matrix<f32> = vec![
            1.0, 3.0, 1.0,
            2.0, 2.0, -1.0,
            2.0, -1.0, 0.0
        ].into();
        let rhs: vector::Vector<f32> = vec![1.0, -3.0, 3.0].into();

        let test = A.solve(rhs);
        let exp: vector::Vector<f32> = vec![1.0, -1.0, 3.0].into();
        assert_eq!(test, exp)
    }
}