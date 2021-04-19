mod point;
mod problem;
mod solution;
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
            let solution = Solution::new(&problem);
            println!("{}", solution.compute_cost());
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
