mod vector_test
{
    use crate::math::*;

    #[test]
    fn test_scalar_dot_product()
    {
        let test = Vector::from(vec![1,3,-5]).dot(Vector::from(vec![4,-2,-1]));
        assert_eq!(test,3);
    }

    #[test]
    fn test_vector_tensor_product()
    {
        let test = Vector::from(vec![2,4,6,8]).tensor(Vector::from(vec![1,3,5,7]));
        let exp = Vector::from(vec![2,6,10,14,4,12,20,28,6,18,30,42,8,24,40,56]);
        assert_eq!(test,exp);
    }
}

mod matrix_test
{
    use num::Complex;
    use crate::math::*;

    #[test]
    fn test_column_permutation()
    {
        let exp = vec![0, 3, 6, 1, 4, 7, 2, 5, 8].into_iter();
        let test = Matrix::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]).permute_cols();
        for (exp, test) in exp.into_iter()
            .zip( test )
        {
            assert_eq!(exp, test);
        }
    }

    #[test]
    fn test_matrix_get()
    {   
        let test = Matrix::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(test.get(1,1).unwrap(), 4);
        assert_eq!(test.get(1,2).unwrap(), 5);
        assert_eq!(test.get(2,1).unwrap(), 7);
        match test.get(2,8) {
            Err(_) => { },
            _ => panic!("MatrixError was not returned as expected")
        }
    }

    #[test]
    fn test_scalar_mul()
    {
        let test = Matrix::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]).scalar(3);
        let exp = Matrix::from(vec![0, 3, 6, 9, 12, 15, 18, 21, 24]);
        assert_eq!(test, exp);
    }

    #[test]
    fn test_matrix_dot_product()
    {
        let test = Matrix::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]).cross(Matrix::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]));
        let exp = Matrix::from(vec![15, 18, 21, 42, 54, 66, 69, 90, 111]);
        assert_eq!(test, exp);
    }

    #[test]
    fn test_kronecker()
    {
        let test = Matrix::from(vec![1,2,3,4]).kronecker(Matrix::from(vec![0,5,6,7]));
        let exp = Matrix::from(vec![0,5,0,10,6,7,12,14,0,15,0,20,18,21,24,28]);
        assert_eq!(test,exp);
    }

    #[test]
    fn test_matrix_vector_product()
    {
        let test = Matrix::from(vec![1,2,1,0,1,0,2,3,4]).vector_product(Vector::from(vec![2,6,1]));
        let exp = Vector::from(vec![15,6,26]);
        assert_eq!(test, exp);
    }

    #[test]
    fn test_complex_conjugate()
    {
        let test = ComplexMatrix::from(vec![
            Complex::new(1.0,2.0), Complex::new(2.0,3.0), Complex::new(3.0,4.0), 
            Complex::new(4.0,-5.0), Complex::new(5.0,-6.0), Complex::new(6.0,-7.0), 
            Complex::new(7.0,8.0), Complex::new(8.0,-9.0), Complex::new(9.0,10.0)
        ]).complex_conjugate();
        let exp = Matrix::from(vec![
            Complex::new(1.0,-2.0), Complex::new(2.0,-3.0), Complex::new(3.0,-4.0), 
            Complex::new(4.0,5.0), Complex::new(5.0,6.0), Complex::new(6.0,7.0), 
            Complex::new(7.0,-8.0), Complex::new(8.0,9.0), Complex::new(9.0,-10.0)
        ]);
        assert_eq!(test,exp);
    }

    #[test]
    fn test_hermitian_conjugate()
    {
        let test = Matrix::from(vec![
            Complex::new(1.0,3.0), Complex::new(0.0,2.0), 
            Complex::new(1.0,1.0), Complex::new(1.0,-4.0)
        ]).hermitian_conjugate();
        let exp = Matrix::from(vec![
            Complex::new(1.0,-3.0), Complex::new(1.0,-1.0), 
            Complex::new(0.0,-2.0), Complex::new(1.0,4.0)  
        ]);
        assert_eq!(test,exp);
    }

    #[test]
    fn test_identity_matrix()
    {
        let exp = Matrix::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let test = Matrix::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]).identity();
        assert_eq!(test,exp);
    }
}