use super::{ Vector, ComplexMatrix, Matrix, QuantumUnit, VectorAlgebra, ComplexMatrixAlgebra, MatrixAlgebra };
use super::error::{ VectorError, MatrixError };
use num::{Float, Complex};
use std::ops::Div;

struct Eigen<T> {
    values: Vec<T>,
    vectors: Vector<T>,
}

struct KrylovSpace<T> {
    inner: Vec<T>
}

impl<T:QuantumUnit> Eigen<Complex<T>> 
where
    ComplexMatrix<T>: MatrixAlgebra<T>
{
    /* fn householder_factor(mat: &mut Matrix<T>, v: &mut Vector<T>)
    {
        let n = v.len();
        for i in 0..n {
            for j in 0..n {
                let val = -(T::one()+T::one())*v.get(i).unwrap()*v.get(j).unwrap();
                mat.set(i,j,val).unwrap();
            }
        }
        for i in 0..n {
            let val = mat.get(i,i).unwrap() + T::one();
            mat.set(i,i,val).unwrap();
        }
    } 

    fn househoulder(mat: &mut Matrix<T>, R: &mut Matrix<T>, Q: &mut Matrix<T>)
    {
        let m: usize = mat.dim();
        let qv: Vec<Matrix<T>> = Vec::new();
        let z: Matrix<T> = mat.clone();
        let mut z1: Matrix<T> = vec![T::zero(); m].into();
        for k in 0..m {
            let mut e: Vector<T> = vec![T::zero(); m].into();
            let mut x: Vector<T> = vec![T::zero(); m].into();
            let mut a: T;
            z1.minor(&z, k);
            let x: Vector<T> = z1.extract_col(k).into();
            a = x.eucl_dist();
            if mat.get(k,k).unwrap() != T::zero() {
                a = -a
            }
            for i in 0..e.len() {
                
            }
        }
    } */
}