extern crate nalgebra;
extern crate rand;

mod distance_matrix;
mod point;
mod problem;
mod simulated_annealing;
mod two_opt;
mod solution;

use distance_matrix::DistanceMatrix;
use nalgebra::DMatrix;
use problem::Problem;
use solution::Solution;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let default = String::from("./problems/ch130.tsp");
    let filename = args.get(1).unwrap_or(&default);
    let problem = Problem::from_file(filename);
    let seed: u64 = args.get(2).map(|x| x.parse().unwrap()).unwrap_or(0);
    match problem {
        Ok(problem) => {
            let distance_matrix = DistanceMatrix::from_problem(&problem);
            let mut solution = Solution::new(&problem, &distance_matrix, seed);
            println!("Initial Cost: {}", solution.compute_cost());
            solution.two_opt();
            println!("Two Opt: {}", solution.compute_cost());
            solution.shuffle();
            println!("Shuffle: {}", solution.compute_cost());
            solution.two_opt();
            println!("Two Opt: {}", solution.compute_cost());
            solution.simulated_annealing();
            println!("SA: {}", solution.compute_cost())
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
