use crate::rand::Rng;
use crate::Solution;
use std::mem;

impl<'a> Solution<'a> {
    pub fn simulated_annealing(&mut self) {
        let mut temperature = 1000000000f32;
        let cooling = 0.01;
        let abs_temp = 0.0000001;
        let mut best_energy = 0;
        let len = self.path.len();
        while temperature > abs_temp {
            let current_energy = 0;
            let mut city_index_a = self.rng.gen_range(0..len - 1);
            let mut city_index_b = self.rng.gen_range(0..len - 1);
            if city_index_b < city_index_a {
                mem::swap(&mut city_index_a, &mut city_index_b);
            }
            self.swap(city_index_a, city_index_b);

            temperature *= 1f32 - cooling;
        }
    }
}
