use crate::problem::Problem;
use std::iter::repeat_with;

#[derive(Debug)]
pub struct DistanceMatrix {
    pub matrix: Vec<Vec<u32>>,
}

impl DistanceMatrix {
    pub fn from_problem(problem: &Problem) -> Self {
        let n = problem.dimension;
        let mut matrix: Vec<Vec<u32>> = repeat_with(|| Vec::with_capacity(n)).collect();
        // let mut matrix = Vec::with_capacity(n);
        for i in 0..n {
            for j in 0..n {
                matrix[i][j] = problem.points[j].distance_from(&problem.points[i])
            }
        }
        Self { matrix }
    }
}
