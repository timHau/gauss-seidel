use crate::utils;

#[derive(Debug)]
pub(crate) struct Matrix {
    nrows: usize,
    ncols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub(crate) fn new(nrows: usize, ncols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), nrows * ncols, "dimensions do not match");
        Matrix { nrows, ncols, data }
    }

    pub(crate) fn solve(&self, b: Self, eps: f64) -> Vec<f64> {
        assert_eq!(self.ncols, b.nrows, "dimensions of b do not match");
        assert_eq!(b.ncols, 1, "b should only have one column (vector)");

        let mut x_prev = vec![0.0; self.ncols];
        let mut x_next = x_prev.clone();

        for i in 0..self.nrows {
            let a_ii: f64 = self[(i, i)].into();
            let b_i: f64 = b[(i, 0)].into();

            let mut sub_1 = 0.0;
            for j in 0..i {
                sub_1 += self[(i, j)] * x_next[j];
            }

            let mut sub_2 = 0.0;
            for j in i..self.ncols {
                sub_2 += self[(i, j)] * x_prev[j];
            }

            x_next[i] = 1.0 / a_ii * (b_i - sub_1 - sub_2)
        }

        x_next
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.data.get(index.0 * self.ncols + index.1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use std::collections::HashMap;

    #[test]
    fn index() {
        let m = Matrix::new(
            2,
            5,
            vec![
                0.0, 1.0, 2.0, 3.0, 4.0, // first row
                5.0, 6.0, 7.0, 8.0, 9.0, // second row
            ],
        );
        let val = m[(1, 4)];
        let expect = 9.0;
        assert_eq!(val, expect, "indexing should return correct value");
    }
}
