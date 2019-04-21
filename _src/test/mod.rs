mod math_test;
// mod operator_test;

pub(crate) use super::*;

#[test]
fn test_iterator()
{
    let A = &Matrix::from(vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0])
        .update(Some(3),Some(3))
        .unwrap();
    let test_vec = vec![0.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0];    
    for (test, exp) in A.into_iter()
        .zip(test_vec.into_iter())
    {
        assert_eq!(test,exp);
    } 
    let B = A;     
}