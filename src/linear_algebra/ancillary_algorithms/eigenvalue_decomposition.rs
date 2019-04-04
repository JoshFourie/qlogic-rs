/********* Imports **************/

use super::{ BasicTransform, Float};
use super::{Mul, Add, Sub, Div};

/********* Functions **********/

pub(crate) fn real_hessenberg<T: Copy, M>(A: &M) -> Result<(M,M),M::Error>
where
    T: Float + num::Signed,
    M: BasicTransform<T>,
    for<'a> &'a M: IntoIterator<Item=T>
{
    let mut H: M = M::from(Vec::new())
        .update(A.row_dim(), A.col_dim())?;
    for a in A.into_iter() {
        H.push(a)
    }    
    let mut Q_store: Vec<M> = Vec::new();
    let mut R_store: Vec<M> = Vec::new();
    let col_dim = H.col_dim();
    let row_dim = H.row_dim();

    for k in 0..col_dim?.sub(1) {
        let x: M = M::from(H.extract_col(k)?)
            .update(row_dim, Some(1))?;    
        let alpha: T = x.get(Some(k.add(1)), Some(0))?
            .signum()
            .mul( x.eucl_norm() );  
        let epsilon: M = {
            let mut e = vec![T::zero(); col_dim?];
            e[k] = T::one();
            M::from(e).update(row_dim, Some(1))?
        };
        let mu: M = x.subtraction(&epsilon.scalar(alpha)?)?;
        let mu_norm: T = mu.eucl_norm();
        let I: M = H.identity()?;
        let vvT: M = mu.kronecker(&mu)?;
        let Qk: M = I.subtraction(
            &vvT.scalar( T::one().add(T::one()).div(mu_norm.mul(mu_norm)) )?
        )?;
        let mut Q: M = Qk.cross(&H)?;
        for i in 0..row_dim? {
            Q.set(Some(k), Some(i), T::zero())?;
            Q.set(Some(i), Some(k), T::zero())?;
        }
        Q_store.push(Qk.transpose()?);
        R_store.push(Qk);
        Q.set(Some(k), Some(k), T::one())?;
        H = Q;
    }
    // we have to unwrap here because acc. doesn't work well with results.
    let R: M = R_store.into_iter()
        .rev()
        .fold(H.identity()?, |acc,q| acc.cross(&q).unwrap())
        .cross(A)?;
    let Q: M = Q_store.into_iter()
        .fold(H.identity()?, |acc,q| acc.cross(&q).unwrap());
    Ok((Q,R))
}

/*
// TODO: adjust for changes to function as above.
pub fn complex_hessenberg<T: Copy,M>(A: &M) -> Result<(M,M),M::Error>
where
    T: Float,
    Complex<T>: Num + Copy,
    M: BasicTransform<Complex<T>>,
    for<'a> &'a M: IntoIterator<Item=Complex<T>>
{

    let mut M: M = A.clone();
    let mut _Q: Vec<M> = Vec::new();
    let mut _R: Vec<M> = Vec::new();
    // for k in 0..M.dim() {
    for k in 0..M.dim()?-1 {
        let x: M = M.extract_col(k)?.into();
        let arg: T = x.get(Some(k+1), Some(1))?.arg();
        let alpha: Complex<T> = -( Complex::<T>::i().mul(arg) ).exp().mul(x.eucl_norm());
        let epsilon: M = {
            let mut _e = vec![Complex::zero(); M.dim()?];
            _e[k]=Complex::one();
            _e.into()
        };
        let mu: M = x.subtraction(&epsilon.scalar(alpha)?)?;
        let mu_norm: Complex<T> = mu.eucl_norm();
        let I = M.identity()?;
        let vvT: M = mu.kronecker(&mu)?;
        let Qk: M = I.subtraction(&vvT.scalar( (Complex::<T>::one()+Complex::one()).div(mu_norm*mu_norm))?)?;

        let mut Q = Qk.cross(&M)?;
        for i in 0..Q.dim()? {
            Q.set(Some(k),Some(i),Complex::zero())?;
            Q.set(Some(i),Some(k),Complex::zero())?;
        }
        _R.push(Qk.clone());
        _Q.push(Qk);
        
        Q.set(Some(k),Some(k),Complex::one())?;
        M = Q;
    }
    let R: M = _R.into_iter()
        .rev()
        .fold(M.identity(), |acc: Result<M,M::Error>, q| -> Result<M, M::Error> 
        {
            acc?.cross(&q)
        })?;      
    let Q: M = _Q.into_iter()
        .fold(M.identity(), |acc: Result<M,M::Error>, q| -> Result<M, M::Error> 
        {
            acc?.cross(&q)?.transpose()
        })?;
    Ok((Q,R))
}
*/

