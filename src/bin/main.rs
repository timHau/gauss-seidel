use gauss_seidel::matrix::Matrix;

fn main() {
    let a = Matrix::new(2, 2, vec![16.0, 3.0, 7.0, -11.0]);
    let b = Matrix::new(2, 1, vec![11.0, 13.0]);
    let eps = 0.2;
    let res = a.solve(b, eps);
    println!("res: {:?}", res);
}
