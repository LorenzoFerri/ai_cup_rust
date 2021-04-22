use crate::Solution;

impl<'a> Solution<'a> {
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

    pub fn swap(&mut self, i: usize, j: usize) {
        let mut i = i;
        let mut j = j;
        while i < j {
            self.path.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}