/* 
/* P.cross(
            &psi.update(Some(l), Some(1))?
        )?.into_iter()
        .collect::<Vec<T>>();
*/
// adjusted for zero-indexing at delta constructor and phi constructor.
// we need to add 1 at each for loop.
pub fn francis_double_step<T, M>(H: &mut M) -> Result<(M,M), M::Error>
where
    T: QuantumReal,
    M: MatrixAlgebra<T>,
    for<'a> &'a M: IntoIterator<Item=T>,
{
    // let delta = H.col_dim()?.sub(1);
    for delta in 2..H.col_dim()?.sub(1) {
        let phi = delta.sub(1);
        let sigma = H.get(Some(phi), Some(phi))?
            .add( H.get(Some(delta), Some(delta))?);      
        let theta = H.get(Some(phi), Some(phi))?
            .mul( H.get(Some(delta), Some(delta))?)
            .sub( 
                H.get(Some(phi), Some(delta))?
                    .mul(H.get(Some(delta), Some(phi))?)
            );

        let mut xi = H.get(Some(1), Some(1))?
            .pow64(2.0)
            .add( 
                H.get(Some(1), Some(2))?
                    .mul(H.get(Some(2), Some(1))?)
            ).sub(
                H.get(Some(1), Some(1))?
                    .mul(sigma)
            ).add(theta);
        let mut gamma = H.get(Some(2), Some(1))?
            .mul(
                H.get(Some(1), Some(1))?
                    .add(H.get(Some(2), Some(2))?)
                    .sub(sigma)
            );
        let mut zeta = H.get(Some(2), Some(1))?
            .mul(H.get(Some(3), Some(2))?);

        for kappa in 0..delta.sub(3) {            
            let refl = M::from(vec![xi, gamma, zeta])
                    .update(Some(3), Some(1))?;

            let rho_max = max(1, kappa);
            let alpha_range = kappa.add(1)..kappa.add(3).add(1);
            let beta_range = rho_max..H.dim()?.add(1);
            let l = kappa.add(3)
                .sub(kappa)
                .add(
                    H.dim()?.add(1)
                        .sub(rho_max)
                );

            let psi: M = H.get_sub_matrix(Some(alpha_range.clone()), Some(beta_range.clone()))?
                .update(Some(l), Some(1))?;
            let varho: Vec<T> = refl
                .cross(&psi)?
                .into_iter()
                .collect();
            H.set_sub_matrix(Some(alpha_range), Some(beta_range), varho)?;
            
            let rho_min = min(kappa.add(4), delta);
            let sigma_range = 1..rho_min.add(1);
            let theta_range = kappa.add(1)..kappa.add(3).add(1);
            let l = rho_min.add(
                kappa.add(3)
                    .sub(kappa)
            );
            let omega: M = H.get_sub_matrix(Some(sigma_range.clone()), Some(theta_range.clone()))?
                .update(Some(l), Some(1))?;
            let varho: Vec<T> = omega
                .cross( &refl )?
                .into_iter()
                .collect();
            H.set_sub_matrix(Some(sigma_range), Some(theta_range), varho)?;

            xi = H.get(Some(kappa.add(2)), Some(kappa.add(1)))?;
            gamma = H.get(Some(kappa.add(3)), Some(kappa.add(1)))?;

            if kappa < delta.sub(3) {
                zeta = H.get(Some(kappa.add(4)), Some(kappa.add(1)))?;
            }
        }   
        
        let givens_rotator = M::from(vec![xi, gamma])
            .update(Some(2), Some(1))?;
        
        let alpha_range = phi..delta.add(1);
        let beta_range = delta.sub(2)..H.dim()?;
        let l: usize = delta.add(1)
            .sub(phi)
            .add(
                H.dim()?.sub(delta.sub(2))
            );
        let psi: M = H.get_sub_matrix(Some(alpha_range.clone()), Some(beta_range.clone()))?;
        let varho: Vec<T> = givens_rotator
            .cross(&psi)?
            .into_iter()
            .collect();
        H.set_sub_matrix(Some(alpha_range), Some(beta_range), varho)?;

        let gamma_range = 1..delta.add(1);
        let zeta_range = delta.sub(1)..delta.add(1);
        let omega: M = H.get_sub_matrix(Some(gamma_range.clone()), Some(zeta_range.clone()))?
            .update(Some(l), Some(1))?;
        let varho: Vec<T> = omega
            .cross( &givens_rotator )?
            .into_iter()
            .collect();
        H.set_sub_matrix(Some(gamma_range), Some(zeta_range), varho)?;
        let x = H.get(Some(delta), Some(phi))?;
    }
    Ok(( H.transpose()?, H.clone() ))
}   

