mod point;
mod problem;

use problem::Problem;

fn main() {
    let problem = Problem::from_file("./problems/ch130.tsp");
    println!("{:?}", problem);
}
