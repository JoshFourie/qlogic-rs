use super::{ MatrixAlgebra, QuantumReal, QuantumUnit, Complex };
use std::ops::{ Add, Sub, Div, Mul };

pub(crate) fn real_hessenberg<T, M>(A: &M) -> Result<(M,M),M::Error>
where
    T: QuantumReal,
    M: MatrixAlgebra<T>,
    for<'a> &'a M: IntoIterator<Item=T>
{
    let mut M: M = A.clone();
    let mut Q_store: Vec<M> = Vec::new();
    let mut R_store: Vec<M> = Vec::new();
    let col_dim = M.col_dim();
    let row_dim = M.row_dim();

    for k in 0..col_dim?.sub(1) {
        let x: M = M::from(M.extract_col(k)?)
            .update(row_dim, Some(1))?;
        let alpha: T = x.get( Some(k.add(1)), Some(0))?
            .signum()
            .mul( x.eucl_norm() );
        let epsilon: M = {
            let mut e = vec![T::zero(); col_dim?];
            e[k] = T::one();
            M::from(e).update(row_dim, Some(1))?
        };
        let mu: M = x.subtraction(&epsilon.scalar(alpha)?)?;
        let mu_norm: T = mu.eucl_norm();
        let I: M = M.identity()?;
        let vvT: M = mu.kronecker(&mu)?;
        let Qk: M = I.subtraction(
            &vvT.scalar( T::one().add(T::one()).div(mu_norm.mul(mu_norm)) )?
        )?;
        let mut Q: M = Qk.cross(&M)?;
        for i in 0..row_dim? {
            Q.set(Some(k), Some(i), T::zero())?;
            Q.set(Some(i), Some(k), T::zero())?;
        }
        Q_store.push(Qk.transpose()?);
        R_store.push(Qk);

        Q.set(Some(k), Some(k), T::one())?;
        M = Q;
    }
    // we have to unwrap here because acc.
    let R: M = R_store.into_iter()
        .rev()
        .fold(M.identity()?, |acc,q| acc.cross(&q).unwrap());
    let Q: M = Q_store.into_iter()
        .fold(M.identity()?, |acc,q| acc.cross(&q).unwrap());
    Ok((Q,R))
}

use num_traits::identities::{ One, Zero};

pub fn complex_hessenberg<T,M>(A: &M) -> Result<(M,M),M::Error>
where
    T: num::Float,
    Complex<T>: QuantumUnit
    + Mul<T,Output=Complex<T>>,
    M: MatrixAlgebra<Complex<T>>,
    for<'a> &'a M: IntoIterator<Item=Complex<T>>
{
    let mut M: M = A.clone();
    let mut _Q: Vec<M> = Vec::new();
    let mut _R: Vec<M> = Vec::new();

    // for k in 0..M.dim() {
    for k in 0..M.dim()?-1 {
        let x: M = M.extract_col(k)?.into();
        let arg: T = x.get(Some(k+1), None)?.arg();
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