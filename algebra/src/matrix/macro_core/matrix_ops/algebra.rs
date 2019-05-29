macro_rules! impl_transpose 
{    
    ($id:ty) => {
        impl<'a, T: Copy> crate::interface::Transpose for $id 
        {
            type Output = crate::matrix::Matrix<T>;
            
            fn transpose(self) -> Self::Output {
                let mut C: Vec<T> = Vec::new();
                let (r,c): (usize,usize) = (self.row, self.col);
                for i in 0..r {
                    for j in 0..c {
                        C.push(self[j][i])
                    }
                }
                crate::matrix::Matrix {
                    inner: C,
                    row: r,
                    col: c
                }
            }    
        }
    }
}

impl_transpose!(crate::matrix::Matrix<T>);
impl_transpose!(&'a crate::matrix::Matrix<T>);
impl_transpose!(&'a mut crate::matrix::Matrix<T>);

macro_rules! impl_diagonal 
{
    ($id:ty) => {
        impl<'a, T: Copy> crate::interface::Diagonal<T> for $id
        where   
            T: num::Zero + std::ops::Add<Output=T>
        {
            type Output = Vec<T>;

            fn diagonal(self) -> Self::Output
            {
                let mut diag: Vec<T> = Vec::new();
                for i in 0..self.col {
                    diag.push(self[i][i])
                }
                diag
            }

            fn trace(self) -> T
            {
                self.diagonal()
                    .into_iter()
                    .fold(T::zero(), |acc, x| acc + x)
            }
        }
    }
}

impl_diagonal!(crate::matrix::Matrix<T>);
impl_diagonal!(&'a crate::matrix::Matrix<T>);
impl_diagonal!(&'a mut crate::matrix::Matrix<T>);

macro_rules! impl_kronecker 
{
    ($id:ty) => {
        impl<'a, T: Copy> crate::interface::Kronecker<$id> for $id
        where   
            T: num::Zero + std::ops::Mul<Output=T>
        {
            type Output = crate::matrix::Matrix<T>;

            fn kronecker(self, rhs: $id) -> Self::Output
            {
                let m = self.row;
                let p = rhs.row;
                let n = self.col;
                let q = rhs.col;

                let mut N: crate::matrix::Matrix<T> = crate::matrix::Matrix {
                    inner: vec![T::zero(); m*n*p*q],
                    row: m*p,
                    col: n*q
                };

                for i in 1..=m*p as usize {
                    for j in 1..=n*q as usize 
                    {
                        let i = i as f64;
                        let j = j as f64;

                        let a1 = (((i - 1.0) / (p as f64)).floor() + 1.0) as usize - 1;
                        let a2 = (((j - 1.0) / (q as f64)).floor() + 1.0) as usize - 1;
                        let b1 = ((i - 1.0) % (p as f64) + 1.0) as usize - 1;
                        let b2 = ((j - 1.0) % (q as f64) + 1.0) as usize - 1;
                        
                        let alpha = self[a1][a2];
                        let beta = rhs[b1][b2];
                        let delta = alpha*beta;
                        N[i as usize - 1][j as usize -1] = delta;
                    }
                }
                N
            }
        }
    }
}

impl_kronecker!(crate::matrix::Matrix<T>);
impl_kronecker!(&'a crate::matrix::Matrix<T>);
impl_kronecker!(&'a mut crate::matrix::Matrix<T>);

/* impl_inverse_for_crate::matrix::matrix!(crate::matrix::Matrix<T>); */

#[test] fn test_inversion() 
{
    use crate::interface::PLUDecomposition;

    let T: crate::matrix::Matrix<f32> = 
    {
        let a: Vec<f32> = vec![
            1.0, 0.5, 0.3333333, 0.25, 0.2, 
            0.5, 0.33333, 0.25, 0.2, 0.16666667, 
            0.333333, 0.25, 0.2, 0.1666667, 0.14285714286,
            0.25, 0.2, 0.166667, 0.14285714286, 0.125,
            0.25, 0.166667, 0.14285714286, 0.125, 0.11111111111
        ];
        /* let mut b: Vec<f32> = vec![0.0;25];
        a.append(&mut b);
        crate::matrix::Matrix {
            inner: a,
            row: 10,
            col: 5
        } */
        a.into()
    };

    let (P,L,U): _ = T.plu_decomposition();
    println!(" {:?} \n\n {:?} \n\n {:?} \n\n", &P,&L,&U);
    println!(" {:?} \n\n", L*U);
    panic!()        
}