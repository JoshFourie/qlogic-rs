use crate::matrix;

use matrix::interface;

use interface::LU;

impl<T> interface::Inverse for matrix::Matrix<T>
{

    type Output = Self;

    fn inverse(self) -> Self::Output
    {
        

        unimplemented!()
    }

}


/* 

#[ignore]#[test] fn test_inversion() 
{
    use crate::matrix::interface::LU;

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

    let (P,L,U): _ = T.lu();
    println!(" {:?} \n\n {:?} \n\n {:?} \n\n", &P,&L,&U);
    println!(" {:?} \n\n", L*U);
    panic!()        
}
*/