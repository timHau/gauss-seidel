# Gauss Seidel Method

Implements the [Gauss-Seidel method](https://en.wikipedia.org/wiki/Gauss%E2%80%93Seidel_method), which is an iterative method to approximately solve a system of linear equations. 

So Given an `n x n` Matrix `A` and an `n x 1` Matrix (Vector) `b` it approximately computes
```
A * x = b
```

## How to use

You can find an example usage in `bin/main.rs`. Basically you have to give the system of linear equations in matrix form and call the `.solve(b, eps)` method on the `matrix::Matrix` struct, where `b` is the Vector of solutions and `eps` is a sufficiently small residual. 

Example:
``` rust 
    let a = Matrix::new(2, 2, vec![16.0, 3.0, 7.0, -11.0]);
    let b = Matrix::new(2, 1, vec![11.0, 13.0]);
    let eps = 0.2;
    let res = a.solve(b, eps); // res ≈ vec![0.8122, -0.6650]
```

So `eps` gives you the precision of the Approximation.

## Important !!

Note that the matrix you want to solve has to have non-zero elements on the diagnoals and is either [strictly diagonally dominant](https://en.wikipedia.org/wiki/Diagonally_dominant_matrix) or symmetric and positive definite.

Currently only Matrices with `f64` components are supported.
