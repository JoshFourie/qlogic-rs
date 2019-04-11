/********* Imports **************/

use super::*;

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

pub fn parlett_reinsch_balance<T: Copy + Debug, M>(H: &M) -> Result<M,M::Error>
where
    T: Float,
    M: Square<T>,
    for<'a> &'a M: IntoIterator<Item=T>
{
    let b = T::one().add(T::one());
    let mut A: M = H.into_inner().into(); 

    let mut D = A.identity()?;
    let mut done=0;
    while done == 0 {     
        for j in 0..A.row_dim()? {        
            let mut c = T::zero();
            let mut r = T::zero();        
            for i in 0..A.row_dim()? {
                if i!=j { 
                    c = c.add( A.get(Some(i), Some(j))?.abs() ); 
                    r = r.add( A.get(Some(j), Some(i))?.abs() );
                }
            }
            let s = c.add(r);
            let mut f = T::one();
            while b.mul(c) < r {
                c = b.mul(c);
                r = r.div(b);
                f = b.mul(f);
            }
            while b.mul(r) < c {
                c = c.div(b);
                r = b.div(r);
                f = f.div(b);
            }
            println!("c: {:?} b: {:?} f: {:?} ", c, b, f);
            if c.add(r) < T::from(0.95)?.mul(s) {
                let sigma = f.mul(D.get(Some(j),Some(j))?);
                D.set(Some(j),Some(j),sigma)?;
// swapping omega/theta set().
                for n in 0..A.row_dim()? {
                    let omega = f.mul(A.get(Some(n), Some(j))?);
                    A.set(Some(n), Some(j), omega)?;

                    let theta = T::one().div(f).mul(A.get(Some(j), Some(n))?);
                    A.set(Some(j), Some(n), theta)?;
                }                
            } else { done=1 }
        }
    }
    Ok(A.scalar(T::one().add(T::one()))?)
}

#[test]
fn test_balance() {
    let A: SquareMatrix<_> = vec![
        -5.5849.mul(10_f64.powf(-1.0)),
        -2.4075.mul(10_f64.powf(7.0)),
        -6.1644.mul(10_f64.powf(14.0)),
        -6.6275.mul(10_f64.powf(0.0)),
        -7.1724.mul(10_f64.powf(-9.0)),
        -2.1248.mul(10_f64.powf(0.0)),
        -3.6083.mul(10_f64.powf(6.0)),
        -2.6435.mul(10_f64.powf(-6.0)),
        -4.1508.mul(10_f64.powf(-16.0)),
        -2.1647.mul(10_f64.powf(-7.0)),
        1.6229.mul(10_f64.powf(-1.0)),
        -7.6315.mul(10_f64.powf(-14.0)),
        4.3648.mul(10_f64.powf(-3.0)),
        1.2614.mul(10_f64.powf(6.0)),
        -1.1986.mul(10_f64.powf(13.0)),
        -6.2002.mul(10_f64.powf(-1.0))
    ].into();
    let E: SquareMatrix<_> = vec![
        -0.5585, -0.3587, -1.0950, 0.1036,
        -0.4813, -2.1248, -0.4313, 2.7719,
        -0.2337, -1.8158, 0.1623, -0.6713,
        0.2793, 1.2029, -1.3627, -0.6200 
    ].into();
    let T: _ = parlett_reinsch_balance(&A).unwrap();
    assert_eq!(E,T);
}