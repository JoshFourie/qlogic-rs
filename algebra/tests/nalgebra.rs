#[test]
fn nalgebra_vector_multiplication() 
{
    let mut x: nalgebra::DVector<isize> = nalgebra::DVector::new_random(1024);
    for element in x.iter_mut() {
        *element = *element % 256;
    }
        
    let scalar: isize = 125;

    let y: _ = &x * scalar;
    assert!(y != x);
}