use crate::problem::Problem;
use crate::DMatrix;

use rand::prelude::*;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Solution<'a> {
    pub path: Vec<usize>,
    pub problem: &'a Problem,
    pub distance_matrix: &'a DMatrix<i32>,
    rng: StdRng,
}

impl<'a> Solution<'a> {
    pub fn new(problem: &'a Problem, distance_matrix: &'a DMatrix<i32>, seed: u64) -> Self {
        let size = problem.dimension as usize;
        let rng = StdRng::seed_from_u64(seed);

        Self {
            path: (1..size + 1).collect(),
            problem,
            distance_matrix,
            rng,
        }
    }

    pub fn compute_cost(&self) -> i32 {
        let mut sum = 0;
        let len = self.path.len();
        for i in 0..len {
            let current_idx = self.path[i];
            let next_idx = self.path[(i + 1) % len];
            let current_point = self.problem.get_point(current_idx);
            let next_point = self.problem.get_point(next_idx);
            sum += current_point.distance_from(next_point);
        }
        return sum;
    }

    pub fn shuffle(&mut self) {
        self.path.shuffle(&mut self.rng);
    }
}
