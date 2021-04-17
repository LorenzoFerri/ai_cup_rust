mod point;
mod problem;
mod solution;

use problem::Problem;
use solution::Solution;

fn main() {
    let problem = Problem::from_file("./problems/fl1577.tsp");
    match problem {
        Ok(problem) => {
            let solution = Solution::new(&problem);
            println!("{}", solution.compute_cost());
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
