use crate::utils;

#[derive(Debug)]
pub struct Matrix {
    nrows: usize,
    ncols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(nrows: usize, ncols: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), nrows * ncols, "dimensions do not match");
        Matrix { nrows, ncols, data }
    }

    pub fn solve(&self, b: Self, eps: f64) -> Vec<f64> {
        assert_eq!(self.ncols, b.nrows, "dimensions of b do not match");
        assert_eq!(b.ncols, 1, "b should only have one column (vector)");

        let mut x_prev = vec![1.0; self.ncols];
        let mut x_next = x_prev.clone();

        let mut not_good_enough = true;

        while not_good_enough {
            for i in 0..self.nrows {
                let a_ii = self[(i, i)];
                let b_i = b[(i, 0)];

                let mut sigma = 0.0;

                for j in 0..self.nrows {
                    if j != i {
                        sigma += self[(i, j)] * x_next[j]
                    }
                }

                x_next[i] = (b_i - sigma) / a_ii
            }

            let dist = utils::dist_2(&x_next, &x_prev);
            not_good_enough = dist >= eps;
            x_prev = x_next.clone();
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
    use crate::{matrix::Matrix, utils};

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

    #[test]
    fn solve() {
        let a = Matrix::new(2, 2, vec![16.0, 3.0, 7.0, -11.0]);
        let b = Matrix::new(2, 1, vec![11.0, 13.0]);
        let eps = 0.2;
        let res = a.solve(b, eps);
        let expect = vec![0.8122, -0.6650];
        let d = utils::dist_2(&res, &expect);
        assert!(d <= eps, "Approximation should be 'good enough' ");
    }

    #[test]
    fn solve_2() {
        let a = Matrix::new(3, 3, vec![4.0, -1.0, -1.0, -2.0, 6.0, 1.0, -1.0, 1.0, 7.0]);
        let b = Matrix::new(3, 1, vec![3.0, 9.0, -6.0]);
        let eps = 0.2;
        let res = a.solve(b, eps);
        let expect = vec![1.0, 2.0, -1.0];
        println!("res: {:?}", res);
        let d = utils::dist_2(&res, &expect);
        assert!(d <= eps, "Approximation should be 'good enough' ");
    }
}
