use crate::problem::Problem;

#[derive(Debug, Clone)]
pub struct Solution<'a> {
    path: Vec<usize>,
    problem: &'a Problem,
}

impl<'a> Solution<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        let size = problem.dimension as usize;
        Self {
            path: (1..size + 1).collect(),
            problem,
        }
    }

    pub fn compute_cost(self) -> u32 {
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
}
