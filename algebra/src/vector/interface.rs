pub trait Norm<T> {

    fn eucl_norm(self) -> T;

}

pub trait Length<T> {

    fn len(self) -> T;

}

pub trait Dot<T> {

    fn dot(self, rhs: Self) -> T;

}

pub trait Cross {

    fn cross(self, rhs: Self) -> Self;

}

pub trait Direct {

    type Output;

    fn direct_product(self, rhs: Self) -> Self::Output;

}

pub trait Scalar<T> {

    fn scalar(self, rhs: T) -> Self;

}