/********* Imports **************/

use super::*;

/********* Functions **********/
fn householder_reflector<T: Copy, M>(A: &M, k: usize) -> Result<M,M::Error>
where
    T: Float + Signed,
    M: BasicTransform<T>,
    for<'a> &'a M: IntoIterator<Item=T>
{
    let xi: M = M::from(
                A.extract_col(Some(k))?
            ).update(A.row_dim(),Some(1))?;  
    let alpha: T = xi.get(Some(k.add(1)), Some(0))?
        .signum()
        .mul(xi.eucl_norm());  
    let epsilon: M = {
        let mut _e = vec![T::zero(); A.col_dim()?];
        _e[k] = T::one();
        M::from(_e).update(A.row_dim(), Some(1))?
    };
    let mu: M = xi.subtraction(&epsilon.scalar(alpha)?)?;
    let eucl_norm: T = mu.eucl_norm();
    let vi: M = mu.apply_to_each(|u| u.div(eucl_norm))?;
    let I: M = A.identity()?;
    I.subtraction(&vi.kronecker(&vi)?.scalar(T::from(2)?)?)
}

pub(crate) fn qr_transform<T: Copy, M>(A: &M) -> Result<(M,M),M::Error>
where
    T: Float + Signed,
    M: BasicTransform<T>,
    for<'a> &'a M: IntoIterator<Item=T>
{
    let mut H: M = M::from(A.into_inner())
        .update(A.row_dim(), A.col_dim())?;
    let mut Q_store: Vec<M> = Vec::new();
    let mut R_store: Vec<M> = Vec::new();
    let col_dim = H.col_dim();
    let row_dim = H.row_dim();

    for k in 0..col_dim?.sub(1) {
        let P = householder_reflector(&H,k)?;
        let mut Q: M = P.cross(&H)?;
        for i in 0..row_dim? {
            Q.set(Some(k), Some(i), T::zero())?;
            Q.set(Some(i), Some(k), T::zero())?;
        }
        Q_store.push(P.transpose()?);
        R_store.push(P);
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

    // Could also return the eigenvectors here.
}

// returns UTU*.
pub fn francis_double_step_qr<T: Copy + Debug, M>(A: &M) -> Result<M,M::Error>
where
    T: Float + Signed,
    M: BasicTransform<T>,
    for<'a> &'a M: IntoIterator<Item=T>
{
    let mut H: M = A.into_inner().into();
    let n: usize = A.row_dim()?;
    let mut p: usize = A.row_dim()?.sub(1);

    while p > 2 
    {
        let q: usize = p.sub(1);

        let hqq: T = H.get(Some(q),Some(q))?;
        let hpp: T = H.get(Some(p),Some(p))?;
        let hqp: T = H.get(Some(q),Some(p))?;
        let hpq: T = H.get(Some(q),Some(p))?;       
        let h11: T = H.get(Some(0),Some(0))?;
        let h12: T = H.get(Some(0),Some(1))?;
        let h21: T = H.get(Some(1),Some(0))?;
        let h22: T = H.get(Some(1),Some(1))?;
        let h32: T = H.get(Some(2),Some(1))?;
        
        let s: T = hqq.add(hpp);
        let t: T = hqq.mul(hpp).sub(hqp.mul(hpq)); 
        
        let mut x: T = num::pow(h11,2)
            .add(h12.mul(h21))
            .sub(s.mul(h11))
            .add(t);
        let mut y: T = h21.mul(h11.add(h22).sub(s));
        let mut z: T = h21.mul(h32);

        for k in 0..p.sub(3)
        {            
            let P: Vec<T> = vec![x,y,z];

            // potential problem at this >= stage.
            let mut r: usize;

            if k >= 1 {
                r = k;
            } else { r = 1; };
            for i in k.add(1)..k.add(3) {
                for j in r..n {
                    let hij: T = H.get(Some(i),Some(j))?;
                    let pi: T = P[i];
                    let val: T = hij.mul(pi);
                    H.set(Some(i),Some(j),val)?;
                }
            }

            if k.add(4) <= p {
                r = k;
            } else { r = p; }
            for i in 0..r {
                for j in k.add(1)..k.add(3) {
                    let hij: T = H.get(Some(i),Some(j))?;
                    let pi: T = P[i];
                    let val: T = hij.mul(pi);
                    H.set(Some(i),Some(j),val)?;
                }
            }

            x = H.get(Some(k.add(2)),Some(k.add(1)))?;
            y = H.get(Some(k.add(3)),Some(k.add(1)))?;
            if k < p.sub(3) {
                z = H.get(Some(k.add(4)),Some(k.add(1)))?;
            }
        }

        // assumption.
        let P: Vec<T> = vec![x,y];

        for i in q..p {
            // double check 1..n;
            for mut j in 1..n {
                j = p.sub(j);
                let hij: T = H.get(Some(i),Some(j))?;
                let pi: T = P[i];
                let val: T = hij.mul(pi);
                H.set(Some(i),Some(j),val)?;
            }
        }

        // potential problem
        for i in 0..p {
            for mut j in 0..p {
                j = p.sub(j);
                let h: T = H.get(Some(i),Some(j))?;
                let p: T = P[i];
                let val: T = h.mul(p);
                H.set(Some(i),Some(j),val)?;
            }
        }

        // arbitrary error bounding.
        let eps: T = T::from(0.0001)?;
        let hpq: T = H.get(Some(p),Some(q))?.abs();
        let hqq: T = H.get(Some(q),Some(q))?.abs();
        let hpp: T = H.get(Some(p),Some(p))?.abs();

        if hpq < eps.mul(hqq.add(hpp)) {
            H.set(Some(p.sub(1)),Some(q.sub(1)),T::zero())?;
            p = p.sub(2);
            // q = p.sub(1);
        }
    }

    Ok(H)
}

pub fn james_langou_balance<T: Copy + Debug, M>(H: &M) -> Result<M,M::Error>
where
    T: Float,
    M: BasicTransform<T>,
    for<'a> &'a M: IntoIterator<Item=T>
{
    let mut A: M = H.into_inner().into();
    let mut I: M = A.identity()?;
    let mut converged: bool = false;
    let beta: T = T::from(2.0)?;
    
    while !converged {
        for i in 0..A.row_dim()? 
        {
            /* 
            let mut f = T::one();
            let mut c: T = A.extract_col(Some(i))?
                .into_iter()
                .fold(T::zero(), |acc,x| acc + num::pow(x,2))
                .sqrt();
            let mut r: T = A.extract_row(Some(i))?
                .into_iter()
                .fold(T::zero(), |acc,x| acc + num::pow(x,2))
                .sqrt();
            let s = num::pow(c,2).add(num::pow(r,2));
            */
            let mut c = T::zero();
            let mut r = T::zero();        
            for j in 0..A.row_dim()? {
                if j!=i { 
                    c = c.add( A.get(Some(j), Some(i))?.abs() ); 
                    r = r.add( A.get(Some(i), Some(j))?.abs() );
                }
            }
            let s = c.add(r);
            let mut f = T::one();

            while c < r.div(beta) {
                c = c.mul(beta);
                r = r.div(beta);
                f = f.mul(beta);
            }
            
            while c >= r.mul(beta) {
                c = c.div(beta);
                r = r.mul(beta);
                f = f.div(beta);
            }
            
            // if num::pow(c,2).add(num::pow(r,2)) < T::from(0.95)?.mul(s)
             if c.add(r) < T::from(0.95)?.mul(s)
            {
                let omega = f.mul(I.get(Some(i),Some(i))?);
                I.set(Some(i),Some(i), omega)?;

                for j in 0..A.col_dim()? {
                    
                    let theta = f.mul(A.get(Some(j), Some(i))?);
                    A.set(Some(j), Some(i), theta)?;

                    let xi = A.get(Some(i), Some(j))?.div(f);
                    A.set(Some(i), Some(j), xi)?;
                }
            } else { converged = true }
        }   
    }
    Ok(A)
}

// signum error that may carry through?
// the hessenberg step for the subroutine zeroes through the col 
// that has the error.
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
    let T: SquareMatrix<_> = james_langou_balance(&A).unwrap();
    // assert_eq!(E,T);
    for (exp,test) in E.into_iter()
        .zip(T.into_iter())
    {
        match exp.sub(test) < 0.0001 {
            true => { },
            false => { assert_eq!(exp,test) }
        }
    }
}

