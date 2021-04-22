use crate::problem::Problem;
use crate::DMatrix;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Solution<'a> {
    path: Vec<usize>,
    problem: &'a Problem,
    distance_matrix: &'a DMatrix<i32>,
}

impl<'a> Solution<'a> {
    pub fn new(problem: &'a Problem, distance_matrix: &'a DMatrix<i32>) -> Self {
        let size = problem.dimension as usize;
        Self {
            path: (1..size + 1).collect(),
            problem,
            distance_matrix,
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

    pub fn two_opt(&mut self) {
        let mut improve = true;
        let mut best_gain: i32;
        let mut first = 0;
        let mut second = 0;
        let len = self.path.len();
        while improve {
            improve = false;
            for i in 0..len {
                best_gain = 0;
                for j in i + 2..len {
                    let a = self.path[i] - 1;
                    let b = self.path[i + 1] - 1;
                    let c = self.path[j] - 1;
                    let d = self.path[(j + 1) % len] - 1;
                    if !(self.distance_matrix[(a, b)] > self.distance_matrix[(b, c)])
                        && !(self.distance_matrix[(c, d)] > self.distance_matrix[(c, a)])
                    {
                        continue;
                    };
                    if b == c || d == a {
                        continue;
                    };
                    let local_gain = -self.distance_matrix[(a, b)] - self.distance_matrix[(c, d)]
                        + self.distance_matrix[(a, c)]
                        + self.distance_matrix[(b, d)];
                    if local_gain < best_gain {
                        best_gain = local_gain;
                        first = i + 1;
                        second = j;
                        improve = true;
                    }
                }
                if best_gain < 0 {
                    self.path.swap(first, second);
                }
            }
        }
    }

    pub fn shuffle(&mut self) {
        self.path.shuffle(&mut thread_rng())
    }
}
