/* mod vector_test
{
    use crate::math::{ Vector, VectorAlgebra, Matrix };

    #[test]
    fn test_scalar_dot_product()
    {
        let test = Vector::from(vec![1,3,-5]).dot(Vector::from(vec![4,-2,-1]));
        assert_eq!(test,3);
    }

    #[test]
    fn test_vector_tensor_product()
    {
        let test: Matrix<isize> = Vector::from(vec![2,4,6,8]).kronecker(Vector::from(vec![1,3,5,7]));
        let exp = Matrix::from(vec![2,6,10,14,4,12,20,28,6,18,30,42,8,24,40,56]);
        assert_eq!(test,exp);
    }
} */

mod matrix_test
{
    use crate::math::{ matrix::Matrix, MatrixAlgebra };
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_dim() {
        let test: usize = Matrix::from(vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0])
            .update(Some(3),Some( 3))
            .unwrap()
            .dim()
            .unwrap();
        assert_eq!(test, 9);
    }

    #[test]
    fn test_into_inner() {
        let test: Matrix<f64> = Matrix::from(vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0])
            .update(Some(3),Some( 3))
            .unwrap();
        let exp = vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
        assert_eq!(test.into_inner(), exp);
    }

    #[test]
    fn test_push() {
        let mut test: Matrix<f64> = Matrix::from(vec![0.0,1.0,2.0,3.0])
            .update(Some(2),Some( 2))
            .unwrap();
        let exp: Matrix<f64> = Matrix::from(vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0])
            .update(Some(3),Some( 3))
            .unwrap();
        test.push(4.0);
        test.push(5.0);
        test.push(6.0);
        test.push(7.0);
        test.push(8.0);
        assert_eq!(test.update(Some(3),Some(3)).unwrap(),exp);
    }

    #[test]
    fn test_row_permutation() {
        let exp = vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];
        let test = Matrix::from(vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0])
            .update(Some(3),Some( 3))
            .unwrap()
            .permute_rows()
            .unwrap()
            .collect::<Vec<f64>>();
        assert_eq!(exp, test);
        
    }

    #[test]
    fn test_row_extraction() {
        let test = Matrix::from(vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0])
            .update(Some(3),Some( 3))
            .unwrap()
            .extract_row(0)
            .unwrap();
        let exp = vec![0.0,1.0,2.0];
        assert_eq!(test,exp);
    }

    #[test]
    fn test_col_extraction() {
        let test = Matrix::from(vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0])
            .update(Some(3),Some( 3))
            .unwrap()
            .extract_col(0)
            .unwrap();
        let exp = vec![0.0,3.0,6.0];
        assert_eq!(test,exp);
    }

    #[test]
    fn test_column_permutation()
    {

        let exp = vec![0.0,3.0,6.0,9.0,1.0,4.0,7.0,10.0,2.0,5.0,8.0,11.0];
        let test = Matrix::from(vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0])
            .update(Some(3),Some( 4))
            .unwrap()
            .permute_cols()
            .unwrap()
            .collect::<Vec<f64>>();
        assert_eq!(exp, test);
    }

    #[test]
    fn test_matrix_get()
    {   
        let test = Matrix::from(
            vec![
                0.0, 1.0, 2.0,
                3.0, 4.0, 5.0,
                6.0, 7.0, 8.0
            ]).update(Some(3),Some( 3))
            .unwrap();
        assert_eq!(test.get(Some(1),Some(1)).unwrap(), 4.0);
        assert_eq!(test.get(Some(1),Some(2)).unwrap(), 5.0);
        assert_eq!(test.get(Some(2),Some(1)).unwrap(), 7.0);
    }

    #[test]
    fn test_scalar_mul()
    {
        let test = Matrix::from(vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0])
            .update(Some(3),Some( 3))
            .unwrap()
            .scalar(3.0)
            .unwrap();
        let exp: Matrix<f64> = Matrix::from(vec![0.0, 3.0, 6.0, 9.0, 12.0, 15.0, 18.0, 21.0, 24.0])
            .update(Some(3),Some( 3))
            .unwrap();
        assert_eq!(test, exp);
    }


    #[test]
    fn test_matrix_cross_product()
    {
        let test = Matrix::from(vec![1.0,2.0,1.0,0.0,1.0,0.0,2.0,3.0,4.0])
            .update(Some(3),Some( 3)).unwrap()
            .cross(
                &Matrix::from(vec![2.0,5.0,6.0,7.0,1.0,8.0])
                    .update(Some(3),Some(2)).unwrap()
            ).unwrap();
        let exp = Matrix::from(vec![15.0, 27.0, 6.0, 7.0, 26.0, 63.0])
            .update(Some(3),Some(2)).unwrap();
        assert_eq!(test, exp);
    }

    #[test]
    fn test_kronecker()
    {
        let test1 = Matrix::from(vec![2.0,4.0,6.0,8.0])
            .update(Some(2),Some(2)).unwrap()
            .kronecker(
                &Matrix::from(vec![1.0,3.0,5.0,7.0,9.0,11.0])
                    .update(Some(3),Some(2)).unwrap()
            ).unwrap();
        
        let test2 = Matrix::from(vec![2.0,4.0,6.0,8.0])
            .update(Some(2),Some(2)).unwrap()
            .kronecker(
                &Matrix::from(vec![1.0,3.0,5.0,7.0,9.0,11.0])
                    .update(Some(2),Some(3)).unwrap()
            ).unwrap();
           
        let exp1 = Matrix::from(vec![
            2.0,    6.0,    4.0,    12.0,
            10.0,   14.0,   20.0,   28.0,
            18.0,   22.0,   36.0,   44.0,
            6.0,    18.0,   8.0,    24.0,
            30.0,   42.0,   40.0,   56.0,
            54.0,   66.0,   72.0,   88.0
        ]).update(Some(6),Some(4)).unwrap();
        
        let exp2 = Matrix::from(vec![   
            2.0,    6.0,    10.0,   4.0,    12.0,   20.0,
            14.0,   18.0,   22.0,   28.0,   36.0,   44.0,
            6.0,    18.0,   30.0,   8.0,    24.0,   40.0,
            42.0,   54.0,   66.0,   56.0,   72.0,   88.0
        ]).update(Some(4),Some(6)).unwrap();
        
        assert_eq!(test1,exp1);
        
        assert_eq!(test2,exp2);
    }

    #[test]
    fn test_set()
    {
        let mut test = Matrix::from(vec![
            2.0,    6.0,    4.0,    12.0,
            10.0,   14.0,   20.0,   28.0,
            18.0,   22.0,   36.0,   44.0,
            6.0,    18.0,   8.0,    24.0,
            30.0,   42.0,   40.0,   56.0,
            54.0,   66.0,   72.0,   88.0
        ]).update(Some(6),Some(4)).unwrap();
        assert_eq!(test.get(Some(2),Some(3)).unwrap(), 44.0);
        test.set(Some(2),Some (3), 1111.0).unwrap();
        assert_eq!(test.get(Some(2),Some(3)).unwrap(), 1111.0);  

        assert_eq!(test.get(Some(4),Some( 3)).unwrap(), 56.0);
        test.set(Some(4),Some (3), 1111.0).unwrap();
        assert_eq!(test.get(Some(4),Some( 3)).unwrap(), 1111.0);  
    }

    #[test]
    fn test_matrix_vector_product()
    {
        let test: Matrix<f64> = Matrix::from(vec![1.0,2.0,1.0,0.0,1.0,0.0,2.0,3.0,4.0])
            .update(Some(3),Some(3)).unwrap()
            .cross( &Matrix::from(vec![2.0,6.0,1.0]).update(Some(3),Some(1)).unwrap() )
            .unwrap();
        let exp = Matrix::from(vec![15.0,6.0,26.0]).update(Some(3),Some(1)).unwrap();
        assert_eq!(test, exp);
    }
    /*
    #[test]
    fn test_complex_conjugate()
    {
        let test: Matrix<Complex<f32>> = vec![
            Complex::<f32>::new(1.0,2.0), Complex::<f32>::new(2.0,3.0), Complex::<f32>::new(3.0,4.0), 
            Complex::<f32>::new(4.0,-5.0), Complex::<f32>::new(5.0,-6.0), Complex::<f32>::new(6.0,-7.0), 
            Complex::<f32>::new(7.0,8.0), Complex::<f32>::new(8.0,-9.0), Complex::<f32>::new(9.0,10.0)
        ].into();
        let exp = Matrix::<Complex<f32>>::from(vec![
            Complex::new(1.0,-2.0), Complex::new(2.0,-3.0), Complex::new(3.0,-4.0), 
            Complex::new(4.0,5.0), Complex::new(5.0,6.0), Complex::new(6.0,7.0), 
            Complex::new(7.0,-8.0), Complex::new(8.0,9.0), Complex::new(9.0,-10.0)
        ]);
        assert_eq!(test.complex_conjugate() ,exp);
    }

    #[test]
    fn test_hermitian_conjugate()
    {
        let test: Matrix<Complex<f32>> = Matrix::<Complex<f32>>::from(vec![
            Complex::new(1.0,3.0), Complex::new(0.0,2.0), 
            Complex::new(1.0,1.0), Complex::new(1.0,-4.0)
        ]).hermitian_conjugate();
        let exp = Matrix::<Complex<f32>>::from(vec![
            Complex::new(1.0,-3.0), Complex::new(1.0,-1.0), 
            Complex::new(0.0,-2.0), Complex::new(1.0,4.0)  
        ]);
        assert_eq!(test,exp);
    }
    */ 
    #[test]
    fn test_identity_matrix()
    {
        let exp = Matrix::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
            .update(Some(3),Some(3))
            .unwrap();
        let test = Matrix::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]).update(Some(3),Some( 3)).unwrap();
        let identity = test.identity().unwrap();
        assert_eq!(test.cross(&identity).unwrap(),exp);
    }

    #[test]
    fn test_diagonal() {
        let test: Matrix<f32> = Matrix::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
            .update(Some(3),Some(3))
            .unwrap();
        let diag = test.diagonal().unwrap();
        let exp = vec![0.0,4.0,8.0];
        assert_eq!(diag,exp);
    }

    #[test]
    fn test_trace() {
        let test = Matrix::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
            .update(Some(3),Some(3))
            .unwrap();
        let trace = test.trace().unwrap();
        let exp = 12.0;
        assert_eq!(trace,exp);
    }

    #[test]
    fn test_addition() {
        let test1: Matrix<f64> = Matrix::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
            .update(Some(3),Some(3))
            .unwrap();
        let test2: Matrix<f64> = Matrix::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
            .update(Some(3),Some(3))
            .unwrap();
        let exp: Matrix<f64> = Matrix::from(vec![0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0])
            .update(Some(3),Some(3))
            .unwrap();
        let test: Matrix<f64> = test1.addition(&test2).unwrap();
        assert_eq!(test,exp);
    }

    #[test]
    fn test_subtraction() {
        let test1: Matrix<f64> = Matrix::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
            .update(Some(3),Some(3))
            .unwrap();
        let test2: Matrix<f64> = Matrix::from(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
            .update(Some(3),Some(3))
            .unwrap();
        let exp: Matrix<f64> = Matrix::from(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])
            .update(Some(3),Some(3))
            .unwrap();
        let test: Matrix<f64> = test1.subtraction(&test2).unwrap();
        assert_eq!(test,exp);
    }

    #[test]
    fn test_decomp()
    {
        let M: Matrix<f64> = Matrix::from(vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0])
            .update(Some(3),Some(3))
            .unwrap();
        let (Q,R): (Matrix<f64>,_) = M.hessenberg().unwrap();
        let A: Matrix<f64> =  Matrix::from(vec![14.0, 21.0, -14.0, 0.0, 175.0, -70.0, 0.0, 0.0, -35.0])
            .update(Some(3),Some(3))
            .unwrap();
        let r_dot_m = R.cross(&M).unwrap();
        for (t,e) in r_dot_m.permute_rows()
            .unwrap()
            .zip(A.permute_rows().unwrap())
        {
            assert_approx_eq!(t,e);
        }
    }

    #[test]
    fn test_determinant()
    {
        let M: Matrix<f64> = Matrix::from(vec![12.0, -51.0, 4.0, 6.0, 167.0, -68.0, -4.0, 24.0, -41.0])
            .update(Some(3),Some(3))
            .unwrap();
        let det = M.determinant().unwrap(); 
        let exp = -85750.0;
        assert_approx_eq!(det,exp);
    }
}