#[test]
fn test_francis()
{
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
    let B: _ = francis_double_step_qr(&A).unwrap();
    println!("{:?}", B);

    /* let M: Matrix<f32> = Matrix::from(vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0])
            .update(Some(3),Some(3))
            .unwrap();
    let T: Matrix<f32> = francis_double_step_qr(&M).unwrap();
    
    let Q: Matrix<f32> = Matrix::from(vec![0.8571, -0.3943, 0.3314, 0.4286, 0.9020, -0.0343, -0.2857, 0.1714, 0.9429])
        .update(Some(3), Some(3))
        .unwrap();
    let R: Matrix<f32> =  Matrix::from(vec![14.0, 21.0, -14.0, 0.0, 175.0, -70.0, 0.0, 0.0, -35.0])
        .update(Some(3),Some(3))
        .unwrap();
    let E: Matrix<f32> = Q.cross(&R)
        .unwrap()
        .update(Some(3),Some(3))
        .unwrap();
    let RQ: Matrix<f32> = R.cross(&Q)
        .unwrap();

    println!("{:?}", RQ);
    println!("{:?}", T);

    for (exp,test) in E.into_iter()
        .zip(RQ.into_iter())
    {
        match exp.sub(test) < 0.0001 {
            true => { },
            false => { assert_eq!(exp,test) }
        }
    }   */
}

#[test]
fn test_eigen() {
    let T: Vec<f64> = francis_double_step_qr(&Matrix::from(
        vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0])
            .update(Some(3),Some(3))
            .unwrap()
        ).unwrap()
            .update(Some(3),Some(3))
            .unwrap()
            .diagonal()
            .unwrap();

    let E: Vec<f64> = vec![16.05999, 34.19668, 156.13668];
    
    println!("{:?} {:?}", E,T);

    for (exp,test) in E.into_iter()
        .zip(T.into_iter())
    {
        match exp.sub(test) < 0.0001 {
            true => { },
            false => { assert_eq!(exp,test) }
        }
    }   
}