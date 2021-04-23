use crate::rand::Rng;
use crate::Solution;
use std::mem;

impl<'a> Solution<'a> {
    pub fn simulated_annealing(&mut self) {
        let mut temperature = 1000000000f32;
        let cooling = 0.01;
        let abs_temp = 0.0000001;
        let mut best_energy = self.compute_cost();
        let len = self.path.len();
        let mut best_solution = self.clone();
        while temperature > abs_temp {
            let mut proposed_solution = self.clone();
            let current_energy = self.compute_cost();
            let mut city_index_a = self.rng.gen_range(0..len);
            let mut city_index_b = self.rng.gen_range(0..len);
            if city_index_b < city_index_a {
                mem::swap(&mut city_index_a, &mut city_index_b);
            }
            proposed_solution.swap(city_index_a, city_index_b);
            let result = proposed_solution.two_opt();
            let neighbour_energy = current_energy + result;

            let prob = Self::acceptance_probability(current_energy, neighbour_energy, temperature);
            let rnd: f32 = self.rng.gen();

            if neighbour_energy == self.problem.best_known as i32 {
                best_solution = proposed_solution;
                break;
            }

            if neighbour_energy < best_energy {
                // mem::replace(&mut best_solution, proposed_solution);
                best_solution = proposed_solution;
                best_energy = result;
            } else {
                if prob > rnd {
                    // mem::replace(&mut best_solution, proposed_solution);
                    best_solution = proposed_solution;
                }
            }
            temperature *= 1f32 - cooling;
            println!("{}", temperature);
        }
        self.path = best_solution.path.clone();
    }

    fn acceptance_probability(energy: i32, new_energy: i32, temperature: f32) -> f32 {
        if new_energy < energy {
            return 1f32;
        } else {
            return ((energy - new_energy) as f32 / temperature).exp();
        }
    }
}
