#[macro_use] pub(super) mod structure;

#[macro_use] pub(super) mod matrix_ops;

impl_into_vec!(&'a crate::matrix::Matrix<T>);
impl_into_vec!(&'a mut crate::matrix::Matrix<T>);

impl_index!(crate::matrix::Matrix<T>);
impl_index!(&'a mut crate::matrix::Matrix<T>);

impl<'a, T> std::ops::Index<usize> for &'a crate::matrix::Matrix<T>
{
    type Output = [T];

    fn index<'b>(&'b self,idx:usize) -> &'b Self::Output {
        let i0: usize = idx * self.col;
        let ter: usize = i0 + self.col;
        let i: std::ops::Range<usize> = i0..ter;
        &self.inner[i]
    }
}

impl_getter!(crate::matrix::Matrix<T>);
impl_getter!(&'a crate::matrix::Matrix<T>);
impl_getter!(&'a mut crate::matrix::Matrix<T>);

impl_row_col_traits!(crate::matrix::Matrix<T>);
impl_row_col_traits!(&'a crate::matrix::Matrix<T>);
impl_row_col_traits!(&'a mut crate::matrix::Matrix<T>);

use num_integer::Roots;

impl_mul!(crate::matrix::Matrix<T>);
impl_mul!(&'a crate::matrix::Matrix<T>);
impl_mul!(&'a mut crate::matrix::Matrix<T>);
impl_mul!(crate::matrix::Matrix<T>, crate::matrix::Matrix<T>);
impl_mul!(crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>);
impl_mul!(&'a crate::matrix::Matrix<T>, crate::matrix::Matrix<T>);
impl_mul!(&'a crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>);
impl_mul!(crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>);
impl_mul!(&'a mut crate::matrix::Matrix<T>, crate::matrix::Matrix<T>);
impl_mul!(&'a mut crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>);

use std::ops::{Add, Sub};

impl_add_or_sub!(crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_add_or_sub!(&'a crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_add_or_sub!(crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_add_or_sub!(&'a crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);

impl_add_or_sub!(crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_add_or_sub!(&'a crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_add_or_sub!(crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_add_or_sub!(&'a crate::matrix::Matrix<T>, &'a crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);

impl_add_or_sub!(&'a mut crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_add_or_sub!(crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);
impl_add_or_sub!(&'a mut crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>, Add, add, CheckedAdd, checked_add);

impl_add_or_sub!(&'a mut crate::matrix::Matrix<T>, crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_add_or_sub!(crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);
impl_add_or_sub!(&'a mut crate::matrix::Matrix<T>, &'a mut crate::matrix::Matrix<T>, Sub, sub, CheckedSub, checked_sub);

impl_identity!(crate::matrix::Matrix<T>);
impl_identity!(&'a crate::matrix::Matrix<T>);
impl_identity!(&'a mut crate::matrix::Matrix<T>);

impl_elem_row_operations!(crate::matrix::Matrix<T>);
impl_elem_row_operations!(&'a crate::matrix::Matrix<T>);
impl_elem_row_operations!(&'a mut crate::matrix::Matrix<T>);