fn max<T: PartialOrd>(alpha: T, beta: T) -> T
{
    if alpha == beta {
        alpha
    } else {
        match alpha > beta {
            true => alpha,
            false => beta
        }
    }
}

fn min<T: PartialOrd> (alpha: T, beta: T) -> T
{
    if alpha == beta {
        alpha
    } else {
        match alpha < beta {
            true => alpha,
            false => beta
        }
    }
}

// 3381

#[test]
fn test_francis_step()
{
    use super::{matrix::Matrix, MatrixAlgebra};
    let mut M: Matrix<f64> = Matrix::from(
        vec![
            7.0, 3.0, 4.0, -11.0, -9.0, -2.0, -6.0, 4.0, -5.0, 7.0, 1.0, 12.0,
            -1.0, -9.0, 2.0, 2.0, 9.0, 1.0, -8.0, 0.0, -1.0, 5.0, 0.0, 8.0,
            -4.0, 3.0, -5.0, 7.0, 2.0, 10.0, 6.0, 1.0, 4.0, -11.0, -7.0, -1.0
        ]
    ).update(Some(6), Some(6)).unwrap();
    francis_double_step(&mut M).unwrap();
}

*/





















































/* 
pub(crate) fn francis_double_step<T,M>(A: &M) -> Result<M,M::Error>
where
    T: QuantumReal + std::fmt::Debug,
    M: MatrixAlgebra<T>,
    for<'a> &'a M: IntoIterator<Item=T>
{
    use std::ops::RangeInclusive;

    let (Q,R) = real_hessenberg::<T,M>(A)?;
    let mut H = R.cross(A)?.cross(&Q)?;
    let p = A.dim();
    let n = A.dim();

    let reflect = |P: M, alpha: RangeInclusive<usize>, beta: RangeInclusive<usize>| -> Result<(),M::Error> 
    {
        let phi = Vec::new();
        for i in alpha {
            for j in beta {
                phi.push(
                    H.get(i,j)?
                )
            }
        }
        let rho = P.matrix_product(phi.into())?
            .into_iter()
            .collect::<Vec<_>>();
        let mut l = 0;
        for i in alpha {
            for j in beta {
                let delta = rho[l];
                H.set(Some(i),Some(j),delta)?;
            }
        }
        Ok(())
    };

    // reduce all indexing by 1. !!!!!!!!!!!!!!!!!!!!
    while p > 1 
    {
        let q: usize = p.sub(1);
        let s: T = H.get(q,q)?
            .add(H.get(p,p)?);
        let t: T = H.get(q,q)?
            .mul(H.get(p,p)?)
            .sub( H.get(q,p)?.mul(H.get(p,q)?) );
        let mut x: T = H.get(0,0)?
            .pow64(2.0)
            .add( H.get(0,1)?.mul( H.get(1,0)? ) )
            .sub(s.mul(H.get(0,0)?))
            .add(t);
        let mut y: T = H.get(1,0)?
            .mul( H.get(0,0)?.add(H.get(1,1)?).sub(s) );
        let mut z: T = H.get(1,0)?
            .mul(H.get(2,1)?);
        let mut hh_reflector: V = vec![x,y,z].into();
        let mut givens_reflector: V = vec![x,y].into();
        for k in 0..p.sub(4)
        {
            if k > 1 { 
                reflect(hh_reflector, k.add(1)..=k.add(3), k..=n )? 
            } else {
                reflect(hh_reflector, 2..=4, 1..=n)?
            };

            if k.add(4) < p { 
                reflect(hh_reflector, 1..=k, k.add(1)..=k.add(3) )?
            } else {
                reflect(hh_reflector, 1..=p, p.add(1)..=p.add(3) )?
            };

            x = H.get(k.add(2),k.add(1))?;
            y = H.get(k.add(3), k.add(1))?;
            if k < p.sub(3) { 
                z = H.get(k.add(4), k.add(1))?; 
            }
        }
        reflect(givens_reflector, q..=p, p.sub(2)..=n)?;

        // if the enclosure accepts a function this can be cleaner.
        let phi = Vec::new();
        for i in 1..=p {
            for j in p.sub(1)..p {
                phi.push(
                    H.get(i,j)?
                )
            }
        }
        let rho = H.vector_product(givens_reflector)?
            .into_iter()
            .collect::<Vec<_>>();
        let mut l = 0;
        for i in 1..=p {
            for j in p.sub(1)..p {
                let delta = rho[l];
                H.set(Some(i),Some(j),delta)?;
            }
        }
        // There's some convergence checking here.
    };
    Ok(H)
} */