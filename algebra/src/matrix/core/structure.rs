macro_rules! impl_into_vec {
    ($id:ty) => {
        impl<'a, T:Copy> Into<Vec<T>> for $id 
        { 
            fn into(self) -> Vec<T> { 
                self.inner.clone() 
            } 
        }
    }
}

macro_rules! impl_index
{
    ($id:ty) => {
        impl<'a, T> std::ops::Index<usize> for $id
        {
            type Output = [T];

            fn index(&self, idx:usize) -> &Self::Output {
                let i0: usize = idx * self.col;
                let ter: usize = i0 + self.col;
                let i: std::ops::Range<usize> = i0..ter;
                &self.inner[i]
            }
        }

        impl<'a, T> std::ops::IndexMut<usize> for $id
        {
            fn index_mut(&mut self, idx:usize) -> &mut Self::Output {
                let i0: usize = idx * self.col;
                let ter: usize = i0 + self.col;
                let i: std::ops::Range<usize> = i0..ter;
                &mut self.inner[i]
            }
        }

    }
}

macro_rules! impl_getter 
{
    ($id:ty) => 
    {
        impl<'a,T> crate::matrix::interface::Dimension<usize> for $id {
            fn dim(self) -> (usize,usize)
            {
                (self.row, self.col)
            }
        }
    }
}

macro_rules! impl_row_col_traits 
{
    ($id:ty) =>
    {
        impl<'a,T: Copy> crate::matrix::interface::Column<usize> for $id 
        {            
            type Output = Vec<T>;

            fn get_col(self, idx: usize) -> Self::Output
            {
                let mut temp: Vec<T> = Vec::new();
                for i in 0..self.row { temp.push(self[i][idx]) }
                temp
            }

        }

        impl<'a,T: Copy> crate::matrix::interface::Row<usize> for $id 
        {            
            type Output = Vec<T>;

            fn get_row(self, idx: usize) -> Self::Output
            {
                let mut temp: Vec<T> = Vec::new();
                for i in 0..self.col { temp.push(self[idx][i]) }
                temp
            }
        }
    }
}

impl<'a,T: Copy> crate::matrix::interface::MutableColumn<usize> for &'a mut crate::matrix::Matrix<T> {

    type Output = &'a mut [T];

    fn get_mut_col(mut self, idx: usize) -> Self::Output {
        unimplemented!()
    }

}

impl<'a,T: Copy> crate::matrix::interface::MutableRow<usize> for &'a mut crate::matrix::Matrix<T> {

    type Output = &'a mut [T];

    fn get_mut_row(mut self, idx: usize) -> Self::Output {
        unimplemented!()
    }

}