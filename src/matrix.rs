use num_traits::identities::{One, Zero};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct Matrix<T> {
    nrows: usize,
    ncols: usize,
    data: HashMap<usize, Vec<(usize, T)>>,
}

impl<T> Matrix<T>
where
    T: Clone + One + Zero,
{
    pub(crate) fn new(nrows: usize, ncols: usize, values: Vec<T>) -> Self {
        assert_eq!(values.len(), nrows * ncols, "dimensions do not match");

        let mut data = HashMap::new();

        for (i, val) in values.into_iter().enumerate() {
            if !val.is_zero() {
                let row_index = i / ncols;
                let col_index = i % ncols;

                let row_vals = data.entry(row_index).or_insert(Vec::new());
                row_vals.push((col_index, val));
            }
        }

        Matrix { nrows, ncols, data }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        let row = self.data.get(&x).unwrap();
        let found = row.iter().find(|(col_index, _)| col_index == &y);
        let (_, value) = found.unwrap();
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use std::collections::HashMap;

    #[test]
    fn create() {
        let m = Matrix::new(2, 2, vec![1, 1, 0, 1]);
        let mut expect = HashMap::new();
        expect.insert(0, vec![(0, 1), (1, 1)]);
        expect.insert(1, vec![(1, 1)]);
        assert_eq!(m.data, expect, "Matrix should be created correctly");
    }

    #[test]
    fn create_rect() {
        let m = Matrix::new(
            2,
            5,
            vec![
                0, 1, 2, 3, 4, // first row
                5, 6, 7, 8, 9, // second row
            ],
        );
        let mut expect = HashMap::new();
        expect.insert(0, vec![(1, 1), (2, 2), (3, 3), (4, 4)]);
        expect.insert(1, vec![(0, 5), (1, 6), (2, 7), (3, 8), (4, 9)]);
        assert_eq!(m.data, expect, "Matrix should be created correctly");
    }

    #[test]
    fn index() {
        let m = Matrix::new(
            2,
            5,
            vec![
                0, 1, 2, 3, 4, // first row
                5, 6, 7, 8, 9, // second row
            ],
        );
        let val = m[(1, 4)];
        let expect = 9;
        assert_eq!(val, expect, "indexing should return correct value");
    }
}
