extern crate nalgebra;
mod distance_matrix;
mod point;
mod problem;
mod solution;

use distance_matrix::DistanceMatrix;
use nalgebra::DMatrix;
use problem::Problem;
use solution::Solution;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = String::from("./problems/ch130.tsp");
    let filename = args.get(1).unwrap_or(&default);
    let problem = Problem::from_file(filename);
    match problem {
        Ok(problem) => {
            let distance_matrix = DMatrix::<u32>::from_problem(&problem);
            let solution = Solution::new(&problem);
            println!("{}", solution.compute_cost());
            println!("{:?}", distance_matrix.diagonal());
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
