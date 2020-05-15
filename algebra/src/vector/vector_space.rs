pub trait VectorSpace
{
    type Scalar;

    type Vector;

    fn dimensions(&self) -> usize;
}

pub trait VAdd
{
    type Vector;

    fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector;
}

pub trait VScale 
{
    type Scalar;

    type Vector;

    fn vscale(&self, scalar: &Self::Scalar, vector: &Self::Vector) -> Self::Vector;
}


pub trait VIdentity: VMultiplicativeIdent + VAdditiveIdent
{
    // Supertrait.
}  


pub trait VAdditiveIdent
{
    type Output;

    fn additive_ident(&self) -> Self::Output;    
}


pub trait VMultiplicativeIdent
{
    type Output;

    fn mul_ident(&self) -> Self::Output;
}

pub trait VAdditiveInverse
{
    type Vector;

    type Output;

    fn additive_inv(&self, vector: &Self::Vector) -> Self::Output;
}


#[macro_export]
macro_rules! vadd 
{
    ($vector_space:expr, $lhs:expr, $($rhs:expr),+) => {
        {
            use crate::vector::VAdd;
            $vector_space.vadd($lhs, &crate::vadd!($vector_space, $($rhs),+))
        }
    };
    ($vector_space:expr, $lhs:expr) => { $lhs };
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests 
{
    use crate::vadd;
    use super::{VectorSpace, VAdd, VScale, VAdditiveIdent, VAdditiveInverse};

    struct DummyVectorSpace;

    impl VectorSpace for DummyVectorSpace 
    {
        type Scalar = isize;

        type Vector = Vec<isize>;

        fn dimensions(&self) -> usize 
        {
            3
        }
    }

    #[test]
    fn test_addition() 
    {
        let vector_space = DummyVectorSpace;
        let x = vec![ 3, 0, -1 ];
        let y = vec![ 10, 1, 2 ];

        let exp = vec![ 13, 1, 1 ];
        let test = vector_space.vadd(&x, &y);

        assert_eq!(exp, test);
    }

    #[test]
    fn test_multiplication()
    {
        let vector_space = DummyVectorSpace;
        let x = vec![ 3, 0, -1 ];
        let c = 2;

        let exp = vec![ 6, 0, -2 ];
        let test = vector_space.vscale(&c, &x);
        assert_eq!(exp, test);
    }

    #[test]
    fn test_commutative()
    {
        let vector_space = DummyVectorSpace;
        let x = vec![ 3, 1, 5 ];
        let y = vec![ 6, 2, 7 ];

        let lhs = vector_space.vadd(&x, &y);
        let rhs = vector_space.vadd(&y, &x);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_associative_addition()
    {
        let vector_space = DummyVectorSpace;
        let x = vec![ 3, 1, 5 ];
        let y = vec![ 6, 2, 7 ];
        let z = vec![ 4, 5, 1 ];

        let lhs: Vec<isize> = vadd!(vector_space, &x, &y, &z);
        let rhs: Vec<isize> = vadd!(vector_space, &y, &z, &x);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_additive_ident()
    {
        let vector_space = DummyVectorSpace;
        let exp = vec![ 0, 0, 0 ];

        let test = vector_space.additive_ident();
        assert_eq!(test, exp);
    }

    #[test]
    fn test_additive_inverse()
    {
        let vector_space = DummyVectorSpace;
        let x: Vec<isize> = vec![ 3, 1, 5 ];
        let exp: Vec<isize> = vec![ -3, -1, -5 ];
        
        let test: Vec<isize> = vector_space.additive_inv(&x);
        assert_eq!(test, exp);
    }

    #[test]
    fn test_vadd() {

        let vector_space = DummyVectorSpace;
        
        let x: Vec<isize> = vec![ 3, 1, 5 ];
        let y: Vec<isize> = vec![ 6, 2, 7 ];
        let z: Vec<isize> = vec![ 4, 5, 1 ];
        let test: Vec<isize> = vadd!(vector_space, &x, &y, &z);

        let exp: Vec<isize> = vec![ 13, 8, 13];
        assert_eq!(test, exp);
    }
}
