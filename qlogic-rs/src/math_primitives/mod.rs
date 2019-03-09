mod matrix;
mod vector;

/*
trait QubitConstraints
{
    fn is_orthoganal() -> bool;
    fn completeness_relation() -> bool;
}

struct AbstractQubit<T: Scalar> 
{
    ampl_zero: Unit<T>,
    ampl_one: Unit<T>
}

struct VectorQubit<T: Scalar>
{
    inner: Matrix<UnitComplex<T>, nalgebra::U1, nalgebra::U2, nalgebra::ArrayStorage<UnitComplex<T>, nalgebra::U1, nalgebra::U2>>
}

struct Operator<T: Scalar>
{   
    inner: Matrix<UnitComplex<T>, nalgebra::U2, nalgebra::U2, nalgebra::ArrayStorage<UnitComplex<T>, nalgebra::U2, nalgebra::U2>>
}

impl<T: Scalar> Mul<VectorQubit<T>> for Operator<T>
where
    T: Mul<T, Output=T>
{
    type Output=VectorQubit<T>;
    
    fn mul(self, rhs: VectorQubit<T>) -> Self::Output
    {
        self
    }
}

*/