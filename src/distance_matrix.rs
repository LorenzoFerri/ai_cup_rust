use crate::nalgebra::DMatrix;
use crate::problem::Problem;

pub trait DistanceMatrix {
    fn from_problem(problem: &Problem) -> Self;
}

impl DistanceMatrix for DMatrix<i32> {
    fn from_problem(problem: &Problem) -> Self {
        let n = problem.dimension;
        let matrix: DMatrix<i32> = DMatrix::from_fn(n, n, |i, j| {
            problem.points[j].distance_from(&problem.points[i])
        });
        matrix
    }
}
