#[derive(Debug)]
pub struct Point {
    id: u16,
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(id: u16, x: u32, y: u32) -> Self {
        Self { id, x, y }
    }

    pub fn distance_from(&self, point: &Self) -> u32 {
        let diff_x = self.x as i32 - point.x as i32;
        let diff_y = self.y as i32 - point.y as i32;
        let sum_squared = (diff_x.pow(2) + diff_y.pow(2)) as f32;
        return sum_squared.sqrt() as u32;
    }
}
